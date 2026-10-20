## Sparqlers 

This is an exploration of a Rust libary for building SPARQL queries (mainly for WikiData)

The is a string interpolation way where terms of the triples implement the `Display` trait to render correctly:

[Interpolation Example](./src/bin/interp.rs)

`cargo run --bin iterp

An the even more experimental fluent way enabled be `fluent` feature.

[Fluent Example](./src/bin/fluent.rs)

`cargo run --features fluent --bin fluent