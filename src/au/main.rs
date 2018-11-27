#[macro_use]
extern crate lalrpop_util;

mod ast;
mod eval;
mod read;

lalrpop_mod!(pub parser);

#[cfg(test)]
mod tests;

use crate::{
  eval::{EvalResult, Evaluator},
  read::{ReadResult, Reader},
};
use aunify;

fn print(res: EvalResult) {
  use self::EvalResult::*;

  match res {
    Assert(v) => {
      for stmt in v {
        println!("{}.", stmt);
      }
    },
    Query(i) => {
      for sol in i {
        println!("{};", sol); // TODO: lazy-evaluate this
      }

      println!("⊥.");
    },
    UnifyVal(Ok((a, b, sub, a2, b2))) => {
      println!("unify result: {}", sub);
      println!("    lhs: {} ~ {}", a, a2);
      println!("    rhs: {} ~ {}", b, b2);
    },
    UnifyVal(Err(e)) => println!("unify failed: {}", e),
    UnifyApp(Ok((a, b, sub, a2, b2))) => {
      println!("unify result: {}", sub);
      println!("    lhs: {} ~ {}", a, a2);
      println!("    rhs: {} ~ {}", b, b2);
    },
    UnifyApp(Err(e)) => println!("unify failed: {}", e),
  }
}

fn main() {
  use self::ReadResult::*;

  let mut reader = Reader::new(".au-history");
  let mut evalr = Evaluator::new();

  loop {
    match reader.read() {
      Eval(a) => print(evalr.eval(a)),
      Stop => break,
    }
  }
}
