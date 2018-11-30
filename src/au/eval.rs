use super::prelude::*; // TODO: replace super::prelude with crate::prelude
use crate::ast::{Command, CompileCtx, CompileTo, Expr, Input};
use aunify::{App, Env, MaybeScheme, Statement, Sub, Thing, Value, VarSource};

pub struct Evaluator {
  env: Env,
  var_src: VarSource,
  compile_ctx: CompileCtx,
}

pub enum EvalResult<'a> {
  Unit,
  Assert(Vec<MaybeScheme<Statement>>),
  Query(Box<Iterator<Item = Sub> + 'a>),
  UnifyVal(aunify::Result<(Value, Value, Sub, Value, Value)>),
  UnifyApp(aunify::Result<(App, App, Sub, App, App)>),
  PrintVal(MaybeScheme<Value>),
  PrintStmt(MaybeScheme<Statement>),
  PrintEnv(&'a Vec<MaybeScheme<Statement>>),
}

impl Evaluator {
  pub fn new() -> Self {
    Self {
      env: Env::new(),
      var_src: VarSource::new(),
      compile_ctx: CompileCtx::new(),
    }
  }

  pub fn eval<'a>(&'a mut self, ast: Expr) -> Result<EvalResult> {
    Ok(match ast.compile(&mut self.compile_ctx)? {
      Command::Assert(v) => {
        for stmt in v.clone() {
          self.env.state(stmt);
        }

        EvalResult::Assert(v)
      },
      Command::Query(c) => {
        EvalResult::Query(Box::new(self.env.solve_clause(c, &mut self.var_src)))
      },
      Command::UnifyVal(a, b) => EvalResult::UnifyVal(
        a.inst_and_unify(b, &mut self.var_src)
          .and_then(|(a, b, sub)| {
            let a1 = a.clone();
            let b1 = b.clone();
            let a2 = a.sub(&sub)?;
            let b2 = b.sub(&sub)?;

            Ok((a1, b1, sub, a2, b2))
          }),
      ),
      Command::UnifyApp(a, b) => EvalResult::UnifyApp(
        a.inst_and_unify(b, &mut self.var_src)
          .and_then(|(a, b, sub)| {
            let a1 = a.clone();
            let b1 = b.clone();
            let a2 = a.sub(&sub)?;
            let b2 = b.sub(&sub)?;

            Ok((a1, b1, sub, a2, b2))
          }),
      ),
      Command::PrintVal(v) => EvalResult::PrintVal(v),
      Command::PrintStmt(s) => EvalResult::PrintStmt(s),
      Command::Fold(mut n) => {
        n.fold();
        EvalResult::PrintVal(MaybeScheme::Inst(Value::Numeric(n)))
      },
      Command::PrintEnv => EvalResult::PrintEnv(self.env.premises()),
      Command::Reset => {
        *self = Evaluator::new();
        EvalResult::Unit
      },
    })
  }

  pub fn eval_input(&mut self, ast: Input) -> Result<()> {
    for stmt in ast.compile(&mut self.compile_ctx)? {
      self.env.state(stmt);
    }

    Ok(())
  }
}
