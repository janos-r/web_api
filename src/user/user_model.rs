// Example model
use crate::model::context::ContextDB;

pub struct User {
    pub id: i32,
    pub name: String,
    pub friend_ids: Vec<i32>,
}

/// This is a gql schema description: An example type that can use the context again, after it was first provided with its struct User data from list_all in the UserQuery type
// Assign ContextDB as the context type for User
#[juniper::object(Context = ContextDB)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    // Inject the context by specifying an argument with the context type.
    // Note:
    //   - the type must be a reference
    //   - the name of the argument SHOULD be context (though I use ctx here :P)
    /// Here we see that type User can return again a Vec<&User>, so you could cycle these infinitely in the gql request.
    fn friends(&self, ctx: &ContextDB) -> Vec<&User> {
        // Use the database to lookup users (again)
        self.friend_ids
            .iter()
            .map(|id| ctx.users.get(id).expect("Could not find user with ID"))
            .collect()
    }
}

pub struct Metal;
#[juniper::object]
impl Metal {
    fn gold() -> &'static str {
        "shiny golden text"
    }
    fn silver() -> &'static str {
        "shiny silver text"
    }
}
