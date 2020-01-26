use super::cat_model::CATS_COLLECTION;
use crate::model::context::ContextDB;
use bson::{bson, doc, to_bson};
use serde::{Deserialize, Serialize};

pub struct CatMutation;

#[juniper::object(Context = ContextDB)]
impl CatMutation {
    fn create(ctx: &ContextDB, input: CatInput) -> String {
        let doc = to_bson(&input)
            .expect("couldn't convert to bson")
            .as_document()
            .expect("bson: couldn't convert to document")
            .clone();
        ctx.db
            .collection(CATS_COLLECTION)
            .insert_one(doc, None)
            .expect("couldn't create doc")
            .inserted_id
            .as_object_id()
            .expect("couldn't turn id to objectId")
            .to_hex()
    }

    fn create_simple(ctx: &ContextDB, name: String) -> String {
        ctx.db
            .collection(CATS_COLLECTION)
            .insert_one(doc! {"name": name.clone()}, None)
            .expect("couldn't create doc");
        name
    }
}

#[derive(juniper::GraphQLInputObject, Serialize, Deserialize, Debug)]
#[graphql(description = "The input object for creating a Cat")]
struct CatInput {
    name: String,
}
