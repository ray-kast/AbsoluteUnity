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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct App(RcPred, Vec<Value>);

impl App {
  pub fn new(pred: RcPred, args: Vec<Value>) -> Self {
    if args.len() != pred.1 {
      panic!("App::new: mismatch between predicate arity and argument count");
    }

    App(pred, args)
  }

  pub fn into_parts(self) -> (RcPred, Vec<Value>) { (self.0, self.1) }
}

impl Thing for App {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    for val in &self.1 {
      val.collect_free_vars(set);
    }
  }

  fn sub(mut self, sub: &Sub) -> Self {
    // TODO: maybe substitute predicates?

    for val in &mut self.1 {
      val.sub_self(sub);
    }

    self
  }
}

impl Unify for App {
  fn unify(&self, rhs: &App) -> Result<Sub> {
    if self.0 != rhs.0 {
      return Err(ErrorKind::PredMismatch(self.0.clone(), rhs.0.clone()).into());
    }

    assert!(self.1.len() == rhs.1.len());

    let mut ret = Sub::top();

    for (a, b) in self.1.iter().zip(rhs.1.iter()) {
      let sub = &a.clone().sub(&ret).unify(b)?;
      ret = ret.sub(sub);
    }

    Ok(ret)
  }
}

impl Display for App {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.0 .0, fmt)?;

    fmt.write_str("(")?;

    let mut first = true;

    for val in &self.1 {
      if first {
        first = false;
      } else {
        fmt.write_str(", ")?;
      }

      Display::fmt(val, fmt)?;
    }

    fmt.write_str(")")?;

    Ok(())
  }
}
