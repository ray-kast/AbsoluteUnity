use crate::ast::Expr;
use aunify::{Sub, VarSource};

pub struct Evaluator {
  var_src: VarSource,
}

pub enum EvalResult {
  Unify(aunify::Result<Sub>),
}

impl Evaluator {
  pub fn new() -> Self {
    Self {
      var_src: VarSource::new(),
    }
  }

  pub fn eval(&mut self, ast: Expr) -> EvalResult {
    match ast {
      Expr::Assert(_s) => unimplemented!(),
      Expr::Query(_s) => unimplemented!(),
      Expr::UnifyVal(mut a, mut b) => {
        EvalResult::Unify(a.unify_inst(&mut b, &mut self.var_src))
      },
      Expr::UnifyApp(mut a, mut b) => {
        EvalResult::Unify(a.unify_inst(&mut b, &mut self.var_src))
      },
    }
  }
}
