use super::prelude::*;

#[derive(Clone, PartialEq, Debug)] // TODO: can this safely derive Eq?
pub struct Sub(HashMap<Var, Value>);

impl Sub {
  #[inline]
  pub fn top() -> Self { Sub(HashMap::new()) }

  pub fn with(mut self, var: Var, is: Value) -> Result<Self> {
    if self.0.insert(var, is).is_some() {
      panic!("duplicate substitution"); // TODO: this is bad
    }

    Ok(self)
  }

  pub fn merge(mut self, rhs: Self) -> Result<Self> {
    for (var, is) in rhs.0 {
      self = self.with(var, is)?;
    }

    Ok(self)
  }

  pub fn into_map(self) -> HashMap<Var, Value> { self.0 }

  pub fn get(&self, var: &Var) -> Option<&Value> { self.0.get(var) }

  pub fn is_top(&self) -> bool { self.0.is_empty() }
}

impl Thing for Sub {
  fn sub(mut self, sub: &Sub) -> Self {
    use self::HashEntry::*;

    for (var, is) in &mut self.0 {
      if !sub.0.contains_key(var) {
        is.sub_self(sub);
      }
    }

    for (var, is) in &sub.0 {
      match self.0.entry(var.clone()) {
        Vacant(v) => {
          v.insert(is.clone());
        },
        Occupied(o) => o.into_mut().sub_self(sub),
      }
    }

    self
  }
}

impl Display for Sub {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    if self.0.is_empty() {
      // An empty Sub implies top
      fmt.write_str("⊤")?;
    } else {
      fmt.write_str("{")?;

      let mut first = true;

      for (var, is) in &self.0 {
        if first {
          first = false;
        } else {
          fmt.write_str(", ")?;
        }

        Display::fmt(var, fmt)?;

        fmt.write_str(" ~ ")?;

        Display::fmt(is, fmt)?;
      }

      fmt.write_str("}")?;
    }

    Ok(())
  }
}
