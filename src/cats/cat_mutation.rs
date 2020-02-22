use super::cat_model::CATS_COLLECTION;
use crate::model::context::ContextDB;
use bson::{bson, doc, to_bson};
use juniper::FieldResult;
use serde::{Deserialize, Serialize};

pub struct CatMutation;

#[juniper::object(Context = ContextDB)]
impl CatMutation {
    fn create(ctx: &ContextDB, input: CatInput) -> FieldResult<String> {
        log::info!("CatMutation.create(input: {:?})", input);
        let doc = to_bson(&input)?
            .as_document()
            .ok_or("bson: couldn't convert to document")?
            .clone();
        Ok(ctx
            .db
            .collection(CATS_COLLECTION)
            .insert_one(doc, None)?
            .inserted_id
            .as_object_id()
            .ok_or("created successfully, but can't display ID")?
            .to_hex())
    }

    fn create_simple(ctx: &ContextDB, name: String) -> FieldResult<String> {
        log::info!("CatMutation.create_simple(name: {})", name);
        ctx.db
            .collection(CATS_COLLECTION)
            .insert_one(doc! {"name": name.clone()}, None)?;
        Ok(name)
    }
}

/// The input object for creating a Cat
#[derive(juniper::GraphQLInputObject, Serialize, Deserialize, Debug)]
struct CatInput {
    name: String,
}
