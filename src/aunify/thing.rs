use super::prelude::*;
use std::mem;

// TODO: this is an awful name and you know it
pub trait Thing {
  fn free_vars(&self) -> HashSet<Var> { unimplemented!() } // TODO

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
pub trait Unify {
  fn unify(&self, rhs: &Self) -> Result<Sub>;
}
