use super::{prelude::*, tracer::prelude::*};
use std::mem;

// TODO: this is an awful name and you know it
pub trait Thing: Display {
  fn collect_free_vars(&self, set: &mut HashSet<Var>);

  fn free_vars(&self) -> HashSet<Var> {
    let mut ret = HashSet::new();

    self.collect_free_vars(&mut ret);

    ret
  }

  // TODO: return self on substitution failure?
  // fn sub_impl<T: ThingTracer<Self>>(
  fn sub_impl<T: ThingTracer>(
    self,
    sub: &Sub,
    tracer: T::SubHandle,
  ) -> Result<Self>
  where
    Self: Sized;

  #[inline]
  // fn sub<T: ThingTracer<Self>>(self, sub: &Sub, tracer: T) -> Result<Self>
  fn sub<T: ThingTracer>(self, sub: &Sub, tracer: T) -> Result<Self>
  where
    Self: Display + Sized,
  {
    let tracer = tracer.begin_sub(&self, sub);
    let ret = self.sub_impl::<T>(sub, tracer.clone());
    tracer.pre_return(&ret);
    ret
  }

  // fn sub_self<T: ThingTracer<Self>>(
  fn sub_self<T: ThingTracer>(&mut self, sub: &Sub, tracer: T) -> Result<()>
  where
    Self: Display + Sized,
  {
    let mut me = unsafe { mem::zeroed() };
    mem::swap(self, &mut me);

    me = me.sub(sub, tracer)?;

    mem::swap(self, &mut me);
    mem::forget(me);

    Ok(())
  }

  fn can_sub(&self, sub: &Sub) -> bool;
}

// TODO: add tracing for unification
pub trait Unify: Display {
  fn unify_impl<T: UnifyTracer>(
    &self,
    rhs: &Self,
    tracer: T::UnifyHandle,
  ) -> Result<Sub>;

  #[inline]
  fn unify<T: UnifyTracer>(&self, rhs: &Self, tracer: T) -> Result<Sub>
  where
    Self: Display + Sized,
  {
    let tracer = tracer.begin_unify(self, rhs);
    let ret = self.unify_impl::<T>(rhs, tracer.clone());
    tracer.pre_return(&ret);
    ret
  }
}
