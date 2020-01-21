use super::model::Test;
use crate::model::context::ContextDB;

const COLLECTION: &str = "test";

pub struct TestService;
#[juniper::object(Context = ContextDB)]
impl TestService {
    fn list_all(ctx: &ContextDB) -> Vec<Test> {
        ctx.db
            .collection(COLLECTION)
            .find(None, None)
            .expect("all_test failed to get cursor")
            .map(|result| {
                let doc = result.expect("doc not found");
                bson::from_bson(bson::Bson::Document(doc)).expect("bson: couldn't convert")
            })
            .collect()
    }
}
