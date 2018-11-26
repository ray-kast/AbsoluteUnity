use super::prelude::{gen_iter::GenIter, *};

pub struct Env(Vec<MaybeScheme<Statement>>);

impl Env {
  pub fn new() -> Self { Env(Vec::new()) }

  pub fn premises(&self) -> &Vec<MaybeScheme<Statement>> { &self.0 }

  pub fn state(&mut self, stmt: MaybeScheme<Statement>) { self.0.push(stmt); }

  pub fn solve_app<'a>(
    &'a self,
    app: App,
    src: &'a mut VarSource,
  ) -> impl Iterator<Item = Sub> + 'a {
    GenIter(move || {
      for stmt in &self.0 {
        match stmt
          .as_inst(src)
          .and_then(|stmt| stmt.lhs().unify(&app).map(|sub| (stmt, sub)))
        {
          Ok((stmt, sub)) => {
            // TODO: THIS IS VERY BAD, WE COULD GET TRAPPED IN A LOOP
            // Box the iterator to avoid type recursion
            for sub2 in Box::new(self.solve_clause(stmt.rhs().clone(), src)) {
              if let Ok(merged) = sub2.merge(sub.clone()) {
                yield merged;
              }
            }
          },
          Err(_) => return,
        }
      }
    })
  }

  // TODO: the item type must be Solution
  pub fn solve_clause<'a>(
    &'a self,
    clause: Clause,
    src: &'a mut VarSource,
  ) -> impl Iterator<Item = Sub> + 'a {
    GenIter(move || match clause {
      Clause::App(a) => {
        for sol in self.solve_app(a, src) {
          yield sol;
        }
      },
      Clause::Not(c) => unimplemented!(),
      Clause::Any(v) => unimplemented!(),
      Clause::All(v) => {
        if v.is_empty() {
          yield Sub::top();
        } else {
          unimplemented!()
        }
      },
    })
  }
}
