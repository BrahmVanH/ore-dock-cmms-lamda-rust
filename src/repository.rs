use std::collections::HashMap;
use aws_sdk_dynamodb::{ Client, types::AttributeValue };
use async_trait::async_trait;
use tracing::{ info, warn };

use crate::AppError;

#[async_trait]
pub trait DynamoDbEntity: Clone + Send + Sync {
    fn table_name() -> &'static str;
    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self>;
    fn to_item(&self) -> HashMap<String, AttributeValue>;
    fn primary_key(&self) -> String;
}

pub struct Repository {
    client: Client,
}

impl Repository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get<T: DynamoDbEntity>(&self, id: String) -> Result<Option<T>, AppError> {
        let mut key = HashMap::new();
        key.insert("id".to_string(), AttributeValue::S(id));

        let response = self.client
            .get_item()
            .table_name(T::table_name())
            .set_key(Some(key))
            .send().await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get item: {}", e)))?;

        Ok(response.item.and_then(|item| T::from_item(&item)))
    }

    pub async fn create<T: DynamoDbEntity>(&self, entity: T) -> Result<T, AppError> {
        let item = entity.to_item();
        info!("new location_type item in repository: {:?}", &item);

        // self.client
        //     .put_item()
        //     .table_name(T::table_name())
        //     .set_item(Some(item))
        //     .condition_expression("attribute_not_exists(id)")
        //     .send()
        //     .await
        //     .map_err(|e| {
        //         if e.to_string().contains("ConditionalCheckFailed") {
        //             AppError::ValidationError("Entity with this ID already exists".to_string())
        //         } else {
        //             AppError::DatabaseError(format!("Failed to create entity: {}", e))
        //         }
        //     })?;

        let temp = self.client
            .put_item()
            .table_name(T::table_name())
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(id)")
            .send().await;
        match &temp {
            Ok(v) => {
                info!("new item temp in repository: {:#?}", &v);
            }
            Err(e) => {
                warn!("new item temp has sdk error: {:#?}", e);
            }
        }

        let temp2 = &temp.map_err(|e| {
            if e.to_string().contains("ConditionalCheckFailed") {
                AppError::ValidationError("Entity with this ID already exists".to_string())
            } else {
                AppError::DatabaseError(format!("Failed to create entity: {}", e))
            }
        })?;

        info!("new iten putitemoutput in repository: {:#?}", temp2);

        Ok(entity)
    }

    pub async fn update<T: DynamoDbEntity>(&self, entity: T) -> Result<T, AppError> {
        let item = entity.to_item();

        self.client
            .put_item()
            .table_name(T::table_name())
            .set_item(Some(item))
            .condition_expression("attribute_exists(id)")
            .send().await
            .map_err(|e| AppError::DatabaseError(format!("Failed to update entity: {}", e)))?;

        Ok(entity)
    }

    pub async fn delete<T: DynamoDbEntity>(&self, id: String) -> Result<bool, AppError> {
        self.client
            .delete_item()
            .table_name(T::table_name())
            .key("id", AttributeValue::S(id))
            .condition_expression("attribute_exists(id)")
            .send().await
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete entity: {}", e)))?;

        Ok(true)
    }

    pub async fn list<T: DynamoDbEntity>(&self, limit: Option<i32>) -> Result<Vec<T>, AppError> {
        let mut scan = self.client.scan().table_name(T::table_name());

        if let Some(limit) = limit {
            scan = scan.limit(limit);
        }

        let response = scan
            .send().await
            .map_err(|e| AppError::DatabaseError(format!("Failed to scan table: {}", e)))?;

        let entities = response.items
            .unwrap_or_default()
            .iter()
            .filter_map(|item| T::from_item(item))
            .collect();

        Ok(entities)
    }
}
