use crate::{
  ast::{Expr as Ast, ParserTag},
  parser::ExprParser,
};
use lalrpop_util::ParseError;
use rustyline::{self, error::ReadlineError, Editor};
use std::{fmt::Write, path::Path};

pub struct Reader<'a> {
  histfile: &'a Path,
  editor: Editor<()>,
  parser: ExprParser,
  parser_tag: ParserTag,
}

pub enum ReadResult {
  Eval(Ast),
  Stop,
}

impl<'a> Reader<'a> {
  pub fn new<P: AsRef<Path> + ?Sized>(histfile: &'a P) -> Self {
    let mut ret = Self {
      histfile: histfile.as_ref(),
      editor: Editor::new(),
      parser: ExprParser::new(),
      parser_tag: ParserTag::new(),
    };

    let _ = ret.editor.load_history(ret.histfile);

    ret
  }

  pub fn read(&mut self) -> ReadResult {
    use self::ReadResult::*;

    let mut input = String::new();

    loop {
      match self
        .editor
        .readline(if input.is_empty() { "> " } else { "|   " })
      {
        Ok(line) => {
          input.write_str(&line).unwrap();

          match self.parser.parse(&mut self.parser_tag, &input) {
            Ok(x) => {
              self.editor.add_history_entry(&*input);
              break Eval(x);
            },
            Err(ParseError::UnrecognizedToken {
              token: None,
              expected: _,
            }) => {},
            Err(e) => {
              println!("{}", e);

              self.editor.add_history_entry(&*input);
              input.clear();
            },
          };
        },
        Err(ReadlineError::Interrupted) => input.clear(),
        Err(ReadlineError::Eof) => break Stop,
        Err(e) => panic!(format!("{}", e)),
      }
    }
  }
}

impl<'a> Drop for Reader<'a> {
  fn drop(&mut self) {
    // TODO: maybe emit a warning if this fails
    let _ = self.editor.save_history(self.histfile);
  }
}
