use super::context::ContextDB;
use crate::cats::cat_service::CatService;
use crate::user::user_service::UserService;

/// The root query object of the schema
pub struct Query;
#[juniper::object(Context = ContextDB)]
impl Query {
    fn version() -> &str {
        "1.0.0"
    }

    fn cats() -> CatService {
        CatService
    }

    fn users() -> UserService {
        UserService
    }
}
