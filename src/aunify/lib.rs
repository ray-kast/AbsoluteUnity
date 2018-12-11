#![feature(generators, generator_trait)]

#[macro_use]
extern crate error_chain; // TODO: this should be able to be removed, somehow

pub mod bind;
pub mod clause;
pub mod env;
pub mod gen_iter;
pub mod list;
pub mod numeric;
pub mod pred;
pub mod scheme;
pub mod statement;
pub mod sub;
pub mod thing;
pub mod tracer;
pub mod tuple;
pub mod value;
pub mod var;

pub use self::{
  bind::{Bind, MaybeBind},
  clause::Clause,
  env::Env,
  list::List,
  numeric::Numeric,
  pred::{App, Pred, RcPred},
  scheme::{MaybeScheme, Scheme},
  statement::Statement,
  sub::Sub,
  thing::{Thing, Unify},
  tracer::NilTracer,
  tuple::Tuple,
  value::Value,
  var::{Var, VarSource},
};

error_chain! {
  types { Error, ErrorKind, ResultExt, Result; }

  foreign_links {}

  errors {
    DuplicateSub {
      description("duplicate variable in substitution")
      display("duplicate variable in substitution")
    }

    SubBadType(expect: &'static str, got: Value) {
      description("substitution failure: bad type")
      display("substitution failure: expected {}, got {}", expect, got)
    }

    // TODO: add some kind of traceback?
    BadValueUnify(a: Value, b: Value) {
      description("values couldn't be unified")
      display("values {} and {} couldn't be unified", a, b)
    }

    // TODO: add some kind of traceback?
    BadNumericUnify(a: Numeric, b: Numeric) {
      description("numeric expressions couldn't be unified")
      display("numeric expressions {} and {} couldn't be unified", a, b)
    }

    // TODO: add some kind of traceback?
    BadTupleUnify(a: Tuple, b: Tuple) {
      description("tuples couldn't be unified")
      display("tuples {} and {} couldn't be unified", a, b)
    }

    // TODO: add some kind of traceback?
    BadListUnify(a: List, b: List) {
      description("lists couldn't be unified")
      display("lists {} and {} couldn't be unified", a, b)
    }

    PredMismatch(a: RcPred, b: RcPred) {
      description("predicate mismatch")
      display("predicates {} and {} don't match", a, b)
    }

    // VarBothSides(var: Var) {
    //   description("variable on both sides of unify")
    //   display("unsolvable unify: {} is on both sides", var)
    // }
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
