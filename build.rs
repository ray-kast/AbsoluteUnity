use lalrpop;
use std::env;

fn main() {
  lalrpop::Configuration::new()
    .set_in_dir("src/au")
    .set_out_dir(env::var("OUT_DIR").unwrap())
    .process()
    .unwrap();
}
