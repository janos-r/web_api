use super::context::ContextDB;
use crate::test::service::TestService;
use crate::user::model::{Komodita, User};

/// The root query object of the schema
pub struct Query;
#[juniper::object(Context = ContextDB)]
impl Query {
    fn version() -> &str {
        "1.0.0"
    }

    fn test() -> TestService {
        TestService
    }

    fn all_users(ctx: &ContextDB) -> Vec<&User> {
        ctx.users.iter().map(|(key, user)| user).collect()
    }

    fn komodita() -> Komodita {
        Komodita
    }
}
