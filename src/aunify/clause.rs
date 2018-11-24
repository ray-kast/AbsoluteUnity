use super::prelude::*;

#[derive(Debug)]
pub enum Clause {
  App(App),
  Not(Box<Clause>),
  Any(Vec<Clause>),
  All(Vec<Clause>),
}

impl Clause {
  // Gonna reuse Any and All for top and bottom for simplicity's sake

  #[inline]
  pub fn top() -> Self { Clause::All(Vec::new()) }

  #[inline]
  pub fn bot() -> Self { Clause::Any(Vec::new()) }
}
