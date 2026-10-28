use std::fmt::{Display, Formatter};

use itertools::Itertools;

#[derive(Clone, Copy)]
pub enum PathRepeat {
    ZeroOrMore,
    OneOrMore,
    ZeroOrOne,
}

impl PathRepeat {
    pub fn character(&self) -> char {
        match self {
            PathRepeat::ZeroOrMore => '*',
            PathRepeat::OneOrMore => '+',
            PathRepeat::ZeroOrOne => '?',
        }
    }
}

pub trait PropertyOfType {}

#[derive(Clone, Copy)]
pub struct Plain(pub &'static str, pub u32);
#[derive(Clone, Copy)]
pub struct Repeated(pub &'static str, pub u32, pub PathRepeat);

impl PropertyOfType for Plain {}
impl PropertyOfType for Repeated {}

#[derive(Clone, Copy)]
pub struct Property<T: PropertyOfType>(pub T);

pub trait PathElement: SparqlTerm {}

impl PathElement for Property<Plain> {}
impl PathElement for Property<Repeated> {}

pub struct PropertySequence(pub Vec<Box<dyn PathElement>>);

#[derive(Clone, Copy)]
pub struct Entity(pub &'static str, pub u32);
pub struct Var(pub String);
pub struct Tag(pub &'static str, pub String);
pub struct Literal(pub &'static str);
pub struct LangLiteral(pub &'static str, pub &'static str);
pub struct TypedLiteral(pub &'static str, pub &'static str);
pub struct Blank;

pub struct SubjectPredicate<'a>(&'a dyn SparqlTerm, &'a dyn SparqlTerm);
pub struct WithPredicateObjects<'a>(
    &'a dyn SparqlTerm,
    Vec<(&'a dyn SparqlTerm, &'a dyn SparqlTerm)>,
);
// pub struct Triple<'a>(&'a dyn SparqlTerm, &'a dyn SparqlTerm, &'a dyn SparqlTerm);

impl Var {
    pub fn name(val: &str) -> Self {
        Var(val.to_string())
    }

    pub fn label(&self) -> Var {
        Var(format!("{}Label", self.0))
    }

    pub fn property<'a, T: PropertyOfType>(&'a self, prop: &'a Property<T>) -> SubjectPredicate<'a>
    where
        Property<T>: SparqlTerm,
    {
        SubjectPredicate(self, prop)
    }

    pub fn properties<'a>(
        &'a self,
        predicate_objects: &[(&'a dyn SparqlTerm, &'a dyn SparqlTerm)],
    ) -> WithPredicateObjects<'a> {
        WithPredicateObjects(self, Vec::from(predicate_objects))
    }

    pub fn property_path<'a>(&'a self, prop: &'a PropertySequence) -> SubjectPredicate<'a> {
        SubjectPredicate(self, prop)
    }
}

impl SubjectPredicate<'_> {
    pub fn is<'a>(
        &'a self,
        entity: &'a Entity,
    ) -> (&'a dyn SparqlTerm, &'a dyn SparqlTerm, &'a dyn SparqlTerm) {
        (self.0, self.1, entity)
    }

    pub fn set<'a>(
        &'a self,
        entity: &'a Var,
    ) -> (&'a dyn SparqlTerm, &'a dyn SparqlTerm, &'a dyn SparqlTerm) {
        (self.0, self.1, entity)
    }
}

impl Property<Plain> {
    pub fn to_stmt(&self) -> Property<Plain> {
        Property(Plain("p", self.0.1))
    }

    pub fn stmt(&self) -> Property<Plain> {
        Property(Plain("ps", self.0.1))
    }

    pub fn qualifier(&self) -> Property<Plain> {
        Property(Plain("pq", self.0.1))
    }

    pub fn repeated(&self, repeat: PathRepeat) -> Property<Repeated> {
        Property(Repeated(self.0.0, self.0.1, repeat))
    }

