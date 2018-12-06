use super::prelude::*;

pub mod prelude {
  pub use super::{
    SolveAppHandle, SolveAppUnifyHandle, SolveClauseHandle, Tracer,
  };
}

// I'm already Tracer.
pub trait Tracer: Clone {
  type SolveAppHandle: SolveAppHandle;
  type SolveClauseHandle: SolveClauseHandle;

  fn begin_solve_app(&self, app: &App) -> Self::SolveAppHandle;

  fn begin_solve_clause(&self, clause: &Clause) -> Self::SolveClauseHandle;
}

pub trait SolveAppHandle: Tracer {
  type UnifyHandle: SolveAppUnifyHandle;

  fn drop_key(&self);

  fn begin_unify(&self, lhs: &App, rhs: &App) -> Self::UnifyHandle;
}

pub trait SolveAppUnifyHandle: Tracer {
  fn ok(&self, rhs: &Clause, sub: &Sub);

  fn err(&self, err: Error);
}

pub trait SolveClauseHandle: Tracer {}

#[derive(Clone)]
pub struct NilTracer;

impl Tracer for NilTracer {
  type SolveAppHandle = NilTracer;
  type SolveClauseHandle = NilTracer;

  fn begin_solve_app(&self, _: &App) -> Self::SolveAppHandle { NilTracer }

  fn begin_solve_clause(&self, _: &Clause) -> Self::SolveClauseHandle {
    NilTracer
  }
}

impl SolveAppHandle for NilTracer {
  type UnifyHandle = NilTracer;

  fn drop_key(&self) {}

  fn begin_unify(&self, _: &App, _: &App) -> Self::UnifyHandle { NilTracer }
}

impl SolveAppUnifyHandle for NilTracer {
  fn ok(&self, _: &Clause, _: &Sub) {}

  fn err(&self, _: Error) {}
}

impl SolveClauseHandle for NilTracer {}
