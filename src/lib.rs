// src/lib.rs
pub mod error;
pub mod models;
pub mod schema;
pub mod db;
pub mod repository; // Single repository instead of services
pub mod config;
pub mod context;
pub mod s3;

use async_graphql::{ EmptySubscription, SchemaBuilder };
// Re-exports
pub use error::{ AppError, AppResult };
pub use models::prelude::*;
pub use repository::{ Repository, DynamoDbEntity };

use crate::schema::resolvers::{ MutationRoot, QueryRoot };

// Type aliases
pub type DbClient = aws_sdk_dynamodb::Client;
pub type S3Client = aws_sdk_s3::Client;

pub type GraphQLSchema = async_graphql::Schema<
    schema::resolvers::query::QueryRoot,
    schema::resolvers::mutation::MutationRoot,
    async_graphql::EmptySubscription
>;

pub fn create_schema() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    use schema::resolvers::{ query::QueryRoot, mutation::MutationRoot };

    async_graphql::Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        async_graphql::EmptySubscription
    )
}
