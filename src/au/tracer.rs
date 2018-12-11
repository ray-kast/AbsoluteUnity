use super::prelude::*;
use aunify::{
  tracer::prelude::*, App, Clause, Result as AResult, Sub, Thing, Unify,
};

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
      ret.push_str(": ");
    }

    ret
  }
}

impl Tracer for AuTracer {
  type SolveAppHandle = AuTracer;
  type SolveClauseHandle = AuTracer;
  type ThingTracer = AuTracer;
  type UnifyTracer = AuTracer;

  fn for_thing(&self) -> Self::ThingTracer { self.clone() }

  fn for_unify(&self) -> Self::UnifyTracer { self.clone() }

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

  fn pre_yield(&self, sub: &Sub) {
    println!("{}\x1b[1;38;5;2mYield\x1b[m {}", self.indent(), sub);
  }
}

impl SolveAppHandle for AuTracer {
  // TODO: this is missing the "need <rhs>" trace from SolveAppUnifyHandle

  fn drop_key(&self) {
    println!("{}\x1b[1;38;5;1mDrop\x1b[m", self.indent());
  }
}

impl SolveClauseHandle for AuTracer {}

impl ThingTracer for AuTracer {
  type SubHandle = AuTracer;

  fn begin_sub<T: Thing>(&self, thing: &T, sub: &Sub) -> Self::SubHandle {
    println!(
      "{}\x1b[1;38;5;8mSub\x1b[m {} ~~ {}",
      self.indent(),
      thing,
      sub
    );

    self.child()
  }
}

impl SubHandle for AuTracer {
  fn pre_return<T: Thing>(&self, result: &AResult<T>) {
    match result {
      Ok(t) => println!(
        "{}\x1b[1;38;5;8mSub\x1b[1;38;5;2mOk\x1b[m {}",
        self.indent(),
        t
      ),
      Err(e) => println!(
        "{}\x1b[1;38;5;8mSub\x1b[1;38;5;1mErr\x1b[m {}",
        self.indent(),
        e
      ),
    }
  }
}

impl UnifyTracer for AuTracer {
  type ThingTracer = AuTracer;
  type UnifyHandle = AuTracer;

  fn for_thing(&self) -> Self::ThingTracer { self.clone() }

  fn begin_unify<T: Unify>(&self, lhs: &T, rhs: &T) -> Self::UnifyHandle {
    println!(
      "{}\x1b[1;38;5;8mUnify\x1b[m {} <> {}",
      self.indent(),
      lhs,
      rhs
    );

    self.child()
  }
}

impl UnifyHandle for AuTracer {
  fn pre_return(&self, result: &AResult<Sub>) {
    match result {
      Ok(s) => println!(
        "{}\x1b[1;38;5;8mUnify\x1b[1;38;5;2mOk\x1b[m {}",
        self.indent(),
        s
      ),
      Err(e) => println!(
        "{}\x1b[1;38;5;8mUnify\x1b[1;38;5;1mErr\x1b[m {}",
        self.indent(),
        e
      ),
    }
  }
}
