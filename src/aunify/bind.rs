use super::{prelude::*, tracer::prelude::*};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bind<T>(HashSet<Var>, T);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MaybeBind<T> {
  Bind(Bind<T>),
  Raw(T),
}

impl<T> Bind<T> {
  pub fn over(t: T, vars: HashSet<Var>) -> Self { Bind(vars, t) }

  pub fn new(t: T) -> Self { Bind(HashSet::new(), t) }

  pub fn bind(mut self, var: Var) -> (Self, bool) {
    let ret = self.0.insert(var);
    (self, ret)
  }

  pub fn val(&self) -> &T { &self.1 }

  pub fn into_val(self) -> T { self.1 }
}

impl<T: Thing> Thing for Bind<T> {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    self.1.collect_free_vars(set);
    set.retain(|v| !self.0.contains(v));
  }

  fn sub_impl<R: ThingTracer>(
    self,
    sub: &Sub,
    tracer: R::SubHandle,
  ) -> Result<Self> {
    let sub = sub.clone().relevant_to(&self);
    Ok(Bind(self.0, self.1.sub(&sub, tracer)?))
  }

  fn can_sub(&self, sub: &Sub) -> bool {
    let sub = sub.clone().relevant_to(self);
    self.1.can_sub(&sub)
  }
}

impl<T: Display> Display for Bind<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("for ")?;

    let mut first = true;

    for var in &self.0 {
      if first {
        first = false;
      } else {
        fmt.write_str(", ")?;
      }

      Display::fmt(var, fmt)?;
    }

    fmt.write_str(".")?;

    Display::fmt(&self.1, fmt)
  }
}

impl<T: Thing> MaybeBind<T> {
  pub fn over(raw: T, mut vars: HashSet<Var>) -> Self {
    let free = raw.free_vars();

    vars.retain(|v| free.contains(v));

    if vars.is_empty() {
      MaybeBind::Raw(raw)
    } else {
      MaybeBind::Bind(Bind::over(raw, vars))
    }
  }

  pub fn val(&self) -> &T {
    match self {
      MaybeBind::Bind(b) => b.val(),
      MaybeBind::Raw(r) => &r,
    }
  }

  pub fn into_raw(self) -> T {
    match self {
      MaybeBind::Bind(b) => b.into_val(),
      MaybeBind::Raw(r) => r,
    }
  }
}

impl<T: Thing> Thing for MaybeBind<T> {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    match self {
      MaybeBind::Bind(b) => b.collect_free_vars(set),
      MaybeBind::Raw(r) => r.collect_free_vars(set),
    }
  }

  fn sub_impl<R: ThingTracer>(
    self,
    sub: &Sub,
    tracer: R::SubHandle,
  ) -> Result<Self> {
    use self::MaybeBind::*;

    Ok(match self {
      Bind(b) => Bind(b.sub(sub, tracer)?),
      Raw(r) => Raw(r.sub(sub, tracer)?),
    })
  }

  fn can_sub(&self, sub: &Sub) -> bool {
    match self {
      MaybeBind::Bind(b) => b.can_sub(sub),
      MaybeBind::Raw(r) => r.can_sub(sub),
    }
  }
}

impl<T: Display> Display for MaybeBind<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MaybeBind::Bind(b) => Display::fmt(b, fmt),
      MaybeBind::Raw(r) => Display::fmt(r, fmt),
    }
  }
}
