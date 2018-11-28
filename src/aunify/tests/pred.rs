mod app {
  use super::super::{helpers::value::*, zip_unify};
  use crate::{App, Pred, Tuple, Unify};

  #[test]
  fn unify() {
    zip_unify::test_equal_len(|a, b| {
      let pred = Pred::new_rc("test".into(), a.len());

      App::new(pred.clone(), Tuple(a)).unify(&App::new(pred, Tuple(b)))
    });

    zip_unify::test_noneq_len(|a, b| {
      App::new(Pred::new_rc("test".into(), a.len()), Tuple(a))
        .unify(&App::new(Pred::new_rc("test".into(), b.len()), Tuple(b)))
    });

    assert!(
      App::new(Pred::new_rc("a".into(), 1), Tuple(vec![atomv("a")]))
        .unify(&App::new(
          Pred::new_rc("b".into(), 1),
          Tuple(vec![atomv("a")])
        ))
        .is_err()
    );
  }
}
