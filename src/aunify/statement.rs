use super::prelude::*;

#[derive(Debug)]
pub struct Statement(Clause, Clause);

impl Statement {
  #[inline]
  pub fn new(given: Clause, then: Clause) -> Self { Statement(given, then) }

  #[inline]
  pub fn assert(fact: Clause) -> Self { Statement(Clause::top(), fact) }
}
