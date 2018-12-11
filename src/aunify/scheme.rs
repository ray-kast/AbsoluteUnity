use super::{prelude::*, tracer::prelude::*};
use std::{borrow::Cow, mem};

#[derive(Clone, Debug)]
pub struct Scheme<T>(HashSet<Var>, T);

#[derive(Clone, Debug)]
pub enum MaybeScheme<T> {
  Scheme(Scheme<T>),
  Inst(T),
}

impl<T> Scheme<T> {
  pub fn generalize(t: T, on: HashSet<Var>) -> Self { Scheme(on, t) }

  pub fn val(&self) -> &T { &self.1 }
}

impl<T: Thing> Scheme<T> {
  pub fn inst<R: ThingTracer>(self, src: &VarSource, tracer: R) -> Result<T> {
    let mut sub = Sub::top();

    for var in self.0 {
      // Is directly returning this error the right thing to do?
      sub = sub.with(var, Value::Var(src.acquire()))?;
    }

    self.1.sub(&sub, tracer)
  }
}

impl<T: Thing + Clone> Scheme<T> {
  pub fn make_inst<R: ThingTracer>(
    &self,
    src: &VarSource,
    tracer: R,
  ) -> Result<T> {
    let mut sub = Sub::top();

    for var in &self.0 {
      // Is directly returning this error the right thing to do?
      sub = sub.with(var.clone(), Value::Var(src.acquire()))?;
    }

    self.1.clone().sub(&sub, tracer)
  }
}

impl<T: Display> Display for Scheme<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("forall ")?;

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

impl<T: Thing> MaybeScheme<T> {
  pub fn generalize(inst: T) -> Self {
    let vars = inst.free_vars();

    if vars.is_empty() {
      MaybeScheme::Inst(inst)
    } else {
      MaybeScheme::Scheme(Scheme::generalize(inst, vars))
    }
  }

  // TODO: return self if instantiation fails?
  pub fn to_inst<R: ThingTracer>(
    &mut self,
    src: &VarSource,
    tracer: R,
  ) -> Result<&T> {
    use self::MaybeScheme::*;

    Ok(match self {
      Scheme(s) => {
        let mut scheme = unsafe { mem::zeroed() };
        mem::swap(s, &mut scheme);

        let mut me = Inst(scheme.inst(src, tracer)?);

        mem::swap(self, &mut me);
        mem::forget(me);

        if let Inst(i) = self {
          i
        } else {
          unreachable!()
        }
      },
      Inst(i) => i,
    })
  }

  pub fn into_inst<R: ThingTracer>(
    self,
    src: &VarSource,
    tracer: R,
  ) -> Result<T> {
    use self::MaybeScheme::*;

    match self {
      Scheme(s) => s.inst(src, tracer),
      Inst(i) => Ok(i),
    }
  }
}

impl<T: Thing + Clone> MaybeScheme<T> {
  pub fn as_inst<R: ThingTracer>(
    &self,
    src: &VarSource,
    tracer: R,
  ) -> Result<Cow<T>> {
    use self::MaybeScheme::*;

    Ok(match self {
      Scheme(s) => Cow::Owned(s.make_inst(src, tracer)?),
      Inst(i) => Cow::Borrowed(&i),
    })
  }
}

impl<T: Thing + Unify> MaybeScheme<T> {
  pub fn unify_inst<R: UnifyTracer>(
    &mut self,
    rhs: &mut Self,
    src: &VarSource,
    tracer: R,
  ) -> Result<Sub> {
    let a = self.to_inst(src, tracer.for_thing())?;
    let b = rhs.to_inst(src, tracer.for_thing())?;

    a.unify(b, tracer)
  }

  pub fn inst_and_unify<R: UnifyTracer>(
    self,
    rhs: Self,
    src: &VarSource,
    tracer: R,
  ) -> Result<(T, T, Sub)> {
    self.into_inst(src, tracer.for_thing()).and_then(|a| {
      rhs
        .into_inst(src, tracer.for_thing())
        .and_then(|b| a.unify(&b, tracer).map(|sub| (a, b, sub)))
    })
  }
}

impl<T: Display> Display for MaybeScheme<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MaybeScheme::Scheme(s) => Display::fmt(s, fmt),
      MaybeScheme::Inst(i) => Display::fmt(i, fmt),
    }
  }
}
