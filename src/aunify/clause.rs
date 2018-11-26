use super::prelude::*;

#[derive(Clone, Debug)]
pub enum Clause {
  App(App),
  Not(Box<Clause>),
  Any(Vec<Clause>),
  All(Vec<Clause>),
}

impl Clause {
  // Gonna reuse Any and All for top and bottom for simplicity's sake

  #[inline]
  pub fn top() -> Self { Clause::All(Vec::new()) }

  #[inline]
  pub fn bot() -> Self { Clause::Any(Vec::new()) }

  fn disp_prec(&self, prec: u32, fmt: &mut fmt::Formatter) -> fmt::Result {
    use self::Clause::*;

    let my_prec = match self {
      App(_) => 3,
      Not(_) => 2,
      Any(_) => 0,
      All(_) => 1,
    };

    if my_prec < prec {
      fmt.write_str("(")?;
    }

    match self {
      App(a) => Display::fmt(a, fmt)?,
      Not(c) => {
        fmt.write_str("~")?;
        c.disp_prec(my_prec, fmt)?;
      },
      Any(v) => {
        if v.is_empty() {
          fmt.write_str("⊥")?;
        } else {
          let mut first = true;

          for clause in v {
            if first {
              first = false;
            } else {
              fmt.write_str("; ")?;
            }

            clause.disp_prec(my_prec, fmt)?;
          }
        }
      },
      All(v) => {
        if v.is_empty() {
          fmt.write_str("⊤")?;
        } else {
          let mut first = true;

          for clause in v {
            if first {
              first = false;
            } else {
              fmt.write_str(", ")?;
            }

            clause.disp_prec(my_prec, fmt)?;
          }
        }
      },
    }

    if my_prec < prec {
      fmt.write_str(")")?;
    }

    Ok(())
  }
}

impl Thing for Clause {
  fn sub(self, sub: &Sub) -> Self {
    match self {
      Clause::App(a) => Clause::App(a.sub(sub)),
      Clause::Not(c) => Clause::Not(Box::new(c.sub(sub))),
      Clause::Any(v) => {
        Clause::Any(v.into_iter().map(|c| c.sub(sub)).collect())
      },
      Clause::All(v) => {
        Clause::All(v.into_iter().map(|c| c.sub(sub)).collect())
      },
    }
  }
}

impl Display for Clause {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    self.disp_prec(0, fmt)
  }
}
