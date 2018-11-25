use super::prelude::*;
use std::mem;

#[derive(Clone, Debug)]
pub struct Scheme<T>(Vec<Var>, T);

#[derive(Clone, Debug)]
pub enum MaybeScheme<T> {
  Scheme(Scheme<T>),
  Inst(T),
}

impl<T> Scheme<T> {
  pub fn generalize(t: T, on: Vec<Var>) -> Scheme<T> { Scheme(on, t) }
}

impl<T: Thing> Scheme<T> {
  pub fn inst(self, src: &mut VarSource) -> Result<T> {
    let mut sub = Sub::top();

    for var in self.0 {
      // Is directly returning this error the right thing to do?
      sub = sub.with(var, Value::Var(src.acquire()))?;
    }

    Ok(self.1.sub(&sub))
  }
}

impl<T: Thing> MaybeScheme<T> {
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
}
