mod app {
  use super::super::{helpers::value::*, zip_unify};
  use crate::{App, Pred, Unify};

  #[test]
  fn unify() {
    zip_unify::test_equal_len(|a, b| {
      let pred = Pred::new_rc("test".into(), a.len());

      App::new(pred.clone(), a).unify(&App::new(pred, b))
    });

    zip_unify::test_noneq_len(|a, b| {
      App::new(Pred::new_rc("test".into(), a.len()), a)
        .unify(&App::new(Pred::new_rc("test".into(), b.len()), b))
    });

    assert!(App::new(Pred::new_rc("a".into(), 1), vec![atom("a")])
      .unify(&App::new(Pred::new_rc("b".into(), 1), vec![atom("a")]))
      .is_err());
  }
}
