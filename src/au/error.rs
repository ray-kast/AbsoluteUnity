error_chain! {
  types { Error, ErrorKind, ResultExt, Result; }

  foreign_links {}

  errors {
    CompileTypeError(expect: &'static str, got: String) {
      description("type error during compilation")
      display("type error: expected {}, got {}", expect, got)
    }
  }
}