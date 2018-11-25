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

#[derive(PartialEq, Debug)]
pub struct App(RcPred, Vec<Value>);

impl App {
  pub fn new(pred: RcPred, args: Vec<Value>) -> Self {
    if args.len() != pred.1 {
      panic!("App::new: mismatch between predicate arity and argument count");
    }

    App(pred, args)
  }
}

impl Thing for App {
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

    let mut ret = Ok(Sub::top());

    for (a, b) in self.1.iter().zip(rhs.1.iter()) {
      if let Ok(sub) = ret {
        ret = a.clone().sub(&sub).unify(b).map(|s| sub.sub(&s));
      } else {
        break;
      }
    }

    ret
  }
}
