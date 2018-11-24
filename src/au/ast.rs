pub use aunify::{App, Clause, Pred, Statement, Value, Var};
use std::{collections::HashMap, rc::Rc};

pub struct ParserTag {
  // Yeah, I'm aware this stores redundant values.
  pred_src: HashMap<(String, usize), Rc<Pred>>,
}

impl ParserTag {
  pub fn new() -> Self {
    Self {
      pred_src: HashMap::new(),
    }
  }

  pub fn make_app(&mut self, name: String, vals: Vec<Value>) -> App {
    let pred = self
      .pred_src
      .entry((name.clone(), vals.len()))
      .or_insert_with(|| Pred::new_rc(name, vals.len()));

    App::new(pred.clone(), vals)
  }
}

#[derive(Debug)]
pub enum Expr {
  Assert(Statement),
  Query(Statement),
}
