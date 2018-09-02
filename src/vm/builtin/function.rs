
use value::String;
use value::Value;
use error::Error;

// use js::ast::StatementList;

use std::string;
use std::vec::Vec;


// anonymous
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Function {
    name: String,
    args: Vec<String>,
    // body: StatementList,
}

impl Function {
    pub fn new(name: String, args: Vec<String>, /*body: StatementList*/) -> Result<Function, Error> {
        Ok(Function {
            name: name,
            args: args,
            // body: body
        })
    }

    pub fn call(&self) -> Result<Value, Error> {
        unimplemented!()
    }
}

