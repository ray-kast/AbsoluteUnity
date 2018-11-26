use std::ops::{Generator, GeneratorState};

pub struct GenIter<G: Generator<Return = ()>>(pub G);

impl<G: Generator<Return = ()>> Iterator for GenIter<G> {
  type Item = G::Yield;

  fn next(&mut self) -> Option<Self::Item> {
    match unsafe { self.0.resume() } {
      GeneratorState::Yielded(v) => Some(v),
      GeneratorState::Complete(()) => None,
    }
  }
}
