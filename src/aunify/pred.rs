use super::prelude::*;
use std::sync::Arc;

// TODO: Change this back to an Rc if we stop passing it to the error_chain types
pub type RcPred = Arc<Pred>;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Pred(String, usize);

impl Pred {
  pub fn new_rc(name: String, arity: usize) -> RcPred {
    Arc::new(Pred(name, arity))
  }
}

impl Display for Pred {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.0, fmt)?;
    fmt.write_str("/")?;
    Display::fmt(&self.1, fmt)?;

    Ok(())
  }
}

// TODO: this should really just use a tuple
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct App(RcPred, Tuple);

impl App {
  pub fn new(pred: RcPred, args: Tuple) -> Self {
    if args.0.len() != pred.1 {
      panic!("App::new: mismatch between predicate arity and argument count");
    }

    App(pred, args)
  }
}

impl Thing for App {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    self.1.collect_free_vars(set);
  }

  fn sub(self, sub: &Sub) -> Result<Self> { Ok(App(self.0, self.1.sub(sub)?)) }
}

impl Unify for App {
  fn unify(&self, rhs: &App) -> Result<Sub> {
    if self.0 != rhs.0 {
      return Err(ErrorKind::PredMismatch(self.0.clone(), rhs.0.clone()).into());
    }

    assert!(self.1 .0.len() == rhs.1 .0.len());

    self.1.unify(&rhs.1)
  }
}

impl IntoTrace for App {
  fn into_trace(self) -> Self { App(self.0, self.1.into_trace()) }
}

impl Display for App {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.0 .0, fmt)?;
    Display::fmt(&self.1, fmt)?;
    Ok(())
  }
}
