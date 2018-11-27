use super::prelude::*;
use std::mem;

// TODO: this is an awful name and you know it
pub trait Thing {
  fn collect_free_vars(&self, set: &mut HashSet<Var>);

  fn free_vars(&self) -> HashSet<Var> {
    let mut ret = HashSet::new();

    self.collect_free_vars(&mut ret);

    ret
  }

  fn sub(self, sub: &Sub) -> Self;

  fn sub_self(&mut self, sub: &Sub)
  where
    Self: Sized,
  {
    let mut me = unsafe { mem::zeroed() };
    mem::swap(self, &mut me);

    me = me.sub(sub);

    mem::swap(self, &mut me);
    mem::forget(me);
  }
}

// TODO: this may need to cover non-deterministic cases
pub trait UnifyCore {
  fn unify_core(&self, rhs: &Self) -> Result<Sub>;
}

pub trait Unify {
  fn unify(&self, rhs: &Self) -> Result<Sub>;
}

impl<T: Thing + UnifyCore> Unify for T {
  fn unify(&self, rhs: &Self) -> Result<Sub> {
    if self.free_vars().is_disjoint(&rhs.free_vars()) {
      self.unify_core(rhs)
    } else {
      Err(
        ErrorKind::UnsolvableUnify("some variables occur on both sides").into(),
      )
    }
  }
}
