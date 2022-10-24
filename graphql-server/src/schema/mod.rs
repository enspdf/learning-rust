use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

mod health;

#[derive(MergedObject, Default)]
pub struct Query(health::HealthQuery);

pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}
