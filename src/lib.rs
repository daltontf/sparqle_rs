use std::fmt::{Display, Formatter};

use itertools::Itertools;

#[derive(Clone, Copy)]
pub enum PathSuffix {
    ZeroOrMore,
    OneOrMore,
    ZeroOrOne,
}

impl PathSuffix {
    pub fn character(&self) -> char {
        match self {
            PathSuffix::ZeroOrMore => '*',
            PathSuffix::OneOrMore => '+',
            PathSuffix::ZeroOrOne => '?',
        }
    }
}

#[derive(Clone, Copy)]
pub enum PathPrefix {
    Inverse,
    Negated,
}

impl PathPrefix {
    pub fn character(&self) -> char {
        match self {
            PathPrefix::Inverse => '^',
            PathPrefix::Negated => '!',
        }
    }
}

pub trait PropertyOfType {}

#[derive(Clone, Copy)]
pub struct Property<T: PropertyOfType>(pub T);

#[derive(Clone, Copy)]
pub struct Basic<'a>(pub &'static str, pub &'a str);
#[derive(Clone, Copy)]
pub struct Wikidata(pub &'static str, pub u32);
#[derive(Clone, Copy)]
pub struct Suffixed(pub &'static str, pub u32, pub PathSuffix);
#[derive(Clone, Copy)]
pub struct Prefixed(pub &'static str, pub u32, pub PathPrefix);

impl PropertyOfType for Basic<'_> {}
impl PropertyOfType for Wikidata {}
impl PropertyOfType for Suffixed {}
impl PropertyOfType for Prefixed {}

pub trait PathElement: SparqlTerm {
    fn boxed_with_namespace(&self, ns: &'static str) -> Box<dyn PathElement>;
}

impl PathElement for Property<Wikidata> {
    fn boxed_with_namespace(&self, ns: &'static str) -> Box<dyn PathElement> {
        Box::new(self.with_namespace(ns))
    }
}

impl PathElement for Property<Suffixed> {
    fn boxed_with_namespace(&self, ns: &'static str) -> Box<dyn PathElement> {
        Box::new(self.with_namespace(ns))
    }
}

impl PathElement for Property<Prefixed> {
    fn boxed_with_namespace(&self, ns: &'static str) -> Box<dyn PathElement> {
        Box::new(self.with_namespace(ns))
    }
}

pub struct PropertySequence(pub Vec<Box<dyn PathElement>>);
pub struct PropertyOr(pub Vec<Box<dyn PathElement>>);

#[derive(Clone, Copy)]
pub struct Entity(pub &'static str, pub u32); // TODO wikidata specfic
pub struct Var(pub String);
pub struct Tag(pub &'static str, pub String);
pub struct Literal(pub &'static str);
pub struct LangLiteral(pub &'static str, pub &'static str);
pub struct TypedLiteral(pub &'static str, pub &'static str);
pub struct Blank<'a>(pub &'a [(&'a dyn Predicate, &'a dyn Object)]);

pub trait Subject: SparqlTerm {}
impl Subject for Var {}
impl Subject for Entity {}

pub trait Predicate: SparqlTerm {}
impl<'a> Predicate for Property<Basic<'a>> {}
impl Predicate for Property<Wikidata> {}
impl Predicate for Property<Suffixed> {}
impl Predicate for Property<Prefixed> {}
impl Predicate for PropertySequence {}
impl Predicate for PropertyOr {}

pub trait Object: SparqlTerm {}
impl Object for Entity {}
impl Object for Var {}
impl Object for Literal {}
impl Object for LangLiteral {}
impl Object for TypedLiteral {}
impl Object for Blank<'_> {}

pub struct SubjectPredicate<'a>(&'a dyn Subject, &'a dyn Predicate);

pub struct WithPredicateObjects<'a>(&'a dyn Subject, Vec<(&'a dyn Predicate, &'a dyn Object)>);

pub trait Qualifiable: Sized {
    fn with_namespace(&self, ns: &'static str) -> Self;

    fn link_statement(&self) -> Self {
        self.with_namespace("p")
    }

    fn to_statement(&self) -> Self {
        self.with_namespace("ps")
    }

    fn to_qualified(&self) -> Self {
        self.with_namespace("pq")
    }
}

impl Var {
    pub fn name(val: &str) -> Self {
        Var(val.to_string())
    }

    pub fn label(&self) -> Var {
        Var(format!("{}Label", self.0))
    }

    pub fn property<'a>(&'a self, prop: &'a dyn Predicate) -> SubjectPredicate<'a> {
        SubjectPredicate(self, prop)
    }

    pub fn properties<'a>(
        &'a self,
        predicate_objects: &[(&'a dyn Predicate, &'a dyn Object)],
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
        entity: &'a dyn Object,
    ) -> (&'a dyn Subject, &'a dyn Predicate, &'a dyn Object) {
        (self.0, self.1, entity)
    }

    pub fn set<'a>(
        &'a self,
        entity: &'a Var,
    ) -> (&'a dyn Subject, &'a dyn Predicate, &'a dyn Object) {
        (self.0, self.1, entity)
    }
}

impl Property<Wikidata> {
    pub fn suffixed(&self, suffix: PathSuffix) -> Property<Suffixed> {
        Property(Suffixed(self.0.0, self.0.1, suffix))
    }

    pub fn prefixed(&self, prefix: PathPrefix) -> Property<Prefixed> {
        Property(Prefixed(self.0.0, self.0.1, prefix))
    }

    pub fn append<P: PathElement + Clone + 'static>(&self, path_element: &P) -> PropertySequence {
        PropertySequence(vec![Box::new(*self), Box::new(path_element.clone())])
    }

    pub fn or<P: PathElement + Clone + 'static>(&self, path_element: &P) -> PropertyOr {
        PropertyOr(vec![Box::new(*self), Box::new(path_element.clone())])
    }

    pub fn is<'a>(&'a self, entity: &'a dyn Object) -> (&'a dyn Predicate, &'a dyn Object) {
        (self, entity)
    }

    pub fn set<'a>(&'a self, entity: &'a Var) -> (&'a dyn Predicate, &'a dyn Object) {
        (self, entity)
    }
}

