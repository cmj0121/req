// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use crate::Error;
use log::trace;
use pest::iterators::Pairs;
use pest::Parser as ParserTrait;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Parser;

pub(crate) enum QueryMode {
    Full,
    Group,
    Named,
}

impl QueryMode {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "f" | "full" => Some(QueryMode::Full),
            "g" | "group" => Some(QueryMode::Group),
            "n" | "named" => Some(QueryMode::Named),
            _ => None,
        }
    }
}

pub struct Query {
    // single-mode
    pub(crate) single_mode: bool,

    // query-mode
    pub(crate) query_mode: QueryMode,
}

impl Query {
    pub fn new(query: &str) -> Result<Query, Error> {
        trace!("query: {}", query);

        let mut q = Query::default();
        match Parser::parse(Rule::query, query) {
            Err(err) => Err(Error::InvalidInput(format!(
                "query: {} invalid:{}",
                query, err
            ))),
            Ok(pairs) => {
                q.parse_syntax(pairs)?;
                Ok(q)
            }
        }
    }

    fn parse_syntax(&mut self, pairs: Pairs<'_, Rule>) -> Result<(), Error> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::single_mode => {
                    self.single_mode = true;
                }
                Rule::query_mode => {
                    let token = pair.as_str();
                    self.query_mode =
                        QueryMode::from_str(token).expect(&format!("should implement: {}", token));
                }
                _ => {}
            }

            self.parse_syntax(pair.into_inner())?;
        }

        Ok(())
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            single_mode: false,
            query_mode: QueryMode::Full,
        }
    }
}
