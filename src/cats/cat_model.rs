// DB collection test

use bson::oid::ObjectId;
// use bson::{bson, doc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cat {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
}
#[juniper::object]
impl Cat {
    fn id(&self) -> String {
        self.id.to_hex()
    }
    fn name(&self) -> &str {
        &self.name
    }
}
