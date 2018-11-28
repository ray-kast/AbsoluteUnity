use super::prelude::{gen_iter::GenIter, *};
use std::{cell::RefCell, rc::Rc};

pub struct Env(Vec<MaybeScheme<Statement>>);

pub trait IntoTrace {
  fn into_trace(self) -> Self;
}

impl Env {
  pub fn new() -> Self { Env(Vec::new()) }

  pub fn premises(&self) -> &Vec<MaybeScheme<Statement>> { &self.0 }

  pub fn state(&mut self, stmt: MaybeScheme<Statement>) { self.0.push(stmt); }

  // TODO: trace may not be the right type
  fn solve_app_impl<'a>(
    &'a self,
    app: App,
    src: &'a mut VarSource,
    trace: Rc<RefCell<HashSet<App>>>,
  ) -> impl Iterator<Item = Sub> + 'a {
    GenIter(move || {
      let key = app.clone().into_trace();

      if !trace.borrow_mut().insert(key.clone()) {
        // println!("dropping {}", app);
        return;
      }

      // TODO: clean this up, this is very messy
      for stmt in &self.0 {
        match stmt.as_inst(src).and_then(|stmt| {
          stmt
            .lhs()
            .unify(&app)
            .and_then(|sub| stmt.rhs().clone().sub(&sub).map(|rhs| (rhs, sub)))
        }) {
          Ok((rhs, sub)) => {
            // Box the iterator to avoid type recursion
            for sub2 in
              Box::new(self.solve_clause_impl(rhs, src, trace.clone()))
            {
              if let Ok(ret) = sub.clone().sub(&sub2) {
                let ret = ret.relevant_to(&app);

                if app.can_sub(&ret) {
                  yield ret;
                }
              }
            }
          },
          // Err(e) => println!("WARN: {}", e),
          Err(_) => {},
        }
      }

      trace.borrow_mut().remove(&key);
    })
  }

  pub fn solve_app<'a>(
    &'a self,
    app: App,
    src: &'a mut VarSource,
  ) -> impl Iterator<Item = Sub> + 'a {
    self.solve_app_impl(app, src, Rc::new(RefCell::new(HashSet::new())))
  }

  fn solve_clause_impl<'a>(
    &'a self,
    clause: Clause,
    src: &'a mut VarSource,
    trace: Rc<RefCell<HashSet<App>>>,
  ) -> impl Iterator<Item = Sub> + 'a {
    GenIter(move || match clause {
      Clause::Top => yield Sub::top(),
      Clause::Bot => {},
      Clause::App(a) => {
        for sol in self.solve_app_impl(a, src, trace.clone()) {
          yield sol;
        }
      },
      Clause::Not(c) => unimplemented!(),
      Clause::And(a, b) => {
        // TODO: should we really collect this?
        //       (NB: currently doing it to avoid double-mut-borrowing src)
        for sub in self
          .solve_clause_impl(*a, src, trace.clone())
          .collect::<Vec<_>>()
        {
          // TODO: this is gonna result in a lot of cloning...
          match b.clone().sub(&sub) {
            Ok(b) => {
              for sub2 in
                Box::new(self.solve_clause_impl(b, src, trace.clone()))
              {
                match sub.clone().sub(&sub2) {
                  Ok(s) => yield s,
                  Err(_) => {},
                }
              }
            },
            Err(_) => {},
          }
        }
      },
      Clause::Or(a, b) => {
        for sol in Box::new(self.solve_clause_impl(*a, src, trace.clone())) {
          yield sol;
        }

        for sol in Box::new(self.solve_clause_impl(*b, src, trace.clone())) {
          yield sol;
        }
      },
    })
  }

  pub fn solve_clause<'a>(
    &'a self,
    clause: Clause,
    src: &'a mut VarSource,
  ) -> impl Iterator<Item = Sub> + 'a {
    self.solve_clause_impl(clause, src, Rc::new(RefCell::new(HashSet::new())))
  }
}
