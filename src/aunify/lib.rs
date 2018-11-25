#[macro_use]
extern crate error_chain; // TODO: this should be able to be removed, somehow

mod clause;
mod env;
mod pred;
mod scheme;
mod statement;
mod sub;
mod thing;
mod value;
mod var;

pub use self::{
  clause::*, env::*, pred::*, scheme::*, statement::*, sub::*, thing::*,
  value::*, var::*,
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
  }
}

#[cfg(test)]
mod tests;

mod prelude {
  pub use super::*;
  pub use std::{
    collections::{hash_map::Entry as HashEntry, HashMap, HashSet},
    fmt::{self, Display},
  };
}
