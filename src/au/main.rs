#![feature(try_from, bind_by_move_pattern_guards, label_break_value)]

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate error_chain; // TODO: this should be able to be removed, somehow

mod ast;
mod error;
mod eval;
mod read;
mod readch;
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
  readch::Readch,
};
use aunify;
use clap::{App, Arg};
use std::{
  fs::File,
  io::{self, prelude::*},
  os::unix::io::AsRawFd,
  path::Path,
};

fn print(res: Result<EvalResult>, readch: &Readch) {
  use self::EvalResult::*;

  enum QueryAction {
    Next,
    Stop,
  }

  fn query_action<P: Fn() -> Result<()>>(
    readch: &Readch,
    prompt: P,
  ) -> Result<QueryAction> {
    prompt()?;

    Ok(loop {
      match readch.read()? {
        '\x03' => break QueryAction::Stop,
        '\x04' => break QueryAction::Stop,
        '\n' => break QueryAction::Stop,
        '\t' => break QueryAction::Next,
        '.' => break QueryAction::Stop,
        ';' => break QueryAction::Next,
        c => {
          writeln!(io::stdout(), "\r\x1b[2K Invalid input {:?}.", c)?;
          prompt()?;
        },
      }
    })
  }

  match res {
    Ok(r) => match r {
      Unit => {},
      Assert(v) => {
        for stmt in v {
          println!(" {}.", stmt);
        }
      },
      Query(i) => {
        let broken = 'unwrap: {
          for sol in i {
            let sol = sol.without_autos();

            match query_action(&readch, || {
              write!(io::stdout(), " {}", sol)?;
              io::stdout().flush()?;
              Ok(())
            })
            .unwrap()
            {
              QueryAction::Next => writeln!(io::stdout(), ";").unwrap(),
              QueryAction::Stop => break 'unwrap true,
            }
          }

          false
        };

        if broken {
          writeln!(io::stdout(), ".").unwrap();
        } else {
          writeln!(io::stdout(), " ⊥.").unwrap();
        }
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

  let readch = Readch::new(io::stdin().as_raw_fd()).unwrap();

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
      Eval(a) => print(evalr.eval(a), &readch),
      Stop => break,
    }
  }
}
