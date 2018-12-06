use super::prelude::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum List {
  Cons(Value, Tail),
  Nil,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Tail {
  Open(Var),
  Close(Box<List>),
}

impl Thing for List {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    match self {
      List::Cons(h, t) => {
        h.collect_free_vars(set);
        t.collect_free_vars(set);
      },
      List::Nil => {},
    }
  }

  fn sub(self, sub: &Sub) -> Result<Self> {
    use self::List::*;

    Ok(match self {
      Cons(h, t) => Cons(h.sub(sub)?, t.sub(sub)?),
      Nil => Nil,
    })
  }

  fn can_sub(&self, sub: &Sub) -> bool {
    use self::List::*;

    match self {
      Cons(h, t) => h.can_sub(sub) && t.can_sub(sub),
      Nil => true,
    }
  }
}

impl Unify for List {
  fn unify(&self, rhs: &Self) -> Result<Sub> {
    use self::List::*;

    match (self, rhs) {
      (Cons(h1, t1), Cons(h2, t2)) => {
        let hsub = h1.unify(&h2)?;
        let tsub = t1.clone().sub(&hsub)?.unify(&t2.clone().sub(&hsub)?)?;

        hsub.sub(&tsub)
      },
      (Nil, Nil) => Ok(Sub::top()),
      _ => Err(ErrorKind::BadListUnify(self.clone(), rhs.clone()).into()),
    }
  }
}

impl Display for List {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      List::Cons(h, t) => {
        Display::fmt(h, fmt)?;
        fmt.write_str(":")?;
        Display::fmt(t, fmt)
      },
      List::Nil => fmt.write_str("[]"),
    }
  }
}

impl Thing for Tail {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    match self {
      Tail::Open(v) => {
        set.insert(v.clone());
      },
      Tail::Close(l) => l.collect_free_vars(set),
    }
  }

  fn sub(self, sub: &Sub) -> Result<Self> {
    use self::Tail::*;

    Ok(match self {
      Open(v) => match sub.get(&v).map_or(Value::Var(v), |v| v.clone()) {
        Value::Var(v) => Open(v),
        Value::List(l) => Close(Box::new(l.sub(sub)?)),
        v => return Err(ErrorKind::SubBadType("Tail", v).into()),
      },
      Close(l) => Close(Box::new(l.sub(sub)?)),
    })
  }

  fn can_sub(&self, sub: &Sub) -> bool {
    use self::Tail::*;

    match self {
      Open(v) => sub.get(v).map_or(true, |v| match v {
        Value::Var(_) => true,
        Value::List(_) => true,
        _ => false,
      }),
      Close(l) => l.can_sub(sub),
    }
  }
}

impl Unify for Tail {
  fn unify(&self, rhs: &Self) -> Result<Sub> {
    use self::Tail::*;

    match (self, rhs) {
      (Open(a), Open(b)) => {
        if a == b {
          Ok(Sub::top())
        } else {
          Sub::top().with(a.clone(), Value::Var(b.clone()))
        }
      },
      (Open(a), Close(b)) => {
        if b.free_vars().contains(a) {
          Err(ErrorKind::VarBothSides(a.clone()).into())
        } else {
          Sub::top().with(a.clone(), Value::List(b.clone()))
        }
      },
      (Close(a), Open(b)) => {
        if a.free_vars().contains(b) {
          Err(ErrorKind::VarBothSides(b.clone()).into())
        } else {
          Sub::top().with(b.clone(), Value::List(a.clone()))
        }
      },
      (Close(a), Close(b)) => a.unify(&b),
    }
  }
}

impl Display for Tail {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Tail::Open(v) => Display::fmt(v, fmt),
      Tail::Close(l) => Display::fmt(l, fmt),
    }
  }
}
