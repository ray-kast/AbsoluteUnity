use super::prelude::*;

pub mod prelude {
  pub use super::{
    SolveAppHandle, SolveClauseHandle, SubHandle, ThingTracer, Tracer,
    UnifyHandle, UnifyTracer,
  };
}

// I'm already Tracer.
pub trait Tracer: Clone {
  // type ThingTracer<T>: ThingTracer<T>;
  type ThingTracer: ThingTracer;
  // type UnifyTracer<T>: UnifyTracer<T>;
  type UnifyTracer: UnifyTracer;
  type SolveAppHandle: SolveAppHandle;
  type SolveClauseHandle: SolveClauseHandle;

  fn for_thing(&self) -> Self::ThingTracer;

  fn for_unify(&self) -> Self::UnifyTracer;

  fn begin_solve_app(&self, app: &App) -> Self::SolveAppHandle;

  fn begin_solve_clause(&self, clause: &Clause) -> Self::SolveClauseHandle;

  fn pre_yield(&self, sub: &Sub);
}

pub trait SolveAppHandle: Tracer {
  fn drop_key(&self);
}

pub trait SolveClauseHandle: Tracer {}

// TODO: use this once generic associated types are more stable
// pub trait ThingTracer<T: Thing>: Clone {
pub trait ThingTracer: Clone {
  // type SubHandle: SubHandle<T>;
  type SubHandle: SubHandle;

  // fn begin_sub(&self, thing: &T, sub: &Sub) -> Self::SubHandle;
  fn begin_sub<T: Thing>(&self, thing: &T, sub: &Sub) -> Self::SubHandle;
}

// pub trait UnifyTracer<T: Unify>: Clone {
pub trait UnifyTracer: Clone {
  // type ThingTracer<T>: ThingTracer<T>;
  type ThingTracer: ThingTracer;
  // type UnifyHandle: UnifyHandle<T>;
  type UnifyHandle: UnifyHandle;

  fn for_thing(&self) -> Self::ThingTracer;

  // fn begin_unify(&self, thing: &T, rhs: &T) -> Self::SubHandle;
  fn begin_unify<T: Unify>(&self, thing: &T, rhs: &T) -> Self::UnifyHandle;
}

// TODO: see above
// pub trait SubHandle<T: Thing>: ThingTracer<T> {}
pub trait SubHandle: ThingTracer {
  fn pre_return<T: Thing>(&self, result: &Result<T>);
}

// pub trait UnifyHandle<T: Thing>: ThingTracer<T> {}
pub trait UnifyHandle: UnifyTracer {
  fn pre_return(&self, result: &Result<Sub>);
}

#[derive(Clone)]
pub struct NilTracer;

impl Tracer for NilTracer {
  type SolveAppHandle = NilTracer;
  type SolveClauseHandle = NilTracer;
  type ThingTracer = NilTracer;
  type UnifyTracer = NilTracer;

  fn for_thing(&self) -> Self::ThingTracer { NilTracer }

  fn for_unify(&self) -> Self::UnifyTracer { NilTracer }

  fn begin_solve_app(&self, _: &App) -> Self::SolveAppHandle { NilTracer }

  fn begin_solve_clause(&self, _: &Clause) -> Self::SolveClauseHandle {
    NilTracer
  }

  fn pre_yield(&self, _: &Sub) {}
}

impl SolveAppHandle for NilTracer {
  fn drop_key(&self) {}
}

impl SolveClauseHandle for NilTracer {}

// impl<T: Thing> ThingTracer<T> for NilTracer {
impl ThingTracer for NilTracer {
  type SubHandle = NilTracer;

  // fn begin_sub(&self, _: &T, _: &Sub) -> Self::SubHandle { NilTracer }
  fn begin_sub<T: Thing>(&self, _: &T, _: &Sub) -> Self::SubHandle { NilTracer }
}

impl SubHandle for NilTracer {
  fn pre_return<T: Thing>(&self, _: &Result<T>) {}
}

impl UnifyTracer for NilTracer {
  type ThingTracer = NilTracer;
  type UnifyHandle = NilTracer;

  fn for_thing(&self) -> Self::ThingTracer { NilTracer }

  fn begin_unify<T: Unify>(&self, _: &T, _: &T) -> Self::UnifyHandle {
    NilTracer
  }
}

impl UnifyHandle for NilTracer {
  fn pre_return(&self, _: &Result<Sub>) {}
}
