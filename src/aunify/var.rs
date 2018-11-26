use super::prelude::*;

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
        Display::fmt(n, fmt)?;
      },
      Var::Auto(i) => {
        fmt.write_str("_")?;
        Display::fmt(i, fmt)?;
      },
    }

    Ok(())
  }
}

pub struct VarSource {
  curr: u32,
} // TODO

impl VarSource {
  pub fn new() -> Self { Self { curr: 0 } }

  pub fn acquire(&mut self) -> Var {
    let ret = Var::Auto(self.curr);

    self.curr = self.curr + 1;

    ret
  }
}
