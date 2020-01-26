use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const CATS_COLLECTION: &str = "cats";

#[derive(Serialize, Deserialize, Debug)]
pub struct Cat {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
}

#[juniper::object]
#[graphql(description = "An example type that can also show the _id as 'id'")]
impl Cat {
    fn id(&self) -> String {
        self.id.to_hex()
    }
    fn name(&self) -> &str {
        &self.name
    }
}
