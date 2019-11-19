extern crate iron;
extern crate juniper;
extern crate juniper_iron;
extern crate logger;
extern crate mount;
extern crate serde;

mod graphql;
use graphql::root::*;
use iron::prelude::*;
use juniper::EmptyMutation;
use juniper_iron::{GraphQLHandler, PlaygroundHandler};
use logger::Logger;
use mount::Mount;
use std::env;

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint =
        GraphQLHandler::new(context_factory, Query, EmptyMutation::<ContextDB>::new());
    let graphiql_endpoint = PlaygroundHandler::new("/graphql");
    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let host = env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
