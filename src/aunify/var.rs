use super::prelude::*;
use std::cell::Cell;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Var {
  Formal(String),
  Auto(u32),
}

impl Display for Var {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Var::Formal(n) => {
        fmt.write_str("$")?;
        Display::fmt(n, fmt)
      },
      Var::Auto(i) => {
        fmt.write_str("_")?;
        Display::fmt(i, fmt)
      },
    }
  }
}

pub struct VarSource {
  curr: Cell<u32>,
}

impl VarSource {
  pub fn new() -> Self { Self { curr: Cell::new(0) } }

  pub fn acquire(&self) -> Var {
    let curr = self.curr.get();

    let ret = Var::Auto(curr);

    self.curr.set(curr + 1);

    ret
  }
}
