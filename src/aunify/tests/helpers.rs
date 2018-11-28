pub mod misc {
  pub use std::{collections::HashSet, hash::Hash};

  pub fn hset<T: Hash + Eq, I: IntoIterator<Item = T>>(i: I) -> HashSet<T> {
    i.into_iter().collect()
  }
}

pub mod var {
  pub use crate::Var::{self, *};

  pub fn formal<S: Into<String>>(s: S) -> Var { Formal(s.into()) }
}

pub mod value {
  use super::{tuple::*, var::*};
  pub use crate::Value::{self, Tuple as TupleV, *};

  pub fn formalv<S: Into<String>>(s: S) -> Value { Var(formal(s)) }

  pub fn atomv<S: Into<String>>(s: S) -> Value { Atom(s.into()) }

  pub fn tuplev<I: IntoIterator<Item = Value>>(i: I) -> Value {
    TupleV(tuple(i))
  }
}

pub mod tuple {
  pub use crate::Tuple;
  use crate::Value;

  pub fn tuple<I: IntoIterator<Item = Value>>(i: I) -> Tuple {
    Tuple(i.into_iter().collect())
  }
}
