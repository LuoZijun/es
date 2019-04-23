use std::collections::{ VecDeque, HashMap, };

pub mod numberic;
pub mod ustring;

pub mod statement;
pub mod expression;
pub mod function;
pub mod class;

// pub mod declaration;
// pub mod pattern;
// pub mod module;

pub mod jsx;

// #[derive(Debug, PartialEq, Clone)]
// pub struct Module {
//     pub imports: HashMap<String, String>,
//     pub exports: HashMap<String, String>,
//     // pub body: Vec<statement::Statement>,
// }


#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {
        match *self {
            Either::Left(_) => true,
            _ => false,
        }
    }
    
    pub fn is_right(&self) -> bool {
        match *self {
            Either::Right(_) => true,
            _ => false,
        }
    }

    pub fn left(&self) -> Option<&L> {
        match self {
            Either::Left(v) => Some(v),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<&R> {
        match self {
            Either::Right(v) => Some(v),
            _ => None,
        }
    }
}

