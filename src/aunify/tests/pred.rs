mod app {
  use crate::{App, Pred, Sub, Value, Var};
  use std::rc::Rc;

  fn pred_eq() -> Rc<Pred> { Pred::new_rc("eq".into(), 2) }

  #[test]
  fn unify() {
    let pred_eq = pred_eq();

    assert_eq!(
      App::new(
        pred_eq.clone(),
        vec![Value::Var(Var("x".into())), Value::Var(Var("x".into()))]
      )
      .unify(&App::new(
        pred_eq.clone(),
        vec![Value::Var(Var("y".into())), Value::Var(Var("y".into()))]
      ))
      .unwrap(),
      Sub::top()
        .with(Var("x".into()), Value::Var(Var("y".into())))
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
      vec![Value::Var(Var("x".into())), Value::Var(Var("x".into()))]
    )
    .unify(&App::new(
      pred_eq.clone(),
      vec![Value::Atom("a".into()), Value::Atom("b".into())]
    ))
    .is_err());
  }

  // #[test]
  // fn unify_bad() {
  //   let pred_eq = pred_eq();

  //   // TODO: this should fail because x is on both sides
  //   assert!(App::new(
  //     pred_eq.clone(),
  //     vec![Value::Var(Var("x".into())), Value::Var(Var("x".into()))]
  //   )
  //   .unify(&App::new(
  //     pred_eq.clone(),
  //     vec![Value::Var(Var("x".into())), Value::Var(Var("y".into()))]
  //   ))
  //   .is_err());
  // }
}