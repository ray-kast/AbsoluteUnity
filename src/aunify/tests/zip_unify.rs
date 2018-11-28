use super::helpers::{value::*, var::*};
use crate::{Result, Sub};

pub fn test_equal_len<F: Fn(Vec<Value>, Vec<Value>) -> Result<Sub>>(f: F) {
  assert_eq!(
    f(
      vec![formalv("x"), formalv("x")],
      vec![formalv("y"), formalv("y")]
    )
    .unwrap(),
    Sub::top().with(formal("x"), formalv("y")).unwrap()
  );

  assert_eq!(
    f(
      vec![formalv("x"), atomv("a")],
      vec![formalv("y"), formalv("a")]
    )
    .unwrap(),
    Sub::top()
      .with(formal("x"), formalv("y"))
      .unwrap()
      .with(formal("a"), atomv("a"))
      .unwrap()
  );

  assert!(f(
    vec![formalv("x"), formalv("x")],
    vec![atomv("a"), atomv("b")]
  )
  .is_err());

  assert_eq!(
    f(
      vec![formalv("x"), formalv("x")],
      vec![formalv("x"), formalv("y")]
    )
    .unwrap(),
    Sub::top().with(formal("x"), formalv("y")).unwrap()
  );
}

pub fn test_noneq_len<F: Fn(Vec<Value>, Vec<Value>) -> Result<Sub>>(f: F) {
  assert!(f(vec![formalv("x")], vec![formalv("y"), atomv("a")]).is_err());
}
