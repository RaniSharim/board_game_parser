#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub lang); // synthesized by LALRPOP

fn main() {
    assert!(lang::DrawActionParser::new().parse("draw an event card").is_ok());
}
