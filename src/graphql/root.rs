use bson::{bson, doc, oid::ObjectId};
use iron::prelude::*;
use juniper::Context;
use mongodb::{Client, Database};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*
Dvě možnosti jak vytvořit gql schéma v juniper.

A:
#[derive(juniper::GraphQLObject)]
struct User {...}
Vytvoří z objektu schéma. Nejjednodušší asi.

B:
custom types
#[juniper::object] nebo
#[juniper::object(Context = Database)]
impl User {...}
Metody implementace pak jsou jako resolvery pro ten daný typ. Jeho zdroj dat je jeho skutečný struct.
*/

/// The root query object of the schema
pub struct Query;
#[juniper::object(Context = ContextDB)]
impl Query {
    fn all_test(ctx: &ContextDB) -> Vec<Test> {
        ctx.db
            .collection("test")
            .find(None, None)
            .expect("all_test failed to get cursor")
            .map(|result| {
                let doc = result.expect("doc not found");
                bson::from_bson(bson::Bson::Document(doc)).expect("bson: couldn't convert")
            })
            .collect()
    }

    fn all_users(ctx: &ContextDB) -> Vec<&User> {
        ctx.users.iter().map(|(key, user)| user).collect()
    }

    fn komodita() -> Komodita {
        Komodita
    }
}

// ## Context:

pub fn context_factory(_: &mut Request) -> IronResult<ContextDB> {
    let client =
        Client::with_uri_str("mongodb://localhost:27017/").expect("Couldn't connect to the DB");
    let db = client.database("local");

    Ok(ContextDB {
        db,
        users: vec![
            (
                1000,
                User {
                    id: 1000,
                    name: "Robin".to_owned(),
                    friend_ids: vec![1001],
                },
            ),
            (
                1001,
                User {
                    id: 1001,
                    name: "Max".to_owned(),
                    friend_ids: vec![1000],
                },
            ),
        ]
        .into_iter()
        .collect(),
    })
}

pub struct ContextDB {
    db: Database,
    users: HashMap<i32, User>,
}
impl Context for ContextDB {}

struct User {
    id: i32,
    name: String,
    friend_ids: Vec<i32>,
}

// Assign Database as the context type for User
// custom types
#[juniper::object(Context = ContextDB)]
impl User {
    // 3. Inject the context by specifying an argument
    //    with the context type.
    // Note:
    //   - the type must be a reference
    //   - the name of the argument SHOULD be context
    fn friends(&self, context: &ContextDB) -> Vec<&User> {
        // 5. Use the database to lookup users
        self.friend_ids
            .iter()
            .map(|id| context.users.get(id).expect("Could not find user with ID"))
            .collect()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn id(&self) -> i32 {
        self.id
    }
}

struct Komodita;
#[juniper::object(Context = ContextDB)]
impl Komodita {
    fn elektrika() -> &'static str {
        "plyn string"
    }
    fn plyn() -> &'static str {
        "elektrika string"
    }
}

// DB collection test
#[derive(Serialize, Deserialize, Debug)]
struct Test {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
}
#[juniper::object]
impl Test {
    fn id(&self) -> String {
        self.id.to_hex()
    }
    fn name(&self) -> &str {
        &self.name
    }
}

/*
NOTE:
kdyby nebylo zapotřebí používat ID, tak by stačil vracet jen DB model struct s #[derive(juniper::GraphQLObject)]

TODO:
rozdělit soubor lépe to dalších souborů / složek
*/
