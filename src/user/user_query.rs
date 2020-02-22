use super::user_model::{Metal, User};
use crate::model::context::ContextDB;

pub struct UserQuery;
#[juniper::object(Context = ContextDB)]
impl UserQuery {
    fn list_all(ctx: &ContextDB) -> Vec<&User> {
        log::info!("UserQuery.list_all");
        ctx.users.iter().map(|(key, user)| user).collect()
    }

    fn Metal() -> Metal {
        Metal
    }
}
