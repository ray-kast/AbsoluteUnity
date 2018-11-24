use crate::{
  ast::{Expr as Ast, ParserTag},
  parser::ExprParser,
};
use rustyline::{self, error::ReadlineError, Editor};
use std::path::Path;

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
    Self {
      histfile: histfile.as_ref(),
      editor: Editor::new(),
      parser: ExprParser::new(),
      parser_tag: ParserTag::new(),
    }
  }

  pub fn read(&mut self) -> ReadResult {
    use self::ReadResult::*;

    loop {
      match self.editor.readline("> ") {
        Ok(line) => {
          self.editor.add_history_entry(&*line);

          match self.parser.parse(&mut self.parser_tag, &*line) {
            Ok(x) => break Eval(x),
            Err(e) => println!("{}", e),
          }
        },
        Err(ReadlineError::Interrupted) => continue,
        Err(ReadlineError::Eof) => break Stop,
        Err(e) => panic!(format!("{}", e)),
      }
    }
  }
}

impl<'a> Drop for Reader<'a> {
  fn drop(&mut self) { self.editor.save_history(self.histfile).unwrap(); }
}
