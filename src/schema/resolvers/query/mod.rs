use async_graphql::MergedObject;

mod asset_type;
mod asset;
mod location;
mod location_type;

#[derive(Debug, Default, MergedObject)]
pub struct QueryRoot(asset_type::Query, asset::Query, location::Query, location_type::Query);
