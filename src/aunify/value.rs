use super::prelude::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Value {
  Var(Var),
  Atom(String), // TODO: add some kind of configurable domain of discourse
}

impl Thing for Value {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    match self {
      Value::Var(v) => {
        set.insert(v.clone());
      },
      Value::Atom(_) => {},
    }
  }

  fn sub(self, sub: &Sub) -> Self {
    use self::Value::*;

    match &self {
      Var(v) => sub.get(&v).map_or(self, |l| l.clone()),
      _ => self.clone(),
    }
  }
}

impl Unify for Value {
  fn unify(&self, rhs: &Value) -> Result<Sub> {
    use self::Value::*;

    match (self, rhs) {
      (Var(a), Var(b)) if a == b => Ok(Sub::top()),
      (Var(a), b) => Sub::top().with(a.clone(), b.clone()),
      (a, Var(b)) => Sub::top().with(b.clone(), a.clone()),
      (Atom(a), Atom(b)) if a == b => Ok(Sub::top()),
      _ => Err(ErrorKind::BadValueUnify(self.clone(), rhs.clone()).into()),
    }
  }
}

impl Display for Value {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Value::Var(v) => Display::fmt(v, fmt)?,
      Value::Atom(a) => {
        fmt.write_str("｢")?;
        Display::fmt(a, fmt)?;
        fmt.write_str("｣")?;
      },
    }

    Ok(())
  }
}
