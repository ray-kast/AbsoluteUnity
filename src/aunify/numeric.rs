use super::prelude::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Numeric {
  Var(Var),
  Int(i32),
  // Real(f64), // TODO: deal with Hash and Eq
  Unop(UnaryOp, Box<Numeric>),
  Binop(BinaryOp, Box<Numeric>, Box<Numeric>),
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum UnaryOp {
  Pos, // TODO: this may need special unification
  Neg,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
}

impl Numeric {
  pub fn fold(&mut self) {
    use self::{BinaryOp::*, Numeric::*, UnaryOp::*};

    match self {
      Var(_) => {},
      Int(_) => {},
      Unop(o, ref mut n) => {
        n.fold();

        match **n {
          Int(i) => {
            *self = Int(match o {
              Pos => i,
              Neg => -i,
            })
          },
          _ => {},
        }
      },
      Binop(o, ref mut a, ref mut b) => {
        a.fold();
        b.fold();

        match (&**a, &**b) {
          (Int(a), Int(b)) => {
            *self = Int(match o {
              Add => a + b,
              Sub => a - b,
              Mul => a * b,
              Div => a / b,
              Mod => a % b,
            })
          },
          _ => {},
        }
      },
    }
  }

  fn disp_prec(&self, prec: u32, fmt: &mut fmt::Formatter) -> fmt::Result {
    use self::{BinaryOp::*, Numeric::*, UnaryOp::*};

    let my_prec = match self {
      Var(_) => 4,
      Int(_) => 4,
      Unop(Pos, _) => 3,
      Unop(Neg, _) => 3,
      Binop(Add, _, _) => 0,
      Binop(Sub, _, _) => 0,
      Binop(Mul, _, _) => 1,
      Binop(Div, _, _) => 1,
      Binop(Mod, _, _) => 2,
    };

    if my_prec < prec {
      fmt.write_str("(")?;
    }

    match self {
      Var(v) => Display::fmt(v, fmt)?,
      Int(i) => Display::fmt(i, fmt)?,
      Unop(o, n) => {
        fmt.write_str(match o {
          Pos => "+",
          Neg => "-",
        })?;

        n.disp_prec(my_prec, fmt)?;
      },
      Binop(o, a, b) => {
        a.disp_prec(my_prec, fmt)?;

        fmt.write_str(match o {
          Add => " + ",
          Sub => " - ",
          Mul => " * ",
          Div => " / ",
          Mod => " % ",
        })?;

        b.disp_prec(my_prec, fmt)?;
      },
    }

    if my_prec < prec {
      fmt.write_str(")")?;
    }

    Ok(())
  }
}

impl Thing for Numeric {
  fn collect_free_vars(&self, set: &mut HashSet<Var>) {
    match self {
      Numeric::Var(v) => {
        set.insert(v.clone());
      },
      Numeric::Int(_) => {},
      Numeric::Unop(_, n) => n.collect_free_vars(set),
      Numeric::Binop(_, a, b) => {
        a.collect_free_vars(set);
        b.collect_free_vars(set);
      },
    }
  }

  fn sub(mut self, sub: &Sub) -> Result<Self> {
    use self::Numeric::*;

    self.fold(); // TODO: doing the fold only here seems highly problematic

    Ok(match self {
      Var(v) => match sub.get(&v).map_or(Value::Var(v), |v| v.clone()) {
        Value::Var(v) => Var(v),
        Value::Numeric(n) => n,
        v => return Err(ErrorKind::SubBadType("Numeric", v).into()),
      },
      Int(i) => Int(i),
      Unop(o, n) => Unop(o, Box::new(n.sub(sub)?)),
      Binop(o, a, b) => Binop(o, Box::new(a.sub(sub)?), Box::new(b.sub(sub)?)),
    })
  }

  fn can_sub(&self, sub: &Sub) -> bool {
    use self::Numeric::*;

    match self {
      Var(v) => sub.get(v).map_or(true, |v| match v {
        Value::Var(_) => true,
        Value::Numeric(_) => true,
        _ => false,
      }),
      Int(_) => true,
      Unop(_, n) => n.can_sub(sub),
      Binop(_, a, b) => a.can_sub(sub) && b.can_sub(sub),
    }
  }
}

impl Unify for Numeric {
  fn unify(&self, rhs: &Self) -> Result<Sub> {
    use self::Numeric::*;

    match (self, rhs) {
      (Var(a), Var(b)) => {
        if a == b {
          Ok(Sub::top())
        } else {
          Sub::top().with(a.clone(), Value::Var(b.clone()))
        }
      },
      (Var(a), b) => {
        if b.free_vars().contains(a) {
          Err(ErrorKind::VarBothSides(a.clone()).into())
        } else {
          Sub::top().with(a.clone(), Value::Numeric(b.clone()))
        }
      },
      (a, Var(b)) => {
        if a.free_vars().contains(b) {
          Err(ErrorKind::VarBothSides(b.clone()).into())
        } else {
          Sub::top().with(b.clone(), Value::Numeric(a.clone()))
        }
      },
      (Int(a), Int(b)) if a == b => Ok(Sub::top()),
      (Unop(o1, n1), Unop(o2, n2)) if o1 == o2 => n1.unify(n2),
      (Binop(o1, a1, b1), Binop(o2, a2, b2)) if o1 == o2 => {
        let asub = a1.unify(a2)?;

        let bsub = b1.clone().sub(&asub)?.unify(&b2.clone().sub(&asub)?)?;

        asub.sub(&bsub)
      },
      _ => Err(ErrorKind::BadNumericUnify(self.clone(), rhs.clone()).into()),
    }
  }
}

impl Display for Numeric {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    self.disp_prec(0, fmt)
  }
}