impl Qualifiable for Property<Wikidata> {
    fn with_namespace(&self, ns: &'static str) -> Self {
        Property(Wikidata(ns, self.0.1))
    }
}

impl Property<Suffixed> {
    pub fn repeated(&self, suffix: PathSuffix) -> Property<Suffixed> {
        Property(Suffixed(self.0.0, self.0.1, suffix))
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

impl Qualifiable for Property<Suffixed> {
    fn with_namespace(&self, ns: &'static str) -> Self {
        Property(Suffixed(ns, self.0.1, self.0.2))
    }
}

impl Property<Prefixed> {
    pub fn repeated(&self, prefix: PathPrefix) -> Property<Prefixed> {
        Property(Prefixed(self.0.0, self.0.1, prefix))
    }

    pub fn append<P: PathElement + Clone + 'static>(&self, path_element: &P) -> PropertySequence {
        PropertySequence(vec![Box::new(*self), Box::new(path_element.clone())])
    }

    pub fn is<'a>(&'a self, entity: &'a dyn Object) -> (&'a dyn Predicate, &'a dyn Object) {
        (self, entity)
    }

    pub fn set<'a>(&'a self, entity: &'a dyn Object) -> (&'a dyn Predicate, &'a dyn Object) {
        (self, entity)
    }
}

impl Qualifiable for Property<Prefixed> {
    fn with_namespace(&self, ns: &'static str) -> Self {
        Property(Prefixed(ns, self.0.1, self.0.2))
    }
}

impl Qualifiable for PropertyOr {
    fn with_namespace(&self, ns: &'static str) -> Self {
        PropertyOr(
            self.0
                .iter()
                .map(|it| it.boxed_with_namespace(ns))
                .collect(),
        )
    }
}

impl PropertySequence {
    pub fn is<'a>(&'a self, entity: &'a dyn Object) -> (&'a dyn Predicate, &'a dyn Object) {
        (self, entity)
    }

    pub fn set<'a>(&'a self, entity: &'a dyn Object) -> (&'a dyn Predicate, &'a dyn Object) {
        (self, entity)
    }
}

