use super::helpers::{value::*, var::*};
use crate::{Result, Sub};

pub fn test_equal_len<F: Fn(Vec<Value>, Vec<Value>) -> Result<Sub>>(f: F) {
  assert_eq!(
    f(
      vec![formalvar("x"), formalvar("x")],
      vec![formalvar("y"), formalvar("y")]
    )
    .unwrap(),
    Sub::top().with(formal("x"), formalvar("y")).unwrap()
  );

  assert_eq!(
    f(
      vec![formalvar("x"), atom("a")],
      vec![formalvar("y"), formalvar("a")]
    )
    .unwrap(),
    Sub::top()
      .with(formal("x"), formalvar("y"))
      .unwrap()
      .with(formal("a"), atom("a"))
      .unwrap()
  );

  assert!(f(
    vec![formalvar("x"), formalvar("x")],
    vec![atom("a"), atom("b")]
  )
  .is_err());

  assert_eq!(
    f(
      vec![formalvar("x"), formalvar("x")],
      vec![formalvar("x"), formalvar("y")]
    )
    .unwrap(),
    Sub::top().with(formal("x"), formalvar("y")).unwrap()
  );
}

pub fn test_noneq_len<F: Fn(Vec<Value>, Vec<Value>) -> Result<Sub>>(f: F) {
  assert!(f(vec![formalvar("x")], vec![formalvar("y"), atom("a")]).is_err());
}
