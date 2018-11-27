#![feature(generators, generator_trait)]

#[macro_use]
extern crate error_chain; // TODO: this should be able to be removed, somehow

pub mod clause;
pub mod env;
pub mod gen_iter;
pub mod pred;
pub mod scheme;
pub mod statement;
pub mod sub;
pub mod thing;
pub mod value;
pub mod var;

pub use self::{
  clause::Clause,
  env::Env,
  pred::{App, Pred, RcPred},
  scheme::{MaybeScheme, Scheme},
  statement::Statement,
  sub::Sub,
  thing::{Thing, Unify},
  value::Value,
  var::{Var, VarSource},
};

error_chain! {
  types { Error, ErrorKind, ResultExt, Result; }

  foreign_links {}

  errors {
    // TODO: add some kind of traceback?
    BadValueUnify(a: Value, b: Value) {
      description("values couldn't be unified")
      display("values {} and {} couldn't be unified", a, b)
    }

    PredMismatch(a: RcPred, b: RcPred) {
      description("predicate mismatch")
      display("predicates {} and {} don't match", a, b)
    }

    UnsolvableUnify(msg: &'static str) {
      description("unsolvable unify")
      display("unsolvable unify: {}", msg)
    }
  }
}

#[cfg(test)]
mod tests;

mod prelude {
  pub use super::{thing::UnifyCore, *};
  pub use std::{
    collections::{hash_map::Entry as HashEntry, HashMap, HashSet},
    fmt::{self, Display},
  };
}