impl PropertyOr {
    pub fn is<'a>(&'a self, entity: &'a dyn Object) -> (&'a dyn Predicate, &'a dyn Object) {
        (self, entity)
    }

    pub fn set<'a>(&'a self, entity: &'a dyn Object) -> (&'a dyn Predicate, &'a dyn Object) {
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
    fn to_sparql(&self) -> String;
}

impl SparqlTerm for Var {
    fn to_sparql(&self) -> String {
        format!("?{}", self.0)
    }
}

impl SparqlTerm for Entity {
    fn to_sparql(&self) -> String {
        format!("{}:Q{}", self.0, self.1)
    }
}

impl SparqlTerm for Property<Basic<'_>> {
    fn to_sparql(&self) -> String {
        format!("{}:{}", self.0.0, self.0.1)
    }
}

impl SparqlTerm for Property<Wikidata> {
    fn to_sparql(&self) -> String {
        format!("{}:P{}", self.0.0, self.0.1)
    }
}

impl SparqlTerm for Property<Suffixed> {
    fn to_sparql(&self) -> String {
        format!("{}:P{}{}", self.0.0, self.0.1, self.0.2.character())
    }
}

impl SparqlTerm for Property<Prefixed> {
    fn to_sparql(&self) -> String {
        format!("{}{}:P{}", self.0.2.character(), self.0.0, self.0.1)
    }
}

impl SparqlTerm for Tag {
    fn to_sparql(&self) -> String {
        format!("{}:{}", self.0, self.1)
    }
}

impl SparqlTerm for Literal {
    fn to_sparql(&self) -> String {
        format!("\"{}\"", self.0)
    }
}

impl SparqlTerm for LangLiteral {
    fn to_sparql(&self) -> String {
        format!("\"{}\"@{}", self.0, self.1)
    }
}

impl SparqlTerm for TypedLiteral {
    fn to_sparql(&self) -> String {
        format!("\"{}\"^^{}", self.0, self.1)
    }
}

impl SparqlTerm for Blank<'_> {
    fn to_sparql(&self) -> String {
        format!(
            "[ {} ]",
            self.0
                .iter()
                .map(|v| format!("{} {}", v.0.to_sparql(), v.1.to_sparql()))
                .join(" ; ")
        )
    }
}

impl SparqlTerm for PropertySequence {
    fn to_sparql(&self) -> String {
        self.0.iter().map(|v| v.to_sparql()).join("/")
    }
}

impl SparqlTerm for PropertyOr {
    fn to_sparql(&self) -> String {
        self.0.iter().map(|v| v.to_sparql()).join("|")
    }
}

impl SparqlTerm for SubjectPredicate<'_> {
    fn to_sparql(&self) -> String {
        format!("{} {}", self.0.to_sparql(), self.1.to_sparql())
    }
}

impl SparqlTerm for WithPredicateObjects<'_> {
    fn to_sparql(&self) -> String {
        let mut start = self.0.to_sparql();
        start.push_str(" ");
        start.push_str(
            &self
                .1
                .iter()
                .map(|v| format!("{} {}", v.0.to_sparql(), v.1.to_sparql()))
                .join(" ; "),
        );
        start.push_str(" .");
        start
    }
}

impl SparqlTerm for (&dyn Subject, &dyn Predicate, &dyn Object) {
    fn to_sparql(&self) -> String {
        format!(
            "{} {} {} .",
            self.0.to_sparql(),
            self.1.to_sparql(),
            self.2.to_sparql()
        )
    }
}

macro_rules! impl_display_via_sparql_term {
    ($($t:ty),* $(,)?) => {
        $(
            impl Display for $t {
                fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
                    formatter.write_str(&self.to_sparql())
                }
            }
        )*
    };
}

impl_display_via_sparql_term!(
    Var,
    Entity,
    Property<Wikidata>,
    Property<Suffixed>,
    Property<Prefixed>,
    Tag,
    PropertySequence,
    Literal,
    Blank<'_>,
);

#[cfg(feature = "fluent")]
pub mod fluent;
pub mod namespace;
