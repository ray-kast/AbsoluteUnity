use super::prelude::*;
use std::rc::Rc;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Pred(String, usize);

impl Pred {
  pub fn new_rc(name: String, arity: usize) -> Rc<Self> {
    Rc::new(Pred(name, arity))
  }
}

#[derive(PartialEq, Debug)]
pub struct App(Rc<Pred>, Vec<Value>);

impl App {
  pub fn new(pred: Rc<Pred>, args: Vec<Value>) -> Self {
    if args.len() != pred.1 {
      panic!("App::new: mismatch between predicate arity and argument count");
    }

    App(pred, args)
  }

  pub fn unify(&self, rhs: &App) -> Result<Sub> {
    if self.0 != rhs.0 {
      return Err(ErrorKind::BadAppUnify(
        format!("{:?}", self.0),
        format!("{:?}", rhs.0),
      ).into());
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
