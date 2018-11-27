use super::prelude::{gen_iter::GenIter, *};
use std::{cell::RefCell, rc::Rc};

pub struct Env(Vec<MaybeScheme<Statement>>);

#[derive(Clone, Hash, PartialEq, Eq)]
struct CacheKey(RcPred, Vec<CkVal>);

#[derive(Clone, Hash, PartialEq, Eq)]
enum CkVal {
  AutoVar,
  FormalVar(String),
  Atom(String),
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
    trace: Rc<RefCell<HashSet<CacheKey>>>,
  ) -> impl Iterator<Item = Sub> + 'a {
    GenIter(move || {
      let key = {
        let (pred, vals) = app.clone().into_parts();

        CacheKey(
          pred,
          vals
            .into_iter()
            .map(|v| match v {
              Value::Var(Var::Auto(_)) => CkVal::AutoVar,
              Value::Var(Var::Formal(v)) => CkVal::FormalVar(v),
              Value::Atom(a) => CkVal::Atom(a),
            })
            .collect(),
        )
      };

      if !trace.borrow_mut().insert(key.clone()) {
        // println!("dropping {}", app);
        return;
      }

      for (i, stmt) in self.0.iter().enumerate() {
        match stmt
          .as_inst(src)
          .and_then(|stmt| stmt.lhs().unify(&app).map(|sub| (stmt, sub)))
        {
          Ok((stmt, sub)) => {
            // println!("{} <> {} under {}", stmt.lhs(), app, sub);

            // TODO: THIS IS VERY BAD, WE COULD GET TRAPPED IN A LOOP
            // Box the iterator to avoid type recursion
            for sub2 in Box::new(self.solve_clause_impl(
              stmt.rhs().clone().sub(&sub),
              src,
              trace.clone(),
            )) {
              if let Ok(merged) = sub2.merge(sub.clone()) {
                yield merged.relevant_to(&app);
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
    trace: Rc<RefCell<HashSet<CacheKey>>>,
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
        // println!("evaluating lhs of ({}), ({})", a, b);

        // TODO: should we really collect this?
        //       (NB: currently doing it to avoid double-mut-borrowing src)
        for sub in self
          .solve_clause_impl(*a, src, trace.clone())
          .collect::<Vec<_>>()
        {
          // println!("evaluating rhs {} under {}", b, sub);

          // TODO: this is gonna result in a lot of cloning...
          for sub2 in Box::new(self.solve_clause_impl(
            b.clone().sub(&sub),
            src,
            trace.clone(),
          )) {
            yield sub2;
          }
        }
      },
      Clause::Or(a, b) => unimplemented!(),
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
