use itertools::Itertools;
use sparql_client::{Error, SparqlBinding, SparqlClient};
use std::marker::PhantomData;

use crate::{Entity, Object, Predicate, SparqlTerm, Subject, Tag, Var, WithPredicateObjects};

pub struct New;
pub struct Select;
pub struct Where;
pub struct Done;

pub trait QueryState {}

impl QueryState for New {}
impl QueryState for Select {}
impl QueryState for Where {}
impl QueryState for Done {}

pub struct Query<Q: QueryState> {
    _marker: PhantomData<Q>,
    pub sparql: String,
}

impl Query<New> {
    fn _select(select_type: &str, vars: &[&Var]) -> Query<Select> {
        Query::<Select> {
            _marker: PhantomData,
            sparql: format!(
                "SELECT {}{}",
                select_type,
                vars.iter().map(|v| v.to_sparql()).join(" ")
            ),
        }
    }

    pub fn select(vars: &[&Var]) -> Query<Select> {
        Self::_select("", vars)
    }

    pub fn select_distinct(vars: &[&Var]) -> Query<Select> {
        Self::_select("DISTINCT ", vars)
    }
}

impl Query<Select> {
    pub fn where_(&self) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            sparql: format!("{} WHERE {{", self.sparql),
        }
    }
}

impl Query<Where> {
    pub fn add(&self, triple: (&dyn Subject, &dyn Predicate, &dyn Object)) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            sparql: format!(
                "{} {} {} {} .",
                self.sparql,
                triple.0.to_sparql(),
                triple.1.to_sparql(),
                triple.2.to_sparql()
            ),
        }
    }

    pub fn add_all(&self, with_predicate_objects: WithPredicateObjects) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            sparql: format!("{} {}", self.sparql, with_predicate_objects.to_sparql(),),
        }
    }

    pub fn optional(&self, triple: (&dyn Subject, &dyn Predicate, &dyn Object)) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            sparql: format!(
                "{} OPTIONAL{{ {} {} {} .}}",
                self.sparql,
                triple.0.to_sparql(),
                triple.1.to_sparql(),
                triple.2.to_sparql()
            ),
        }
    }

    pub fn service(&self, tag: &Tag, service: &Service) -> Self {
        Self {
            _marker: PhantomData,
            sparql: format!("{} SERVICE {} {{ {} }}", self.sparql, tag, service.sparql),
        }
    }

    pub fn union(&self, triples: &[(&dyn Subject, &dyn Predicate, &dyn Object)]) -> Self {
        Self {
            _marker: PhantomData,
            sparql: format!(
                "{} {}",
                self.sparql,
                triples
                    .iter()
                    .map(|triple| format!(
                        "{{ {} {} {} . }}",
                        triple.0.to_sparql(),
                        triple.1.to_sparql(),
                        triple.2.to_sparql()
                    ))
                    .join(" UNION ")
            ),
        }
    }

    pub fn filter_not_exists(&self, filter: &Filter) -> Self {
        Self {
            _marker: PhantomData,
            sparql: format!("{} FILTER NOT EXISTS {{ {} }}", self.sparql, filter.sparql),
        }
    }

    pub fn order_by_asc(&self, var: &Var) -> Query<Done> {
        Query::<Done> {
            _marker: PhantomData,
            sparql: format!("{} }} ORDER BY ASC({})", self.sparql, var),
        }
    }

    pub fn limit(&self, limit: u32) -> Query<Done> {
        Query::<Done> {
            _marker: PhantomData,
            sparql: format!("{} }} LIMIT {}", self.sparql, limit),
        }
    }
}

impl Query<Done> {
    pub async fn query(&self, sparql_client: &SparqlClient) -> Result<Vec<SparqlBinding>, Error> {
        eprintln!("Query:\n {}", &self.sparql);
        sparql_client.sparql_query(&self.sparql).await
    }

    pub fn limit(&self, limit: u32) -> Self {
        Self {
            _marker: PhantomData,
            sparql: format!("{} LIMIT {}", self.sparql, limit),
        }
    }
}

pub struct Filter {
    sparql: String,
}

impl Filter {
    pub fn new() -> Self {
        Filter {
            sparql: "".to_string(),
        }
    }
}

impl Filter {
    pub fn values(&mut self, var: Var, names: &[&Entity]) -> &mut Self {
        self.sparql.push_str(&format!(
            "VALUES {} {{ {} }}",
            var.to_sparql(),
            names.iter().map(|it| it.to_sparql()).join(" ")
        ));
        self
    }

    pub fn add(&mut self, triple: (&dyn Subject, &dyn Predicate, &dyn Object)) -> &mut Self {
        self.sparql.push_str(&format!(
            "{} {} {} .",
            triple.0.to_sparql(),
            triple.1.to_sparql(),
            triple.2.to_sparql()
        ));
        self
    }
}

pub struct Service {
    sparql: String,
}

impl Service {
    pub fn new() -> Self {
        Service {
            sparql: "".to_string(),
        }
    }
}

impl Service {
    pub fn add(
        &mut self,
        t1: &dyn SparqlTerm,
        t2: &dyn SparqlTerm,
        t3: &dyn SparqlTerm,
    ) -> &mut Self {
        self.sparql.push_str(&format!(
            "{} {} {} .",
            t1.to_sparql(),
            t2.to_sparql(),
            t3.to_sparql()
        ));
        self
    }
}
