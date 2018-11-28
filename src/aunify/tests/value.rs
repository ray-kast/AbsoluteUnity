use super::{
  helpers::{misc::*, value::*, var::*},
  zip_unify,
};
use crate::{Sub, Thing, Unify};

#[test]
fn free_vars() {
  assert_eq!(formalvar("x").free_vars(), hset(vec![formal("x")]));

  assert_eq!(atom("a").free_vars(), hset(vec![]));

  assert_eq!(
    tuple(vec![formalvar("x"), formalvar("y")]).free_vars(),
    hset(vec![formal("x"), formal("y")])
  )
}

#[test]
fn sub() {
  let sub = Sub::top()
    .with(formal("x"), formalvar("y"))
    .unwrap()
    .with(formal("z"), formalvar("w"))
    .unwrap();

  assert_eq!(formalvar("x").sub(&sub).unwrap(), formalvar("y"));

  assert_eq!(formalvar("y").sub(&sub).unwrap(), formalvar("y"));

  assert_eq!(atom("x").sub(&sub).unwrap(), atom("x"));

  assert_eq!(
    tuple(vec![formalvar("x"), atom("x"), formalvar("z")])
      .sub(&sub)
      .unwrap(),
    tuple(vec![formalvar("y"), atom("x"), formalvar("w")])
  );
}

#[test]
fn unify() {
  assert_eq!(formalvar("x").unify(&formalvar("x")).unwrap(), Sub::top());

  assert_eq!(
    formalvar("x").unify(&formalvar("y")).unwrap(),
    Sub::top().with(formal("x"), formalvar("y")).unwrap()
  );

  assert_eq!(
    formalvar("x").unify(&atom("a")).unwrap(),
    Sub::top().with(formal("x"), atom("a")).unwrap()
  );

  assert!(formalvar("x")
    .unify(&tuple(vec![formalvar("x"), atom("a")]))
    .is_err());

  assert_eq!(
    atom("a").unify(&formalvar("y")).unwrap(),
    Sub::top().with(formal("y"), atom("a")).unwrap()
  );

  assert!(tuple(vec![formalvar("y"), atom("a")])
    .unify(&formalvar("y"))
    .is_err());

  assert_eq!(atom("a").unify(&atom("a")).unwrap(), Sub::top());

  zip_unify::test_equal_len(|a, b| tuple(a).unify(&tuple(b)));
  zip_unify::test_noneq_len(|a, b| tuple(a).unify(&tuple(b)));

  assert!(atom("a").unify(&atom("b")).is_err());
}
