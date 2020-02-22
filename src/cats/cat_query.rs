use super::cat_model::{Cat, CATS_COLLECTION};
use crate::model::context::ContextDB;
use juniper::FieldResult;

pub struct CatQuery;

#[juniper::object(Context = ContextDB)]
impl CatQuery {
    /// List all Cats. Here you could return either Vec<Cat> or Vec<SipleCat> from the model.
    fn list_all(ctx: &ContextDB) -> FieldResult<Vec<Cat>> {
        log::info!("CatQuery.list_all");
        Ok(ctx
            .db
            .collection(CATS_COLLECTION)
            .find(None, None)?
            .flat_map(|result| result.ok())
            .flat_map(|doc| bson::from_bson(bson::Bson::Document(doc)).ok())
            .collect())
    }
}
