use super::{gen_iter::GenIter, prelude::*, tracer::prelude::*};

// TODO: this could be accelerated by collecting statements on their LHS predicate
pub struct Env(Vec<MaybeScheme<Statement>>);

impl Env {
  pub fn new() -> Self { Env(Vec::new()) }

  pub fn premises(&self) -> &Vec<MaybeScheme<Statement>> { &self.0 }

  pub fn state(&mut self, stmt: MaybeScheme<Statement>) { self.0.push(stmt); }

  // TODO: trace may not be the right type
  fn solve_app_impl<'a, T: Tracer + 'a>(
    &'a self,
    app: App,
    src: &'a VarSource,
    tracer: T,
  ) -> impl Iterator<Item = Sub> + 'a {
    GenIter(move || {
      let tracer = tracer.begin_solve_app(&app);

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

        match stmt.as_inst(src, tracer.for_thing()) {
          Ok(stmt) => {
            match stmt.lhs().unify(&app, tracer.for_unify()).and_then(|sub| {
              stmt
                .rhs()
                .clone()
                .sub(&sub, tracer.for_thing())
                .map(|rhs| (rhs, sub))
            }) {
              Ok((rhs, sub)) => {
                // Box the iterator to avoid type recursion
                for sub2 in Box::new(self.solve_clause_impl(
                  rhs,
                  src,
                  tracer.clone(),
                )) {
                  if let Ok(ret) = sub.clone().sub(&sub2, tracer.for_thing()) {
                    let ret = ret.relevant_to(&app);

                    if app.can_sub(&ret) {
                      tracer.pre_yield(&ret);
                      yield ret;
                    }
                  }
                }
              },
              Err(_) => {},
            }
          },
          Err(_) => {},
        }
      }
    })
  }

  pub fn solve_app<'a, T: Tracer + 'a>(
    &'a self,
    app: App,
    src: &'a VarSource,
    tracer: T,
  ) -> impl Iterator<Item = Sub> + 'a {
    self.solve_app_impl(app, src, tracer)
  }

  fn solve_clause_impl<'a, T: Tracer + 'a>(
    &'a self,
    clause: Clause,
    src: &'a VarSource,
    tracer: T,
  ) -> impl Iterator<Item = Sub> + 'a {
    // TODO: implement short-circuiting for And and Or?

    GenIter(move || {
      let tracer = tracer.begin_solve_clause(&clause);

      match clause {
        Clause::Top => {
          let ret = Sub::top();
          tracer.pre_yield(&ret);
          yield ret;
        },
        Clause::Bot => {},
        Clause::App(a) => {
          for sol in self.solve_app_impl(a, src, tracer.clone()) {
            tracer.pre_yield(&sol);
            yield sol;
          }
        },
        Clause::Not(c) => {
          // TODO: maybe have some kind of non-constructive constraint system?

          match Box::new(self.solve_clause_impl(*c, src, tracer.clone())).next()
          {
            Some(_) => {},
            None => {
              let ret = Sub::top();
              tracer.pre_yield(&ret);
              yield ret;
            },
          }
        },
        Clause::And(a, b) => {
          for sub in Box::new(self.solve_clause_impl(*a, src, tracer.clone())) {
            // TODO: this is gonna result in a lot of cloning...
            match b.clone().sub(&sub, tracer.for_thing()) {
              Ok(b) => {
                for sub2 in
                  Box::new(self.solve_clause_impl(b, src, tracer.clone()))
                {
                  match sub.clone().sub(&sub2, tracer.for_thing()) {
                    Ok(s) => {
                      tracer.pre_yield(&s);
                      yield s;
                    },
                    Err(_) => {},
                  }
                }
              },
              Err(_) => {},
            }
          }
        },
        Clause::Or(a, b) => {
          for sol in Box::new(self.solve_clause_impl(*a, src, tracer.clone())) {
            tracer.pre_yield(&sol);
            yield sol;
          }

          for sol in Box::new(self.solve_clause_impl(*b, src, tracer.clone())) {
            tracer.pre_yield(&sol);
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
    self.solve_clause_impl(clause, src, tracer)
  }
}
