use super::prelude::*;
use aunify::{self as a, MaybeScheme, Pred, RcPred, Scheme, Thing, VarSource};
use std::{collections::HashMap, rc::Rc};

pub use aunify::numeric::{BinaryOp, UnaryOp};

pub struct CompileCtx {
  pred_map: HashMap<(String, usize), RcPred>,
  var_src: Rc<VarSource>,
}

impl CompileCtx {
  pub fn new(var_src: Rc<VarSource>) -> Self {
    Self {
      pred_map: HashMap::new(),
      var_src,
    }
  }

  pub fn pred(&mut self, name: String, arity: usize) -> RcPred {
    self
      .pred_map
      .entry((name.clone(), arity))
      .or_insert_with(|| Pred::new_rc(name, arity))
      .clone()
  }

  pub fn auto_var(&mut self) -> a::Var { self.var_src.acquire() }
}

pub trait CompileTo<T>
where
  Self: Sized,
{
  fn compile(self, ctx: &mut CompileCtx) -> Result<T>;
}

pub struct ParserTag;

#[derive(Debug)]
pub enum Command {
  Assert(Vec<MaybeScheme<a::Statement>>),
  Query(a::Clause),
  UnifyVal(MaybeScheme<a::Value>, MaybeScheme<a::Value>),
  UnifyApp(MaybeScheme<a::App>, MaybeScheme<a::App>),
  TraceQuery(a::Clause),
  PrintVal(MaybeScheme<a::Value>),
  PrintStmt(MaybeScheme<a::Statement>),
  Fold(a::Numeric),
  PrintEnv,
  Reset,
}

#[derive(PartialEq, Debug)]
pub enum Expr {
  Assert(Assert),
  Query(Clause),
  UnifyVal(SchemePrefix<Value>, SchemePrefix<Value>),
  UnifyApp(SchemePrefix<App>, SchemePrefix<App>),
  TraceQuery(Clause),
  PrintVal(SchemePrefix<Value>),
  PrintStmt(SchemePrefix<Statement>),
  Fold(Value),
  PrintEnv,
  Reset,
}

impl CompileTo<Command> for Expr {
  fn compile(self, ctx: &mut CompileCtx) -> Result<Command> {
    Ok(match self {
      Expr::Assert(a) => Command::Assert(a.compile(ctx)?),
      Expr::Query(c) => Command::Query(c.compile(ctx)?),
      Expr::UnifyVal(a, b) => {
        Command::UnifyVal(a.compile(ctx)?, b.compile(ctx)?)
      },
      Expr::UnifyApp(a, b) => {
        Command::UnifyApp(a.compile(ctx)?, b.compile(ctx)?)
      },
      Expr::TraceQuery(c) => Command::TraceQuery(c.compile(ctx)?),
      Expr::PrintVal(v) => Command::PrintVal(v.compile(ctx)?),
      Expr::PrintStmt(s) => Command::PrintStmt(s.compile(ctx)?),
      Expr::Fold(v) => Command::Fold(v.compile(ctx)?),
      Expr::PrintEnv => Command::PrintEnv,
      Expr::Reset => Command::Reset,
    })
  }
}

#[derive(PartialEq, Debug)]
pub struct Input(pub Assert);

impl CompileTo<Vec<MaybeScheme<a::Statement>>> for Input {
  fn compile(
    self,
    ctx: &mut CompileCtx,
  ) -> Result<Vec<MaybeScheme<a::Statement>>> {
    self.0.compile(ctx)
  }
}

#[derive(PartialEq, Debug)]
pub struct Assert(pub Vec<AutoScheme<Statement>>);

impl CompileTo<Vec<a::MaybeScheme<a::Statement>>> for Assert {
  fn compile(
    self,
    ctx: &mut CompileCtx,
  ) -> Result<Vec<MaybeScheme<a::Statement>>> {
    self.0.into_iter().map(|s| s.compile(ctx)).collect()
  }
}

#[derive(PartialEq, Debug)]
pub struct AutoScheme<T>(pub T);

