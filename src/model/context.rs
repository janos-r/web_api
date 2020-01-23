// ## Context:

use iron::prelude::{IronResult, Request};
use juniper::Context;
use mongodb::{Client, Database};
use std::collections::HashMap;

use crate::user::user_model::User;

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
    pub db: Database,
    pub users: HashMap<i32, User>,
}
impl Context for ContextDB {}
