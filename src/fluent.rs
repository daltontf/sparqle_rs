use itertools::Itertools;
use sparql_client::{Error, SparqlBinding, SparqlClient};

use crate::{Entity, SparqlTerm, Tag, Var};

pub struct New;
pub struct Select;
pub struct Where;
pub struct Done;

pub trait QueryState {}

impl QueryState for New {}
impl QueryState for Select {}
impl QueryState for Where {}
impl QueryState for Done {}

use std::marker::PhantomData;
pub struct Query<Q: QueryState> {
    _marker: PhantomData<Q>,
    pub statement: String,
}

impl Query<New> {
    fn _select(select_type: &str, vars: Vec<&Var>) -> Query<Select> {
        Query::<Select> {
            _marker: PhantomData,
            statement: format!(
                "SELECT {}{}",
                select_type,
                vars.iter().map(|v| v.to_query()).join(" ")
            ),
        }
    }

    pub fn select(vars: Vec<&Var>) -> Query<Select> {
        Self::_select("", vars)
    }

    pub fn select_distinct(vars: Vec<&Var>) -> Query<Select> {
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
    pub fn add(&self, triple: (&dyn SparqlTerm, &dyn SparqlTerm, &dyn SparqlTerm)) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            statement: format!(
                "{} {} {} {} .",
                self.statement,
                triple.0.to_query(),
                triple.1.to_query(),
                triple.2.to_query()
            ),
        }
    }

    pub fn optional(
        &self,
        triple: (&dyn SparqlTerm, &dyn SparqlTerm, &dyn SparqlTerm),
    ) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            statement: format!(
                "{} OPTIONAL{{ {} {} {} .}}",
                self.statement,
                triple.0.to_query(),
                triple.1.to_query(),
                triple.2.to_query()
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

    pub fn union(&self, triples: Vec<(&dyn SparqlTerm, &dyn SparqlTerm, &dyn SparqlTerm)>) -> Self {
        Self {
            _marker: PhantomData,
            statement: format!(
                "{} {}",
                self.statement,
                triples
                    .iter()
                    .map(|triple| format!(
                        "{{ {} {} {} . }}",
                        triple.0.to_query(),
                        triple.1.to_query(),
                        triple.2.to_query()
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
    pub fn values(&mut self, var: Var, names: Vec<&Entity>) -> &mut Self {
        self.statement.push_str(&format!(
            "VALUES {} {{ {} }}",
            var.to_query(),
            names.iter().map(|it| it.to_query()).join(" ")
        ));
        self
    }

    pub fn add(
        &mut self,
        triple: (&dyn SparqlTerm, &dyn SparqlTerm, &dyn SparqlTerm),
    ) -> &mut Self {
        self.statement.push_str(&format!(
            "{} {} {} .",
            triple.0.to_query(),
            triple.1.to_query(),
            triple.2.to_query()
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
        t1: &impl SparqlTerm,
        t2: &impl SparqlTerm,
        t3: &impl SparqlTerm,
    ) -> &mut Self {
        self.statement.push_str(&format!(
            "{} {} {} .",
            t1.to_query(),
            t2.to_query(),
            t3.to_query()
        ));
        self
    }
}
