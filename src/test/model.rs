// DB collection test

use bson::oid::ObjectId;
// use bson::{bson, doc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
}
#[juniper::object]
impl Test {
    fn id(&self) -> String {
        self.id.to_hex()
    }
    fn name(&self) -> &str {
        &self.name
    }
}
