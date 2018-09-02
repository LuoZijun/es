
use version::ECMAScriptVersion;
use ast::statement::{ Statement, StatementList, };


pub trait Transform {
    type Item;

    fn transform(&mut self, target: ECMAScriptVersion) -> Self::Item;
}

impl Transform for Statement {
    type Item = Statement;

    fn transform(&mut self, target: ECMAScriptVersion) -> Self::Item {
        unimplemented!()
    }
}