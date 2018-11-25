use crate::{Sub, Unify, Value, Var};

#[test]
fn unify() {
  assert_eq!(
    Value::Var(Var::Formal("x".into()))
      .unify(&Value::Var(Var::Formal("y".into())))
      .unwrap(),
    Sub::top()
      .with(Var::Formal("x".into()), Value::Var(Var::Formal("y".into())))
      .unwrap()
  );

  assert_eq!(
    Value::Var(Var::Formal("x".into()))
      .unify(&Value::Atom("a".into()))
      .unwrap(),
    Sub::top()
      .with(Var::Formal("x".into()), Value::Atom("a".into()))
      .unwrap()
  );

  assert_eq!(
    Value::Atom("a".into())
      .unify(&Value::Var(Var::Formal("y".into())))
      .unwrap(),
    Sub::top()
      .with(Var::Formal("y".into()), Value::Atom("a".into()))
      .unwrap()
  );

  assert_eq!(
    Value::Var(Var::Formal("x".into()))
      .unify(&Value::Var(Var::Formal("x".into())))
      .unwrap(),
    Sub::top()
  );

  assert_eq!(
    Value::Atom("a".into())
      .unify(&Value::Atom("a".into()))
      .unwrap(),
    Sub::top()
  );

  assert!(Value::Atom("a".into())
    .unify(&Value::Atom("b".into()))
    .is_err());
}
