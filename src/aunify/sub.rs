use super::prelude::*;

#[derive(PartialEq, Debug)] // TODO: can this safely derive Eq?
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

  pub fn sub(mut self, sub: &Sub) -> Self {
    use self::HashEntry::*;

    for (var, is) in &sub.0 {
      match self.0.entry(var.clone()) {
        Vacant(v) => {
          v.insert(is.clone());
        },
        Occupied(mut o) => {
          // Temporarily insert a dummy value so I can consume the original
          let val = o.insert(Value::Atom(String::new()));
          o.insert(val.sub(sub));
        },
      }
    }

    self
  }

  pub fn get(&self, var: &Var) -> Option<&Value> { self.0.get(var) }
}
