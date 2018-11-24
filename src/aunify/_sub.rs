use super::prelude::*;

pub struct Sub<T>(HashMap<Var, Value<T>>);

impl<T> Sub<T> {
  pub fn new() -> Self { Sub(HashMap::new()) }

  pub fn with(mut self, var: Var, is: Value<T>) -> Self {
    if self.0.insert(var, is).is_some() {
      panic!("duplicate substitution"); // TODO: this is bad
    }

    self
  }
}