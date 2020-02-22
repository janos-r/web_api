# WEB API

An attempt to create a web api with my favorite technologies:

-   rust
-   graphql
-   mongodb

## Main crates:

-   server: [Iron](https://crates.io/crates/iron)
-   graphql: [Juniper](https://crates.io/crates/juniper)
-   mongodb: [MongoDB](https://crates.io/crates/mongodb)

This project is just an api skeleton.
It represents your collections as folders and shows examples how a mongo modedel is build with struct and the following graphql schema as its methods.

### Database:

The db connection is in the context. It connects by default to `localhost:27017` and the database `local`.

### Graphql:

You can access the api in the playground on <localhost:8080>. Even if you don't set up any collections, the "user" branch of the api uses a hard coded hashMap in the context and can be viewed immediately in the playground after running this code. Mongo has to be running though, or the connection thread panics.

### .env variables:

So far, the project is set up so that the `.env` file is absolutely optional. All the necessary path variables have a hard coded default, that can be overwritten by the environment variable.
The variables are loaded from the `.env` file. So far you can set there:

-   `LISTEN` - default: "0.0.0.0:8080"
-   `GRAPHQL_PATH` - default: "/graphql"
-   `PLAYGROUND_PATH` - default: "/"
-   `RUST_LOG` - default: none, info bellow

These paths have to be unique! Putting graphql and PlayGround on the same path won't work.

### To run:

Just `cargo run`. If you don't have cargo, you can get it with rustup.rs

To run with the [logs](https://crates.io/crates/env_logger) add `RUST_LOG=info` or `RUST_LOG=debug` to the `.env` file, or one-time overwrite the `.env` while calling the program: `RUST_LOG=info cargo run` in bash for example.

## Basic documentation to the code

There are two ways how to create a graphql schema in juniper.

A:

```
#[derive(juniper::GraphQLObject)]
struct User {...}
```

This is the easiest way - derives the whole schema from the struct. No impl necessary.

Note: The mongo \_id field must (probably) have the type "ObjectId", which GraphQLObject can't resolve automatically. So as far as I know for now, this method can be used only if you don't need the id field. You can simply omit it in the struct, so after mongodb provides the data including the \_id, it will not be part of the deserialize and smply skipped (viz "SimpleCat").

B:
custom types

```
#[juniper::object] or
#[juniper::object(Context = Database)]
impl User {...}
```

The methods serve as resolvers. `&self` in the method's argument is the raw data from the db in form of the struct. They can have access to the context again if necessary. Examples are shown in the user and cats modules. But schema types don't have to use neither the context or `&self` - viz Metal.

In my view, the impl serves 2 purposes in juniper. First, as in the impl User, it transforms the User data into a custom type, that will be displayed by juniper in the schema. And second, as in what I called a *Query / *Mutation, you can create a type for the root object and create the schema hierarchy that way.

Note: In fact, I'm still a little surprised that you can write in such a juniper::object

```
fn cats() -> CatQuery {
    CatQuery
}
```

Because this would normally not be legal rust code. This fn should return an object of type CatQuery, but it is written as if it returns just the type plain. The juniper::object micro makes this probably work somehow and thanks to this, we can create the graphql schema very nicely.
