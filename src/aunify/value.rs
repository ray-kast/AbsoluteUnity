use super::{prelude::*, tracer::prelude::*};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Value {
  Var(Var),
  Atom(String),
  Numeric(Numeric),
  Tuple(Tuple), // TODO: handle unification of 1-tuples with values
  List(Box<List>),
}

impl Thing for Value {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    match self {
      Value::Var(v) => {
        set.insert(v.clone());
      },
      Value::Atom(_) => {},
      Value::Numeric(n) => n.collect_free_vars(set),
      Value::Tuple(t) => t.collect_free_vars(set),
      Value::List(l) => l.collect_free_vars(set),
    }
  }

  fn sub_impl<T: ThingTracer>(
    self,
    sub: &Sub,
    tracer: T::SubHandle,
  ) -> Result<Self> {
    use self::Value::*;

    Ok(match self {
      Var(v) => sub.get(&v).map_or(Var(v), |l| l.clone()),
      Atom(a) => Atom(a),
      Numeric(n) => Numeric(n.sub(sub, tracer)?),
      Tuple(t) => Tuple(t.sub(sub, tracer)?),
      List(l) => List(Box::new(l.sub(sub, tracer)?)),
    })
  }

  fn can_sub(&self, sub: &Sub) -> bool {
    use self::Value::*;

    match self {
      Var(_) => true,
      Atom(_) => true,
      Numeric(n) => n.can_sub(sub),
      Tuple(t) => t.can_sub(sub),
      List(l) => l.can_sub(sub),
    }
  }
}

impl Unify for Value {
  fn unify_impl<T: UnifyTracer>(
    &self,
    rhs: &Value,
    tracer: T::UnifyHandle,
  ) -> Result<Sub> {
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
        // if b.free_vars().contains(a) {
        //   Err(ErrorKind::VarBothSides(a.clone()).into())
        // } else {
        Sub::top().with(a.clone(), b.clone())
        // }
      },
      (a, Var(b)) => {
        // if a.free_vars().contains(b) {
        //   Err(ErrorKind::VarBothSides(b.clone()).into())
        // } else {
        Sub::top().with(b.clone(), a.clone())
        // }
      },
      (Atom(a), Atom(b)) if a == b => Ok(Sub::top()),
      (Numeric(a), Numeric(b)) => a.unify(b, tracer),
      (Tuple(a), Tuple(b)) => a.unify(b, tracer),
      (List(a), List(b)) => a.unify(b, tracer),
      _ => Err(ErrorKind::BadValueUnify(self.clone(), rhs.clone()).into()),
    }
  }
}

impl Display for Value {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Value::Var(v) => Display::fmt(v, fmt),
      Value::Atom(a) => {
        fmt.write_str("｢")?;
        Display::fmt(a, fmt)?;
        fmt.write_str("｣")
      },
      Value::Numeric(n) => Display::fmt(n, fmt),
      Value::Tuple(t) => Display::fmt(t, fmt),
      Value::List(l) => Display::fmt(l, fmt),
    }
  }
}
