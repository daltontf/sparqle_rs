use itertools::Itertools;
use sparql_client::{Error, SparqlBinding, SparqlClient};

use crate::{Entity, SparqlTerm, Var, Tag};

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
    pub query: String,
}

// impl <Q: QueryState> Query<Q> {
//
// }

impl Query<New> {
    pub fn select(vars: Vec<&Var>) -> Query<Select> {
        Query::<Select> {
            _marker: PhantomData,
            query: format!("SELECT {}", vars.iter().map(|v| v.to_query()).join(" ")),
        }
    }
}

impl Query<Select> {
    pub fn where_(&self) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            query: format!("{} WHERE {{", self.query),
        }
    }
}

impl Query<Where> {
    pub fn add(
        &self,
        t1: &impl SparqlTerm,
        t2: &impl SparqlTerm,
        t3: &impl SparqlTerm,
    ) -> Query<Where> {
        Query::<Where> {
            _marker: PhantomData,
            query: format!(
                "{} {} {} {} .",
                self.query,
                t1.to_query(),
                t2.to_query(),
                t3.to_query()
            ),
        }
    }

    pub fn service(&self, tag: &Tag, service: &Service) -> Self {
        Self {
            _marker: PhantomData,
            query: format!(
                "{} SERVICE {} {{ {} }}",
                self.query, 
                tag, 
                service.statement
            )
        }
    }         

    pub fn filter_not_exists(&self, filter: &Filter) -> Self {
        Self {
            _marker: PhantomData,
            query: format!(
                "{} FILTER NOT EXISTS {{ {} }}",
                self.query, 
                filter.statement
            ),
        }
    }

    pub fn order_by_asc(&self, var: &Var) -> Query<Done> {
        Query::<Done> {
            _marker: PhantomData,
            query: format!(
                "{} }} ORDER BY ASC({})",
                self.query, 
                var
            ),
        }
    }

    pub fn limit(&self, limit: u32) -> Query<Done> {
        Query::<Done> {
            _marker: PhantomData,
            query: format!("{} }} LIMIT {}", self.query, limit),
        }
    }
}

impl Query<Done> {
    pub async fn query(&self, sparql_client: &SparqlClient) -> Result<Vec<SparqlBinding>, Error> {
        eprintln!("Query:\n {}", &self.query);
        sparql_client.sparql_query(&self.query).await
    }

    pub fn limit(&self, limit: u32) -> Self {
        Self {
            _marker: PhantomData,
            query: format!("{} LIMIT {}", self.query, limit),
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

pub struct Service {
    statement: String,
}

impl Service {
    pub fn new() -> Self {
        Service {
            statement: "".to_string()
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
