use std::env;

mod expression;
mod token;
mod parser;
mod table;

use crate::expression::Expression;
use crate::parser::Parser;
use crate::table::print_table;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: truthtable \"<expression>\" \"<expression2>\" ...");
        eprintln!();
        eprintln!("Operators:");
        eprintln!(" NOT:  ! ~ not");
        eprintln!(" AND:  & && and");
        eprintln!(" OR:  | || or");
        eprintln!(" XOR:  ^ xor");
        eprintln!(" IMPLIES:  -> => impl");
        eprintln!(" BICONDITIONAL:  <-> <=> iff");
        eprintln!();

        std::process::exit(1);
    }

    let expressions: Vec<Expression> = args.iter().map(|s| Parser::new(s).parse()).collect();

    println!();
    print_table(&expressions);
    println!();
}