## Sparqlers 

This is an exploration of a Rust libary for building SPARQL queries (mainly for WikiData)

The is a string interpolation way where terms of the triples implement the `Display` trait to render correctly:

[Interpolation Example](./src/bin/interp.rs)

`cargo run --bin iterp

Rust interpolated strings might be too limited to maximize readability. 
Would be nice to be able to invoke methods inside the "{}" instead of need separate bindings:

'SELECT {stadium} {stadium.label()} {coordinates} {coordinates.label()} {occupant} {occupant.label()} WHERE ...

An even more experimental "fluent" way can be enabled by the `fluent` feature. 

[Fluent Example](./src/bin/fluent.rs)

`cargo run --features fluent --bin fluent

This uses the Typestate pattern to help prevent illegal SQARQL syntax at compile time. 

Can enough of the SPARQL specification be implemented this way to be useful?

---

... Or this could/should this be done with macros like Leptos `view!`?