    pub fn append<P: PathElement + Clone + 'static>(&self, path_element: &P) -> PropertySequence {
        PropertySequence(vec![Box::new(*self), Box::new(path_element.clone())])
    }

    pub fn is<'a>(&'a self, entity: &'a Entity) -> (&'a dyn SparqlTerm, &'a dyn SparqlTerm) {
        (self, entity)
    }

    pub fn set<'a>(&'a self, entity: &'a Var) -> (&'a dyn SparqlTerm, &'a dyn SparqlTerm) {
        (self, entity)
    }
}

impl Property<Repeated> {
    pub fn to_stmt(&self) -> Property<Repeated> {
        Property(Repeated("p", self.0.1, self.0.2))
    }

    pub fn stmt(&self) -> Property<Repeated> {
        Property(Repeated("ps", self.0.1, self.0.2))
    }

    pub fn qualifier(&self) -> Property<Repeated> {
        Property(Repeated("pq", self.0.1, self.0.2))
    }

    pub fn repeated(&self, repeat: PathRepeat) -> Property<Repeated> {
        Property(Repeated(self.0.0, self.0.1, repeat))
    }

    pub fn append<P: PathElement + Clone + 'static>(&self, path_element: &P) -> PropertySequence {
        PropertySequence(vec![Box::new(*self), Box::new(path_element.clone())])
    }

    pub fn is<'a>(&'a self, entity: &'a Entity) -> (&'a dyn SparqlTerm, &'a dyn SparqlTerm) {
        (self, entity)
    }

    pub fn set<'a>(&'a self, entity: &'a Var) -> (&'a dyn SparqlTerm, &'a dyn SparqlTerm) {
        (self, entity)
    }
}

impl PropertySequence {
    pub fn is<'a>(&'a self, entity: &'a Entity) -> (&'a dyn SparqlTerm, &'a dyn SparqlTerm) {
        (self, entity)
    }

    pub fn set<'a>(&'a self, entity: &'a Var) -> (&'a dyn SparqlTerm, &'a dyn SparqlTerm) {
        (self, entity)
    }
}

impl Literal {
    pub fn typed(&self, tipe: &'static str) -> TypedLiteral {
        TypedLiteral(self.0, tipe)
    }

    pub fn lang(&self, lang: &'static str) -> LangLiteral {
        LangLiteral(self.0, lang)
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

impl SparqlTerm for Property<Plain> {
    fn to_query(&self) -> String {
        format!("{}:P{}", self.0.0, self.0.1)
    }
}

impl SparqlTerm for Property<Repeated> {
    fn to_query(&self) -> String {
        format!("{}:P{}{}", self.0.0, self.0.1, self.0.2.character())
    }
}

impl SparqlTerm for Tag {
    fn to_query(&self) -> String {
        format!("{}:{}", self.0, self.1)
    }
}

impl SparqlTerm for Literal {
    fn to_query(&self) -> String {
        format!("\"{}\"", self.0)
    }
}

impl SparqlTerm for LangLiteral {
    fn to_query(&self) -> String {
        format!("\"{}\"@{}", self.0, self.1)
    }
}

impl SparqlTerm for TypedLiteral {
    fn to_query(&self) -> String {
        format!("\"{}\"^^{}", self.0, self.1)
    }
}

impl SparqlTerm for Blank {
    fn to_query(&self) -> String {
        "[]".to_string()
    }
}

impl SparqlTerm for PropertySequence {
    fn to_query(&self) -> String {
        self.0.iter().map(|v| v.to_query()).join("/")
    }
}

impl SparqlTerm for SubjectPredicate<'_> {
    fn to_query(&self) -> String {
        format!("{} {}", self.0.to_query(), self.1.to_query())
    }
}

impl SparqlTerm for WithPredicateObjects<'_> {
    fn to_query(&self) -> String {
        let mut start = self.0.to_query();
        start.push_str(" ");
        start.push_str(
            &self
                .1
                .iter()
                .map(|v| format!("{} {}", v.0.to_query(), v.1.to_query()))
                .join(" ; "),
        );
        start.push_str(" .");
        start
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

impl_display_via_sparql_term!(
    Var,
    Entity,
    Property<Plain>,
    Property<Repeated>,
    Tag,
    PropertySequence,
    Literal,
    Blank
);

#[cfg(feature = "fluent")]
pub mod fluent;
pub mod namespace;
