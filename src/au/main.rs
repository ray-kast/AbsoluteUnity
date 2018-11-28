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
use clap::{App, Arg};
use std::{
  fs::File,
  io::{self, prelude::*},
  path::Path,
};

fn print(res: EvalResult) {
  use self::EvalResult::*;

  match res {
    Unit => {},
    Assert(v) => {
      for stmt in v {
        println!(" {}.", stmt);
      }
    },
    Query(i) => {
      for sol in i {
        println!(" {};", sol); // TODO: lazy-evaluate this
      }

      println!(" ⊥.");
    },
    UnifyVal(Ok((a, b, sub, a2, b2))) => {
      println!(" unify result: {}", sub);
      println!("     lhs: {} ~ {}", a, a2);
      println!("     rhs: {} ~ {}", b, b2);
    },
    UnifyVal(Err(e)) => println!(" unify failed: {}", e),
    UnifyApp(Ok((a, b, sub, a2, b2))) => {
      println!(" unify result: {}", sub);
      println!("     lhs: {} ~ {}", a, a2);
      println!("     rhs: {} ~ {}", b, b2);
    },
    UnifyApp(Err(e)) => println!(" unify failed: {}", e),
    PrintVal(v) => println!(" {}", v),
    PrintStmt(s) => println!(" {}", s),
    PrintEnv(v) => {
      for premise in v {
        println!(" {}", premise);
      }
    },
  }
}

fn main() {
  use self::ReadResult::*;

  let mut reader = Reader::new(".au-history");
  let mut evalr = Evaluator::new();

  let matches = App::new("au")
    .version(env!("CARGO_PKG_VERSION"))
    .about("a Prolog-like declarative language")
    .arg(Arg::with_name("inputs").multiple(true).required(false))
    .get_matches();

  if let Some(inputs) = matches.values_of("inputs") {
    for input in inputs {
      fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
        let mut file = File::open(path)?;

        let mut s = String::new();

        file.read_to_string(&mut s)?;

        Ok(s)
      }

      match read_file(input) {
        Ok(s) => {
          if let Some(i) = reader.read_input(input, &s) {
            evalr.eval_input(i);
          }
        },
        Err(e) => println!("failed to read {}: {}", input, e),
      }
    }
  }

  // TODO: re-read input files on Expr::Reset
  loop {
    match reader.read() {
      Eval(a) => print(evalr.eval(a)),
      Stop => break,
    }
  }
}
