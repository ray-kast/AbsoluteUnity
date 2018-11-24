use crate::{Sub, Value, Var};

#[test]
fn unify() {
  assert_eq!(
    Value::Var(Var("x".into())).unify(&Value::Var(Var("y".into()))).unwrap(),
    Sub::top().with(Var("x".into()), Value::Var(Var("y".into()))).unwrap()
  );

  assert_eq!(
    Value::Var(Var("x".into())).unify(&Value::Atom("a".into())).unwrap(),
    Sub::top().with(Var("x".into()), Value::Atom("a".into())).unwrap()
  );

  assert_eq!(
    Value::Atom("a".into()).unify(&Value::Var(Var("y".into()))).unwrap(),
    Sub::top().with(Var("y".into()), Value::Atom("a".into())).unwrap()
  );

  assert_eq!(
    Value::Var(Var("x".into())).unify(&Value::Var(Var("x".into()))).unwrap(),
    Sub::top()
  );

  assert_eq!(
    Value::Atom("a".into()).unify(&Value::Atom("a".into())).unwrap(),
    Sub::top()
  );

  assert!(Value::Atom("a".into()).unify(&Value::Atom("b".into())).is_err());
}
