use super::cat_model::Cat;
use crate::model::context::ContextDB;

const COLLECTION: &str = "cats";

pub struct CatService;
#[juniper::object(Context = ContextDB)]
impl CatService {
    fn list_all(ctx: &ContextDB) -> Vec<Cat> {
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

/*
TODO:
cat mutation
try new fs logic
https://doc.rust-lang.org/reference/items/modules.html
 */
