// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use crate::Error;
use log::trace;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct QueryParser;

pub struct Query {}

impl Query {
    pub fn new(query: &str) -> Result<(), Error> {
        trace!("query: {}", query);
        let pairs = match QueryParser::parse(Rule::query, query) {
            Err(err) => {
                return Err(Error::InvalidInput(format!(
                    "query: {} invalid:{}",
                    query, err
                )))
            }
            Ok(q) => q,
        };

        for pair in pairs {
            trace!("rule: {:?}", pair.as_rule());
            trace!("span: {:?}", pair.as_span());
            trace!("text: {:?}", pair.as_str());

            for inner_pair in pair.into_inner() {
                trace!("inner rule: {:?}", inner_pair.as_rule());
                trace!("inner span: {:?}", inner_pair.as_span());
                trace!("inner text: {:?}", inner_pair.as_str());
            }
        }

        Ok(())
    }
}
