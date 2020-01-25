// #[macro_use(bson, doc)]
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
