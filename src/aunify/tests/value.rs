use super::{
  helpers::{misc::*, value::*, var::*},
  zip_unify,
};
use crate::{NilTracer, Sub, Thing, Unify};

#[test]
fn free_vars() {
  assert_eq!(formalv("x").free_vars(), hset(vec![formal("x")]));

  assert_eq!(atomv("a").free_vars(), hset(vec![]));

  assert_eq!(
    tuplev(vec![formalv("x"), formalv("y")]).free_vars(),
    hset(vec![formal("x"), formal("y")])
  )
}

#[test]
fn sub() {
  let sub = Sub::top()
    .with(formal("x"), formalv("y"))
    .unwrap()
    .with(formal("z"), formalv("w"))
    .unwrap();

  assert_eq!(formalv("x").sub(&sub, NilTracer).unwrap(), formalv("y"));

  assert_eq!(formalv("y").sub(&sub, NilTracer).unwrap(), formalv("y"));

  assert_eq!(atomv("x").sub(&sub, NilTracer).unwrap(), atomv("x"));

  assert_eq!(
    tuplev(vec![formalv("x"), atomv("x"), formalv("z")])
      .sub(&sub, NilTracer)
      .unwrap(),
    tuplev(vec![formalv("y"), atomv("x"), formalv("w")])
  );
}

#[test]
fn unify() {
  assert_eq!(
    formalv("x").unify(&formalv("x"), NilTracer).unwrap(),
    Sub::top()
  );

  assert_eq!(
    formalv("x").unify(&formalv("y"), NilTracer).unwrap(),
    Sub::top().with(formal("x"), formalv("y")).unwrap()
  );

  assert_eq!(
    formalv("x").unify(&atomv("a"), NilTracer).unwrap(),
    Sub::top().with(formal("x"), atomv("a")).unwrap()
  );

  // TODO: fix this if we're gonna allow infinite bindings
  // assert!(formalv("x")
  //   .unify(&tuplev(vec![formalv("x"), atomv("a")]), NilTracer)
  //   .is_err());

  assert_eq!(
    atomv("a").unify(&formalv("y"), NilTracer).unwrap(),
    Sub::top().with(formal("y"), atomv("a")).unwrap()
  );

  // TODO: fix this if we're gonna allow infinite bindings
  // assert!(tuplev(vec![formalv("y"), atomv("a")])
  //   .unify(&formalv("y"), NilTracer)
  //   .is_err());

  assert_eq!(
    atomv("a").unify(&atomv("a"), NilTracer).unwrap(),
    Sub::top()
  );

  zip_unify::test_equal_len(|a, b| tuplev(a).unify(&tuplev(b), NilTracer));
  zip_unify::test_noneq_len(|a, b| tuplev(a).unify(&tuplev(b), NilTracer));

  assert!(atomv("a").unify(&atomv("b"), NilTracer).is_err());
}
