use crate::{ast::ParserTag, parser::*};
use aunify::{App, Pred, Tuple, Value, Var};

#[test]
fn value() {
  let parser = ValueParser::new();
  let mut tag = ParserTag::new();

  assert_eq!(
    parser.parse(&mut tag, "x").unwrap(),
    Value::Atom("x".into())
  );

  assert_eq!(
    parser.parse(&mut tag, "X").unwrap(),
    Value::Var(Var::Formal("X".into()))
  );

  assert_eq!(
    parser.parse(&mut tag, "myAtom1").unwrap(),
    Value::Atom("myAtom1".into())
  );

  assert_eq!(
    parser.parse(&mut tag, "MyVar1").unwrap(),
    Value::Var(Var::Formal("MyVar1".into()))
  );

  // Might add numerics eventually, but for now let's keep it strictly symbolic
  assert_eq!(
    parser.parse(&mut tag, "1337").unwrap(),
    Value::Atom("1337".into())
  );
}

#[test]
fn app() {
  let parser = AppParser::new();
  let mut tag = ParserTag::new();

  assert_eq!(
    parser.parse(&mut tag, "eq(X, X)").unwrap(),
    App::new(
      Pred::new_rc("eq".into(), 2),
      Tuple(vec![
        Value::Var(Var::Formal("X".into())),
        Value::Var(Var::Formal("X".into()))
      ])
    )
  );

  assert_eq!(
    parser.parse(&mut tag, "eq(X, X,)").unwrap(),
    App::new(
      Pred::new_rc("eq".into(), 2),
      Tuple(vec![
        Value::Var(Var::Formal("X".into())),
        Value::Var(Var::Formal("X".into()))
      ])
    )
  );

  assert_eq!(
    parser.parse(&mut tag, "eq(X, atom)").unwrap(),
    App::new(
      Pred::new_rc("eq".into(), 2),
      Tuple(vec![
        Value::Var(Var::Formal("X".into())),
        Value::Atom("atom".into())
      ])
    )
  );

  assert_eq!(
    parser.parse(&mut tag, "hello(world)").unwrap(),
    App::new(
      Pred::new_rc("hello".into(), 1),
      Tuple(vec![Value::Atom("world".into())])
    )
  );

  // TODO: is there any point to allowing null-order predicates?
  assert_eq!(
    parser.parse(&mut tag, "boi()").unwrap(),
    App::new(Pred::new_rc("boi".into(), 0), Tuple(vec![]))
  );

  // This fails because predicate names must be valid atoms
  assert!(parser.parse(&mut tag, "Cat(dog)").is_err());
}

// TODO: add tests for Clause, Statement, and Expr
