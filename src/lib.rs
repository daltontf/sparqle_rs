use std::fmt::{Display, Formatter};

use itertools::Itertools;

#[derive(Clone, Copy)]
pub struct Property(pub &'static str, pub u32);
pub struct PropertyPath<'a>(pub Vec<&'a Property>);
#[derive(Clone, Copy)]
pub struct Entity(pub &'static str, pub u32);
pub struct Var(pub String);
pub struct Tag(pub &'static str, pub String);
pub struct Constant(pub &'static str);

impl Var {
    pub fn label(&self) -> Var {
        Var(format!("{}Label", self.0))
    }

    pub fn name(val: &str) -> Self {
        Var(val.to_string())
    }
}

impl Property {
    pub fn to_stmt(&self) -> Property {
        Property("p", self.1)
    }

    pub fn stmt(&self) -> Property {
        Property("ps", self.1)
    }

    pub fn qualifier(&self) -> Property {
        Property("pq", self.1)
    }
}

pub trait SparqlTerm {
    fn to_query(&self) -> String;
}

impl SparqlTerm for Var {
    fn to_query(&self) -> String {
        format!("?{}", self.0)
    }
}

impl SparqlTerm for Entity {
    fn to_query(&self) -> String {
        format!("{}:Q{}", self.0, self.1)
    }
}

impl SparqlTerm for Property {
    fn to_query(&self) -> String {
        format!("{}:P{}", self.0, self.1)
    }
}

impl SparqlTerm for Tag {
    fn to_query(&self) -> String {
        format!("{}:{}", self.0, self.1)
    }
}

impl SparqlTerm for Constant {
    fn to_query(&self) -> String {
        format!("\"{}\"", self.0)
    }
}

impl SparqlTerm for PropertyPath<'_> {
    fn to_query(&self) -> String {
        let mut val = self.0.iter().map(|v| v.to_query()).join("/");
        val.push_str("*");
        val
    }
}

macro_rules! impl_display_via_sparql_term {
    ($($t:ty),* $(,)?) => {
        $(
            impl Display for $t {
                fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
                    formatter.write_str(&self.to_query())
                }
            }
        )*
    };
}

impl_display_via_sparql_term!(Var, Entity, Property, Tag, PropertyPath<'_>, Constant);

pub mod namespace;
#[cfg(feature = "fluent")]
pub mod fluent;
