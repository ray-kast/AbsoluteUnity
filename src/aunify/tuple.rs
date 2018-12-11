use super::{prelude::*, tracer::prelude::*};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Tuple(pub Vec<Value>);

impl Thing for Tuple {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    for val in &self.0 {
      val.collect_free_vars(set);
    }
  }

  fn sub_impl<T: ThingTracer>(
    self,
    sub: &Sub,
    tracer: T::SubHandle,
  ) -> Result<Self> {
    Ok(Tuple(
      self
        .0
        .into_iter()
        .map(|l| l.sub(sub, tracer.clone()))
        .collect::<Result<_>>()?,
    ))
  }

  fn can_sub(&self, sub: &Sub) -> bool { self.0.iter().all(|l| l.can_sub(sub)) }
}

impl Unify for Tuple {
  fn unify_impl<T: UnifyTracer>(
    &self,
    rhs: &Self,
    tracer: T::UnifyHandle,
  ) -> Result<Sub> {
    if self.0.len() == rhs.0.len() {
      let mut ret = Sub::top();

      for (l, r) in self.0.iter().zip(rhs.0.iter()) {
        let sub = &l
          .clone()
          .sub(&ret, tracer.for_thing())?
          .unify(&r.clone().sub(&ret, tracer.for_thing())?, tracer.clone())?;
        ret = ret.sub(sub, tracer.for_thing())?;
      }

      Ok(ret)
    } else {
      Err(ErrorKind::BadTupleUnify(self.clone(), rhs.clone()).into())
    }
  }
}

impl Display for Tuple {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(")?;

    let mut first = true;

    for val in &self.0 {
      if first {
        first = false;
      } else {
        fmt.write_str(", ")?;
      }

      Display::fmt(val, fmt)?;
    }

    fmt.write_str(")")
  }
}
