mod app {
  use crate::{App, Pred, RcPred, Sub, Unify, Value, Var};

  fn pred_eq() -> RcPred { Pred::new_rc("eq".into(), 2) }

  #[test]
  fn unify() {
    let pred_eq = pred_eq();

    assert_eq!(
      App::new(
        pred_eq.clone(),
        vec![
          Value::Var(Var::Formal("x".into())),
          Value::Var(Var::Formal("x".into()))
        ]
      )
      .unify(&App::new(
        pred_eq.clone(),
        vec![
          Value::Var(Var::Formal("y".into())),
          Value::Var(Var::Formal("y".into()))
        ]
      ))
      .unwrap(),
      Sub::top()
        .with(Var::Formal("x".into()), Value::Var(Var::Formal("y".into())))
        .unwrap()
    );

    // assert_eq!(
    //   App::new(
    //     pred.clone(),
    //     vec![Value::Var(Var("x".into())), Value::Atom("a".into())]
    //   )
    //   .unify(&App::new(
    //     pred.clone(),
    //     vec![Value::Var(Var("y".into())), Value::Var(Var("a".into()))]
    //   )),
    //   Some(Sub::top())
    // );

    assert!(App::new(
      pred_eq.clone(),
      vec![
        Value::Var(Var::Formal("x".into())),
        Value::Var(Var::Formal("x".into()))
      ]
    )
    .unify(&App::new(
      pred_eq.clone(),
      vec![Value::Atom("a".into()), Value::Atom("b".into())]
    ))
    .is_err());
  }

  #[test]
  fn unify_bad() {
    let pred_eq = pred_eq();

    // This fails because x is on both sides
    assert!(App::new(
      pred_eq.clone(),
      vec![
        Value::Var(Var::Formal("x".into())),
        Value::Var(Var::Formal("x".into()))
      ]
    )
    .unify(&App::new(
      pred_eq.clone(),
      vec![
        Value::Var(Var::Formal("x".into())),
        Value::Var(Var::Formal("y".into()))
      ]
    ))
    .is_err());
  }
}
