use super::{
  helpers::{misc::*, value::*, var::*},
  zip_unify,
};
use crate::{Sub, Thing, Unify};

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

  assert_eq!(formalv("x").sub(&sub).unwrap(), formalv("y"));

  assert_eq!(formalv("y").sub(&sub).unwrap(), formalv("y"));

  assert_eq!(atomv("x").sub(&sub).unwrap(), atomv("x"));

  assert_eq!(
    tuplev(vec![formalv("x"), atomv("x"), formalv("z")])
      .sub(&sub)
      .unwrap(),
    tuplev(vec![formalv("y"), atomv("x"), formalv("w")])
  );
}

#[test]
fn unify() {
  assert_eq!(formalv("x").unify(&formalv("x")).unwrap(), Sub::top());

  assert_eq!(
    formalv("x").unify(&formalv("y")).unwrap(),
    Sub::top().with(formal("x"), formalv("y")).unwrap()
  );

  assert_eq!(
    formalv("x").unify(&atomv("a")).unwrap(),
    Sub::top().with(formal("x"), atomv("a")).unwrap()
  );

  assert!(formalv("x")
    .unify(&tuplev(vec![formalv("x"), atomv("a")]))
    .is_err());

  assert_eq!(
    atomv("a").unify(&formalv("y")).unwrap(),
    Sub::top().with(formal("y"), atomv("a")).unwrap()
  );

  assert!(tuplev(vec![formalv("y"), atomv("a")])
    .unify(&formalv("y"))
    .is_err());

  assert_eq!(atomv("a").unify(&atomv("a")).unwrap(), Sub::top());

  zip_unify::test_equal_len(|a, b| tuplev(a).unify(&tuplev(b)));
  zip_unify::test_noneq_len(|a, b| tuplev(a).unify(&tuplev(b)));

  assert!(atomv("a").unify(&atomv("b")).is_err());
}
