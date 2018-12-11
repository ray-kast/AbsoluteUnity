use super::{prelude::*, tracer::prelude::*};
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
    Display::fmt(&self.1, fmt)
  }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct App(RcPred, Tuple);

impl App {
  pub fn new(pred: RcPred, args: Tuple) -> Self {
    if args.0.len() != pred.1 {
      panic!("App::new: mismatch between predicate arity and argument count");
    }

    App(pred, args)
  }

  pub fn pred(&self) -> &RcPred { &self.0 }
}

impl Thing for App {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    self.1.collect_free_vars(set);
  }

  fn sub_impl<T: ThingTracer>(
    self,
    sub: &Sub,
    tracer: T::SubHandle,
  ) -> Result<Self> {
    Ok(App(self.0, self.1.sub(sub, tracer)?))
  }

  fn can_sub(&self, sub: &Sub) -> bool { self.1.can_sub(sub) }
}

impl Unify for App {
  fn unify_impl<T: UnifyTracer>(
    &self,
    rhs: &App,
    tracer: T::UnifyHandle,
  ) -> Result<Sub> {
    if self.0 != rhs.0 {
      return Err(ErrorKind::PredMismatch(self.0.clone(), rhs.0.clone()).into());
    }

    assert!(self.1 .0.len() == rhs.1 .0.len());

    self.1.unify(&rhs.1, tracer)
  }
}

impl Display for App {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.0 .0, fmt)?;
    Display::fmt(&self.1, fmt)
  }
}
