use super::prelude::*;

#[derive(Clone, Debug)]
pub struct Statement(App, Clause);

impl Statement {
  #[inline]
  pub fn new(given: App, then: Clause) -> Self { Statement(given, then) }

  #[inline]
  pub fn lhs(&self) -> &App { &self.0 }

  #[inline]
  pub fn rhs(&self) -> &Clause { &self.1 }

  #[inline]
  pub fn fact(given: App) -> Self { Statement(given, Clause::Top) }
}

impl Thing for Statement {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    self.0.collect_free_vars(set);
    self.1.collect_free_vars(set);
  }

  fn sub(self, sub: &Sub) -> Result<Self> {
    Ok(Statement(self.0.sub(sub)?, self.1.sub(sub)?))
  }
}

impl Display for Statement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.0, fmt)?;
    fmt.write_str(" ⊢ ")?;
    Display::fmt(&self.1, fmt)?;

    Ok(())
  }
}
