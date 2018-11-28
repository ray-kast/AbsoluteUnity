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
  use super::var::*;
  pub use crate::Value::{self, *};

  pub fn formalvar<S: Into<String>>(s: S) -> Value { Var(formal(s)) }

  pub fn atom<S: Into<String>>(s: S) -> Value { Atom(s.into()) }

  pub fn tuple<I: IntoIterator<Item = Value>>(i: I) -> Value {
    Tuple(i.into_iter().collect())
  }
}
