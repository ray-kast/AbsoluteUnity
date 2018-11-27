use super::prelude::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Value {
  Var(Var),
  Atom(String), // TODO: add some kind of configurable domain of discourse
  Tuple(Vec<Value>),
}

impl Thing for Value {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    match self {
      Value::Var(v) => {
        set.insert(v.clone());
      },
      Value::Atom(_) => {},
      Value::Tuple(v) => {
        for val in v {
          val.collect_free_vars(set);
        }
      },
    }
  }

  fn sub(self, sub: &Sub) -> Self {
    use self::Value::*;

    match self {
      Var(v) => sub.get(&v).map_or(Var(v), |l| l.clone()),
      Atom(a) => Atom(a),
      Tuple(v) => Tuple(v.into_iter().map(|l| l.sub(sub)).collect())
    }
  }
}

impl Unify for Value {
  fn unify(&self, rhs: &Value) -> Result<Sub> {
    use self::Value::*;

    match (self, rhs) {
      (Var(a), Var(b)) => if a == b {
        Ok(Sub::top())
      } else {
        Sub::top().with(a.clone(), Var(b.clone()))
      },
      (Var(a), b) => {
        if b.free_vars().contains(a) {
          Err(ErrorKind::VarBothSides(a.clone()).into())
        } else {
          Sub::top().with(a.clone(), b.clone())
        }
      },
      (a, Var(b)) => {
        if a.free_vars().contains(b) {
          Err(ErrorKind::VarBothSides(b.clone()).into())
        } else {
          Sub::top().with(b.clone(), a.clone())
        }
      },
      (Atom(a), Atom(b)) if a == b => Ok(Sub::top()),
      (Tuple(a), Tuple(b)) if a.len() == b.len() => {
        let mut ret = Sub::top();

        for (l, r) in a.iter().zip(b.iter()) {
          let sub = &l.clone().sub(&ret).unify(r)?;
          ret = ret.sub(sub);
        }

        Ok(ret)
      },
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
      Value::Tuple(v) => {
        fmt.write_str("(")?;

        let mut first = true;

        for val in v {
          if first {
            first = false;
          } else {
            fmt.write_str(", ")?;
          }

          Display::fmt(val, fmt)?;
        }

        fmt.write_str(")")?;
      },
    }

    Ok(())
  }
}
