use super::prelude::*;
use aunify::{tracer::prelude::*, App, Clause, Error as AError, Sub};

#[derive(Clone)]
pub struct AuTracer {
  lvl: usize,
}

impl AuTracer {
  pub fn new() -> Self { Self { lvl: 0 } }

  fn child(&self) -> Self { Self { lvl: self.lvl + 1 } }

  fn indent(&self) -> String {
    let mut ret = String::new();

    for _ in 0..self.lvl {
      ret.push_str("  ");
    }

    ret
  }
}

impl Tracer for AuTracer {
  type SolveAppHandle = AuTracer;
  type SolveClauseHandle = AuTracer;

  fn begin_solve_app(&self, app: &App) -> Self::SolveAppHandle {
    println!("{}\x1b[1;38;5;8mSolveApp\x1b[m {}", self.indent(), app);

    self.child()
  }

  fn begin_solve_clause(&self, clause: &Clause) -> Self::SolveClauseHandle {
    println!(
      "{}\x1b[1;38;5;8mSolveClause\x1b[m {}",
      self.indent(),
      clause
    );

    self.child()
  }
}

impl SolveAppHandle for AuTracer {
  type UnifyHandle = AuTracer;

  fn drop_key(&self) {
    println!("{}\x1b[1;38;5;1mDrop\x1b[m", self.indent());
  }

  fn begin_unify(&self, lhs: &App, rhs: &App) -> Self::UnifyHandle {
    println!(
      "{}\x1b[1;38;5;8mUnifyApp\x1b[m {} <> {}",
      self.indent(),
      lhs,
      rhs
    );

    self.child()
  }
}

impl SolveAppUnifyHandle for AuTracer {
  fn ok(&self, rhs: &Clause, sub: &Sub) {
    println!(
      "{}\x1b[1;38;5;2mOk\x1b[m {}, \x1b[1;38;5;3mneed\x1b[m {}",
      self.indent(),
      sub,
      rhs
    );
  }

  fn err(&self, err: AError) {
    println!("{}\x1b[1;38;5;1mErr\x1b[m {}", self.indent(), err);
  }
}

impl SolveClauseHandle for AuTracer {}
