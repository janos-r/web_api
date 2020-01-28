/*
TODO:
.env variable
try new fs logic
https://doc.rust-lang.org/reference/items/modules.html
*/

/*!
There are two ways how to create a graphql schema in juniper.

A:
```
#[derive(juniper::GraphQLObject)]
struct User {...}
```
This is the easiest way - derives the whole schema from the struct. No impl necessary.

Note: The mongo _id field must (probably) have the type "ObjectId", which GraphQLObject can't resolve automatically. So as far as I know for now, this method can be used only if you don't need the id field. You can simply omit it in the struct, so after mongodb provides the data including the _id, it will not be part of the deserialize and smply skipped (viz "SimpleCat").

B:
custom types
```
#[juniper::object] or
#[juniper::object(Context = Database)]
impl User {...}
```
The methods serve as resolvers. `&self` in the method's argument is the raw data from the db in form of the struct. They can have access to the context again if necessary. Examples are shown in the user and cats modules. But schema types don't have to use neither the context or `&self` - viz Metal.

In my view, the impl serves 2 purposes in juniper. First, as in the impl User, it transforms the User data into a custom type, that will be displayed by juniper in the schema. And second, as in what I called a *_service, you can create a type for the root object and create the schema hierarchy that way.

Note: In fact, I'm still a little surprised that you can write in such a juniper::object
```
fn cats() -> CatQuery {
    CatQuery
}
```
Because this would normally not be legal rust code. This fn should return an object of type CatQuery, but it is written as if it returns just the type plain. The juniper::object micro makes this probably work somehow and thanks to this, we can create the graphql schema very nicely.
*/

extern crate bson;
extern crate iron;
extern crate juniper;
extern crate juniper_iron;
extern crate logger;
extern crate mongodb;
extern crate mount;
extern crate serde;

use iron::prelude::*;
use juniper_iron::{GraphQLHandler, PlaygroundHandler};
use logger::Logger;
use mount::Mount;
use std::env;

mod cats;
mod model;
mod user;
use model::{
    context::context_factory,
    root::{Mutation, Query},
};

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint = GraphQLHandler::new(context_factory, Query, Mutation);
    let playground_endpoint = PlaygroundHandler::new("/graphql");
    mount.mount("/", playground_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let host = env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
