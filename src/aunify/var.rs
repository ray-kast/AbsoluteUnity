use super::prelude::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Var {
  Formal(String),
  Auto(u32),
}

impl IntoTrace for Var {
  fn into_trace(self) -> Self {
    use self::Var::*;

    match self {
      Formal(s) => Formal(s),
      Auto(_) => Auto(0),
    }
  }
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
}

impl VarSource {
  pub fn new() -> Self { Self { curr: 0 } }

  pub fn acquire(&mut self) -> Var {
    let ret = Var::Auto(self.curr);

    self.curr = self.curr + 1;

    ret
  }
}
