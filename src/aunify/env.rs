use super::{gen_iter::GenIter, prelude::*, tracer::prelude::*};
use std::{cell::RefCell, rc::Rc};

// TODO: this could be accelerated by collecting statements on their LHS predicate
pub struct Env(Vec<MaybeScheme<Statement>>);

pub trait IntoTrace {
  fn into_trace(self) -> Self;
}

impl Env {
  pub fn new() -> Self { Env(Vec::new()) }

  pub fn premises(&self) -> &Vec<MaybeScheme<Statement>> { &self.0 }

  pub fn state(&mut self, stmt: MaybeScheme<Statement>) { self.0.push(stmt); }

  // TODO: trace may not be the right type
  fn solve_app_impl<'a, T: Tracer + 'a>(
    &'a self,
    app: App,
    src: &'a VarSource,
    trace: Rc<RefCell<HashSet<App>>>,
    tracer: T,
  ) -> impl Iterator<Item = Sub> + 'a {
    GenIter(move || {
      let tracer = tracer.begin_solve_app(&app);

      let key = app.clone().into_trace();

      if !trace.borrow_mut().insert(key.clone()) {
        tracer.drop_key();
        return;
      }

      // TODO: clean this up, this is very messy
      // TODO: short-circuit this
      for stmt in &self.0 {
        // TODO: this is a dumb optimization and may cause problems later
        if match stmt {
          MaybeScheme::Scheme(s) => s.val().lhs(),
          MaybeScheme::Inst(i) => i.lhs(),
        }
        .pred()
          != app.pred()
        {
          continue;
        }

        match stmt.as_inst(src) {
          Ok(stmt) => {
            let unify_tracer = tracer.begin_unify(stmt.lhs(), &app);

            match stmt.lhs().unify(&app).and_then(|sub| {
              stmt.rhs().clone().sub(&sub).map(|rhs| (rhs, sub))
            }) {
              Ok((rhs, sub)) => {
                unify_tracer.ok(&rhs, &sub);

                // Box the iterator to avoid type recursion
                for sub2 in Box::new(self.solve_clause_impl(
                  rhs,
                  src,
                  trace.clone(),
                  unify_tracer.clone(),
                )) {
                  if let Ok(ret) = sub.clone().sub(&sub2) {
                    let ret = ret.relevant_to(&app);

                    if app.can_sub(&ret) {
                      yield ret;
                    }
                  }
                }
              },
              Err(e) => unify_tracer.err(e),
            }
          },
          Err(_) => {},
        }
      }

      trace.borrow_mut().remove(&key);
    })
  }

  pub fn solve_app<'a, T: Tracer + 'a>(
    &'a self,
    app: App,
    src: &'a VarSource,
    tracer: T,
  ) -> impl Iterator<Item = Sub> + 'a {
    self.solve_app_impl(app, src, Rc::new(RefCell::new(HashSet::new())), tracer)
  }

  fn solve_clause_impl<'a, T: Tracer + 'a>(
    &'a self,
    clause: Clause,
    src: &'a VarSource,
    trace: Rc<RefCell<HashSet<App>>>,
    tracer: T,
  ) -> impl Iterator<Item = Sub> + 'a {
    // TODO: implement short-circuiting for And and Or?

    GenIter(move || {
      let tracer = tracer.begin_solve_clause(&clause);

      match clause {
        Clause::Top => yield Sub::top(),
        Clause::Bot => {},
        Clause::App(a) => {
          for sol in self.solve_app_impl(a, src, trace.clone(), tracer) {
            yield sol;
          }
        },
        Clause::Not(c) => {
          // TODO: maybe have some kind of non-constructive constraint system?

          match Box::new(self.solve_clause_impl(*c, src, trace.clone(), tracer))
            .next()
          {
            Some(_) => {},
            None => yield Sub::top(),
          }
        },
        Clause::And(a, b) => {
          for sub in Box::new(self.solve_clause_impl(
            *a,
            src,
            trace.clone(),
            tracer.clone(),
          )) {
            // TODO: this is gonna result in a lot of cloning...
            match b.clone().sub(&sub) {
              Ok(b) => {
                for sub2 in Box::new(self.solve_clause_impl(
                  b,
                  src,
                  trace.clone(),
                  tracer.clone(),
                )) {
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
          for sol in Box::new(self.solve_clause_impl(
            *a,
            src,
            trace.clone(),
            tracer.clone(),
          )) {
            yield sol;
          }

          for sol in
            Box::new(self.solve_clause_impl(*b, src, trace.clone(), tracer))
          {
            yield sol;
          }
        },
      }
    })
  }

  pub fn solve_clause<'a, T: Tracer + 'a>(
    &'a self,
    clause: Clause,
    src: &'a VarSource,
    tracer: T,
  ) -> impl Iterator<Item = Sub> + 'a {
    self.solve_clause_impl(
      clause,
      src,
      Rc::new(RefCell::new(HashSet::new())),
      tracer,
    )
  }
}
