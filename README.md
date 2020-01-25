# WEB API

An attempt to create a web api with my favorite technologies:

-   rust
-   graphql
-   mongodb

## Crates used:

-   server: Iron
-   graphql: Juniper
-   mongodb: MongoDB

This project is just an api skeleton.
It represents your collections as folders and shows examples how a mongo modedel is build with struct and the following graphql schema as its methods.

### Database:

The db connection is in the context. It connects by default to **localhost:27017**\
TODO: make an .env variable

### Graphql:

You can access the api in the playground on <localhost:8080>. Even if you don't set up any collections, the "user" branch of the api uses a hardcoded hashMap in the context and can be viewed immediately in the playground after running this code. Mongo has to be running though, or the connection thread panics.

### To run:

Just `cargo run`. If you don't have cargo, you can get it with rustup.rs
