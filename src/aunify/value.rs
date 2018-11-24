use super::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Value {
  Var(Var),
  Atom(String), // TODO: add some kind of configurable domain of discourse
}

impl Value {
  pub fn sub(self, sub: &Sub) -> Self {
    use self::Value::*;

    match &self {
      Var(v) => sub.get(&v).map_or(self, |l| l.clone()),
      _ => self.clone(),
    }
  }

  pub fn unify(&self, rhs: &Value) -> Result<Sub> {
    use self::Value::*;

    match (self, rhs) {
      (Var(a), Var(b)) if a == b => Ok(Sub::top()),
      (Var(a), b) => Sub::top().with(a.clone(), b.clone()),
      (a, Var(b)) => Sub::top().with(b.clone(), a.clone()),
      (Atom(a), Atom(b)) if a == b => Ok(Sub::top()),
      _ => Err(ErrorKind::BadValueUnify(
        format!("{:?}", self),
        format!("{:?}", rhs)
      ).into()),
    }
  }
}