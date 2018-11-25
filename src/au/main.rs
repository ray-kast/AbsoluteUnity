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
    Unify(Ok(s)) => println!("unify result: {}", s),
    Unify(Err(e)) => println!("unify failed: {}", e),
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
