use super::prelude::*;
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
}

impl<T: Thing> Scheme<T> {
  pub fn inst(self, src: &mut VarSource) -> Result<T> {
    let mut sub = Sub::top();

    for var in self.0 {
      // Is directly returning this error the right thing to do?
      sub = sub.with(var, Value::Var(src.acquire()))?;
    }

    self.1.sub(&sub)
  }
}

impl<T: Thing + Clone> Scheme<T> {
  pub fn make_inst(&self, src: &mut VarSource) -> Result<T> {
    let mut sub = Sub::top();

    for var in &self.0 {
      // Is directly returning this error the right thing to do?
      sub = sub.with(var.clone(), Value::Var(src.acquire()))?;
    }

    self.1.clone().sub(&sub)
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

    Display::fmt(&self.1, fmt)?;

    Ok(())
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
  pub fn to_inst(&mut self, src: &mut VarSource) -> Result<&T> {
    use self::MaybeScheme::*;

    Ok(match self {
      Scheme(s) => {
        let mut scheme = unsafe { mem::zeroed() };
        mem::swap(s, &mut scheme);

        let mut me = Inst(scheme.inst(src)?);

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

  pub fn into_inst(self, src: &mut VarSource) -> Result<T> {
    use self::MaybeScheme::*;

    match self {
      Scheme(s) => s.inst(src),
      Inst(i) => Ok(i),
    }
  }
}

impl<T: Thing + Clone> MaybeScheme<T> {
  pub fn as_inst(&self, src: &mut VarSource) -> Result<Cow<T>> {
    use self::MaybeScheme::*;

    Ok(match self {
      Scheme(s) => Cow::Owned(s.make_inst(src)?),
      Inst(i) => Cow::Borrowed(&i),
    })
  }
}

impl<T: Thing + Unify> MaybeScheme<T> {
  pub fn unify_inst(
    &mut self,
    rhs: &mut Self,
    src: &mut VarSource,
  ) -> Result<Sub> {
    let a = self.to_inst(src)?;
    let b = rhs.to_inst(src)?;

    a.unify(b)
  }

  pub fn inst_and_unify(
    self,
    rhs: Self,
    src: &mut VarSource,
  ) -> Result<(T, T, Sub)> {
    self.into_inst(src).and_then(|a| {
      rhs
        .into_inst(src)
        .and_then(|b| a.unify(&b).map(|sub| (a, b, sub)))
    })
  }
}

impl<T: Display> Display for MaybeScheme<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MaybeScheme::Scheme(s) => Display::fmt(s, fmt)?,
      MaybeScheme::Inst(i) => Display::fmt(i, fmt)?,
    }

    Ok(())
  }
}
