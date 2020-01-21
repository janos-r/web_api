// Example model

/*
Two ways how to create a gql schema in juniper.

A:
#[derive(juniper::GraphQLObject)]
struct User {...}
The easiest way - derives the whole schema from the struct. No impl necessary.
Note: The mongo _id field must (probably) have the type "ObjectId", which GraphQLObject can't resolve automatically. So as far as I know for now, this method can be used only if you don't need the id field. You can simply ommit it.

B:
custom types
#[juniper::object] or
#[juniper::object(Context = Database)]
impl User {...}
The methods serve as resolvers. &self is the raw data from the db in form of the struct. They can have access to the context again if necessary. But neither is mandatory - viz Komodita.
*/

use crate::model::context::ContextDB;

pub struct User {
    pub id: i32,
    pub name: String,
    pub friend_ids: Vec<i32>,
}

// Assign Database as the context type for User
// custom resolvers for gql
#[juniper::object(Context = ContextDB)]
impl User {
    // 3. Inject the context by specifying an argument
    //    with the context type.
    // Note:
    //   - the type must be a reference
    //   - the name of the argument SHOULD be context
    fn friends(&self, ctx: &ContextDB) -> Vec<&User> {
        // 5. Use the database to lookup users
        self.friend_ids
            .iter()
            .map(|id| ctx.users.get(id).expect("Could not find user with ID"))
            .collect()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn id(&self) -> i32 {
        self.id
    }
}

pub struct Komodita;
#[juniper::object(Context = ContextDB)]
impl Komodita {
    fn elektrika() -> &'static str {
        "plyn string"
    }
    fn plyn() -> &'static str {
        "elektrika string"
    }
}
