use super::user_model::{Metal, User};
use crate::model::context::ContextDB;

pub struct UserService;
#[juniper::object(Context = ContextDB)]
impl UserService {
    fn list_all(ctx: &ContextDB) -> Vec<&User> {
        ctx.users.iter().map(|(key, user)| user).collect()
    }

    fn Metal() -> Metal {
        Metal
    }
}
