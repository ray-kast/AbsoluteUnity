#[macro_use]
extern crate error_chain; // TODO: this should be able to be removed, somehow

mod clause;
mod env;
mod pred;
mod statement;
mod sub;
mod value;
mod var;

error_chain! {
  types { Error, ErrorKind, ResultExt, Result; }

  foreign_links {}

  errors {
    BadValueUnify(a: String, b: String) {
      description("values couldn't be unified")
      display("values {} and {} couldn't be unified", a, b)
    }

    BadAppUnify(a: String, b: String) {
      description("applications couldn't be unified")
      display("applications {} and {} couldn't be unified", a, b)
    }

    PredMismatch(a: String, b: String) {
      description("predicate mismatch")
      display("predicates {} and {} don't match", a, b)
    }
  }
}

pub use self::{
  clause::*, env::*, pred::*, statement::*, sub::*, value::*, var::*,
};

#[cfg(test)]
mod tests;

mod prelude {
  pub use super::*;
  pub use std::collections::{hash_map::Entry as HashEntry, HashMap, HashSet};
}
