use super::prelude::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Value {
  Var(Var),
  Atom(String),
  Tuple(Tuple), // TODO: make tuples into their own type
  List(Box<List>),
}

impl Thing for Value {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    match self {
      Value::Var(v) => {
        set.insert(v.clone());
      },
      Value::Atom(_) => {},
      Value::Tuple(t) => t.collect_free_vars(set),
      Value::List(l) => l.collect_free_vars(set),
    }
  }

  fn sub(self, sub: &Sub) -> Result<Self> {
    use self::Value::*;

    Ok(match self {
      Var(v) => sub.get(&v).map_or(Var(v), |l| l.clone()),
      Atom(a) => Atom(a),
      Tuple(t) => Tuple(t.sub(sub)?),
      List(l) => List(Box::new(l.sub(sub)?)),
    })
  }
}

impl Unify for Value {
  fn unify(&self, rhs: &Value) -> Result<Sub> {
    use self::Value::*;

    match (self, rhs) {
      (Var(a), Var(b)) => {
        if a == b {
          Ok(Sub::top())
        } else {
          Sub::top().with(a.clone(), Var(b.clone()))
        }
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
      (Tuple(a), Tuple(b)) => a.unify(b),
      (List(a), List(b)) => a.unify(b),
      _ => Err(ErrorKind::BadValueUnify(self.clone(), rhs.clone()).into()),
    }
  }
}

impl IntoTrace for Value {
  fn into_trace(self) -> Self {
    use self::Value::*;

    match self {
      Var(v) => Var(v.into_trace()),
      Atom(a) => Atom(a),
      Tuple(t) => Tuple(t.into_trace()),
      List(l) => List(Box::new(l.into_trace())),
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
      Value::Tuple(t) => Display::fmt(t, fmt)?,
      Value::List(l) => Display::fmt(l, fmt)?,
    }

    Ok(())
  }
}
