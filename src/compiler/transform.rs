
use version::ECMAScriptVersion;
use ast::statement::{ Statement, StatementList, };


pub trait Transform {
    type Item;

    fn transform(&mut self, target: ECMAScriptVersion) -> Self::Item;
}

impl<'ast> Transform for Statement<'ast> {
    type Item = Statement<'ast>;

    fn transform(&mut self, target: ECMAScriptVersion) -> Self::Item {
        unimplemented!()
    }
}