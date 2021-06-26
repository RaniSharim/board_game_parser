#[macro_use] extern crate lalrpop_util;

mod ast;

lalrpop_mod!(pub lang); // synthesized by LALRPOP

fn main() {
    print!("{}\n",&format!("{:?}",lang::DrawActionParser::new().parse("draw an event card from the action deck").unwrap()));

    print!("{}\n",&format!("{:?}",lang::ResourcePoolSelectionParser::new().parse("the discard pile").unwrap()));

}
