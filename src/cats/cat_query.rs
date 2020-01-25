use super::cat_model::{Cat, CATS_COLLECTION};
use crate::model::context::ContextDB;

pub struct CatQuery;
#[juniper::object(Context = ContextDB)]
impl CatQuery {
    fn list_all(ctx: &ContextDB) -> Vec<Cat> {
        ctx.db
            .collection(CATS_COLLECTION)
            .find(None, None)
            .expect("all_test failed to get cursor")
            .map(|result| {
                let doc = result.expect("doc not found");
                bson::from_bson(bson::Bson::Document(doc)).expect("bson: couldn't convert")
            })
            .collect()
    }
}