impl<T: CompileTo<U>, U: Thing> CompileTo<MaybeScheme<U>> for AutoScheme<T> {
  fn compile(self, ctx: &mut CompileCtx) -> Result<MaybeScheme<U>> {
    match self.0.compile(ctx) {
      Ok(u) => Ok(MaybeScheme::generalize(u)),
      Err(e) => Err(e.into()),
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum SchemePrefix<T> {
  Specific(Vec<NamedVar>, T),
  All(T),
  Inst(T),
}

impl<T: CompileTo<U>, U: Thing> CompileTo<MaybeScheme<U>> for SchemePrefix<T> {
  fn compile(self, ctx: &mut CompileCtx) -> Result<MaybeScheme<U>> {
    match self {
      SchemePrefix::Specific(v, t) => match t.compile(ctx) {
        Ok(u) => Ok(MaybeScheme::Scheme(Scheme::generalize(
          u,
          v.into_iter().map(|v| v.into()).collect(),
        ))),
        Err(e) => Err(e.into()),
      },
      SchemePrefix::All(t) => match t.compile(ctx) {
        Ok(u) => Ok(MaybeScheme::generalize(u)),
        Err(e) => Err(e.into()),
      },
      SchemePrefix::Inst(t) => match t.compile(ctx) {
        Ok(u) => Ok(MaybeScheme::Inst(u)),
        Err(e) => Err(e.into()),
      },
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum Statement {
  Cond(App, Clause),
  Fact(App),
}

impl CompileTo<a::Statement> for Statement {
  fn compile(self, ctx: &mut CompileCtx) -> Result<a::Statement> {
    Ok(match self {
      Statement::Cond(l, r) => {
        a::Statement::new(l.compile(ctx)?, r.compile(ctx)?)
      },
      Statement::Fact(l) => a::Statement::fact(l.compile(ctx)?),
    })
  }
}

#[derive(PartialEq, Debug)]
pub enum Clause {
  Top,
  Bot,
  App(App),
  Not(Box<Clause>),
  And(Box<Clause>, Box<Clause>),
  Or(Box<Clause>, Box<Clause>),
}

impl CompileTo<a::Clause> for Clause {
  fn compile(self, ctx: &mut CompileCtx) -> Result<a::Clause> {
    Ok(match self {
      Clause::Top => a::Clause::Top,
      Clause::Bot => a::Clause::Bot,
      Clause::App(a) => a::Clause::App(a.compile(ctx)?),
      Clause::Not(c) => a::Clause::Not(Box::new((*c).compile(ctx)?)),
      Clause::And(a, b) => a::Clause::And(
        Box::new((*a).compile(ctx)?),
        Box::new((*b).compile(ctx)?),
      ),
      Clause::Or(a, b) => a::Clause::Or(
        Box::new((*a).compile(ctx)?),
        Box::new((*b).compile(ctx)?),
      ),
    })
  }
}

#[derive(PartialEq, Debug)]
pub struct App(pub Atom, pub Tuple);

impl CompileTo<a::App> for App {
  fn compile(self, ctx: &mut CompileCtx) -> Result<a::App> {
    Ok(a::App::new(
      ctx.pred(self.0 .0, self.1 .0.len()),
      self.1.compile(ctx)?,
    ))
  }
}

#[derive(PartialEq, Debug)]
pub enum Value {
  Var(Var),
  Atom(Atom),

  // Numeric
  Int(i32),
  Unop(UnaryOp, Box<Value>),
  Binop(BinaryOp, Box<Value>, Box<Value>),

  // Tuple
  Tuple(Tuple),

  // List
  List(Box<Value>, Box<Value>),
  EmptyList,
}

impl CompileTo<a::Value> for Value {
  fn compile(self, ctx: &mut CompileCtx) -> Result<a::Value> {
    Ok(match self {
      Value::Var(v) => a::Value::Var(v.compile(ctx)?),
      Value::Atom(a) => a::Value::Atom(a.0),
      Value::Int(i) => a::Value::Numeric(a::Numeric::Int(i)),
      Value::Unop(o, v) => {
        a::Value::Numeric(a::Numeric::Unop(o, Box::new(v.compile(ctx)?)))
      },
      Value::Binop(o, a, b) => a::Value::Numeric(a::Numeric::Binop(
        o,
        Box::new(a.compile(ctx)?),
        Box::new(b.compile(ctx)?),
      )),
      Value::Tuple(t) => a::Value::Tuple(t.compile(ctx)?),
      Value::List(a, b) => a::Value::List(Box::new(a::List::Cons(
        a.compile(ctx)?,
        b.compile(ctx)?,
      ))),
      Value::EmptyList => a::Value::List(Box::new(a::List::Nil)),
    })
  }
}

impl CompileTo<a::Numeric> for Value {
  fn compile(self, ctx: &mut CompileCtx) -> Result<a::Numeric> {
    match self {
      Value::Var(v) => Ok(a::Numeric::Var(v.compile(ctx)?)),
      Value::Int(i) => Ok(a::Numeric::Int(i)),
      Value::Unop(o, v) => Ok(a::Numeric::Unop(o, Box::new(v.compile(ctx)?))),
      Value::Binop(o, a, b) => Ok(a::Numeric::Binop(
        o,
        Box::new(a.compile(ctx)?),
        Box::new(b.compile(ctx)?),
      )),
      Value::Tuple(t) if t.0.len() == 1 => {
        t.0.into_iter().next().unwrap().compile(ctx)
      },
      v => {
        Err(ErrorKind::CompileTypeError("Numeric", format!("{:?}", v)).into())
      },
    }
  }
}

impl CompileTo<a::list::Tail> for Value {
  fn compile(self, ctx: &mut CompileCtx) -> Result<a::list::Tail> {
    match self {
      Value::Var(v) => Ok(a::list::Tail::Open(v.compile(ctx)?)),
      Value::List(a, b) => Ok(a::list::Tail::Close(Box::new(a::List::Cons(
        a.compile(ctx)?,
        b.compile(ctx)?,
      )))),
      Value::EmptyList => Ok(a::list::Tail::Close(Box::new(a::List::Nil))),
      v => Err(ErrorKind::CompileTypeError("Tail", format!("{:?}", v)).into()),
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum Var {
  Named(NamedVar),
  Anon, // > be me
}

impl CompileTo<a::Var> for Var {
  fn compile(self, ctx: &mut CompileCtx) -> Result<a::Var> {
    Ok(match self {
      Var::Named(n) => n.into(),
      Var::Anon => ctx.auto_var(),
    })
  }
}

#[derive(PartialEq, Debug)]
pub struct Tuple(pub Vec<Value>);

impl CompileTo<a::Tuple> for Tuple {
  fn compile(self, ctx: &mut CompileCtx) -> Result<a::Tuple> {
    Ok(a::Tuple(
      self
        .0
        .into_iter()
        .map(|v| v.compile(ctx))
        .collect::<Result<Vec<_>>>()?,
    ))
  }
}

// The "terminal" symbols

#[derive(PartialEq, Debug)]
pub struct NamedVar(pub String);

impl Into<a::Var> for NamedVar {
  fn into(self) -> a::Var { a::Var::Formal(self.0) }
}

#[derive(PartialEq, Debug)]
pub struct Atom(pub String);
