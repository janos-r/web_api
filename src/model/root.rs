use super::context::ContextDB;
use crate::cats::{cat_mutation::CatMutation, cat_query::CatQuery};
use crate::user::user_query::UserQuery;

/// The root query object of the schema
pub struct Query;
#[juniper::object(Context = ContextDB)]
impl Query {
    fn version() -> &str {
        "1.0.0"
    }

    fn cats() -> CatQuery {
        CatQuery
    }

    fn users() -> UserQuery {
        UserQuery
    }
}

pub struct Mutation;
#[juniper::object(Context = ContextDB)]
impl Mutation {
    fn cats() -> CatMutation {
        CatMutation
    }
}
