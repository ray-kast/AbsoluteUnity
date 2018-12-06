#![feature(try_from, bind_by_move_pattern_guards)]

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate error_chain; // TODO: this should be able to be removed, somehow

mod ast;
mod error;
mod eval;
mod read;
mod tracer;

lalrpop_mod!(pub parser);

#[cfg(test)]
mod tests;

pub use self::error::*;

mod prelude {
  pub use super::*;
  pub use std::convert::TryInto;
}

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

fn print(res: Result<EvalResult>) {
  use self::EvalResult::*;

  match res {
    Ok(r) => match r {
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
    },
    Err(e) => {
      println!(" compile error: {}", e);
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
      fn read_file<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<String> {
        let mut file = File::open(path)?;

        let mut s = String::new();

        file.read_to_string(&mut s)?;

        Ok(s)
      }

      match read_file(input) {
        Ok(s) => {
          if let Some(i) = reader.read_input(input, &s) {
            match evalr.eval_input(i) {
              Ok(()) => {},
              Err(e) => println!("failed to compile {}: {}", input, e),
            }
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
