use crate::version::ECMAScriptVersion;
use crate::ast::statement::{ Statement, };
use crate::ast::expression::{ Expression, };

use std::io::{ self, Write, };

// transpiler
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


impl<'ast> Transform for Expression<'ast> {
    type Item = Expression<'ast>;

    fn transform(&mut self, target: ECMAScriptVersion) -> Self::Item {
        unimplemented!()
    }
}


pub trait ByteCodeGen {
    fn byte_code_gen<W: Write>(&self, output: &mut W);
}

pub trait ToSourceCode {
    fn source_code_gen<W: Write>(&self, output: &mut W);
}

pub trait DebugSourceCodeGen {
    fn debug_source_code_gen<W: Write>(&self, output: &mut W);
}
