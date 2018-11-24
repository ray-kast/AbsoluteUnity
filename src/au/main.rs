#[macro_use]
extern crate lalrpop_util;

mod ast;
mod read;

lalrpop_mod!(pub parser);

#[cfg(test)]
mod tests;

use crate::{
  ast::Expr as Ast,
  read::{ReadResult, Reader},
};
use aunify;

type Expr = Ast;

fn eval(ast: Ast) -> Expr { ast }

fn print(expr: Expr) {
  println!("{:?}", expr);
}

fn main() {
  use self::ReadResult::*;

  let mut reader = Reader::new(".au-history");

  loop {
    match reader.read() {
      Eval(a) => print(eval(a)),
      Stop => break,
    }
  }
}
