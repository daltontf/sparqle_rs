use itertools::Itertools;
use sparql_client::{Error, SparqlBinding, SparqlClient};
use std::marker::PhantomData;

use crate::{Entity, SparqlTerm, Tag, Var, WithPredicateObjects, Subject, Predicate, Object};

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
    pub statement: String,
}

impl Query<New> {
    fn _select(select_type: &str, vars: &[&Var]) -> Query<Select> {
        Query::<Select> {
            _marker: PhantomData,
            statement: format!(
                "SELECT {}{}",
                select_type,
                vars.iter().map(|v| v.to_statement()).join(" ")
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
            statement: format!("{} WHERE {{", self.statement),
        }
    }
}

impl Query<Where> {
    pub fn add(&self, triple: (&dyn Subject, &dyn Predicate, &dyn Object)) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            statement: format!(
                "{} {} {} {} .",
                self.statement,
                triple.0.to_statement(),
                triple.1.to_statement(),
                triple.2.to_statement()
            ),
        }
    }

    pub fn add_all(&self, with_predicate_objects: WithPredicateObjects) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            statement: format!("{} {}", self.statement, with_predicate_objects.to_statement(),),
        }
    }

    pub fn optional(
        &self,
        triple: (&dyn Subject, &dyn Predicate, &dyn Object),
    ) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            statement: format!(
                "{} OPTIONAL{{ {} {} {} .}}",
                self.statement,
                triple.0.to_statement(),
                triple.1.to_statement(),
                triple.2.to_statement()
            ),
        }
    }

    pub fn service(&self, tag: &Tag, service: &Service) -> Self {
        Self {
            _marker: PhantomData,
            statement: format!(
                "{} SERVICE {} {{ {} }}",
                self.statement, tag, service.statement
            ),
        }
    }

    pub fn union(&self, triples: &[(&dyn Subject, &dyn Predicate, &dyn Object)]) -> Self {
        Self {
            _marker: PhantomData,
            statement: format!(
                "{} {}",
                self.statement,
                triples
                    .iter()
                    .map(|triple| format!(
                        "{{ {} {} {} . }}",
                        triple.0.to_statement(),
                        triple.1.to_statement(),
                        triple.2.to_statement()
                    ))
                    .join(" UNION ")
            ),
        }
    }

    pub fn filter_not_exists(&self, filter: &Filter) -> Self {
        Self {
            _marker: PhantomData,
            statement: format!(
                "{} FILTER NOT EXISTS {{ {} }}",
                self.statement, filter.statement
            ),
        }
    }

    pub fn order_by_asc(&self, var: &Var) -> Query<Done> {
        Query::<Done> {
            _marker: PhantomData,
            statement: format!("{} }} ORDER BY ASC({})", self.statement, var),
        }
    }

    pub fn limit(&self, limit: u32) -> Query<Done> {
        Query::<Done> {
            _marker: PhantomData,
            statement: format!("{} }} LIMIT {}", self.statement, limit),
        }
    }
}

impl Query<Done> {
    pub async fn query(&self, sparql_client: &SparqlClient) -> Result<Vec<SparqlBinding>, Error> {
        eprintln!("Query:\n {}", &self.statement);
        sparql_client.sparql_query(&self.statement).await
    }

    pub fn limit(&self, limit: u32) -> Self {
        Self {
            _marker: PhantomData,
            statement: format!("{} LIMIT {}", self.statement, limit),
        }
    }
}

pub struct Filter {
    statement: String,
}

impl Filter {
    pub fn new() -> Self {
        Filter {
            statement: "".to_string(),
        }
    }
}

impl Filter {
    pub fn values(&mut self, var: Var, names: &[&Entity]) -> &mut Self {
        self.statement.push_str(&format!(
            "VALUES {} {{ {} }}",
            var.to_statement(),
            names.iter().map(|it| it.to_statement()).join(" ")
        ));
        self
    }

    pub fn add(
        &mut self,
        triple: (&dyn Subject, &dyn Predicate, &dyn Object)
    ) -> &mut Self {
        self.statement.push_str(&format!(
            "{} {} {} .",
            triple.0.to_statement(),
            triple.1.to_statement(),
            triple.2.to_statement()
        ));
        self
    }
}

pub struct Service {
    statement: String,
}

impl Service {
    pub fn new() -> Self {
        Service {
            statement: "".to_string(),
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
        self.statement.push_str(&format!(
            "{} {} {} .",
            t1.to_statement(),
            t2.to_statement(),
            t3.to_statement()
        ));
        self
    }
}
