mod expression;
mod token;
mod parser;
mod table;

use crate::{expression::Expression, parser::Parser, table::print_table};

fn usage() {
    println!("Usage: truthtable \"<expression>\" \"<expression2>\" ...");
    println!();
    println!("Operators:");
    println!(" NOT:  ! ~ not");
    println!(" AND:  & && and");
    println!(" OR:  | || or");
    println!(" XOR:  ^ xor");
    println!(" IMPLIES:  -> => impl");
    println!(" BICONDITIONAL:  <-> <=> iff");
    println!();
    println!("Additional params:");
    println!(" --t <true symbol>");
    println!(" --f <false symbol>");
    println!()
}

fn next_or_exit<I: Iterator<Item = String>>(argv: &mut I) -> String {
    argv.next().unwrap_or_else(|| {
        usage();
        std::process::exit(1);
    })
}

fn main() {
    let mut args = Vec::new();
    let mut args_iter = std::env::args().skip(1);

    let mut t = "T".to_string();
    let mut f = "F".to_string();

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--t" => t = next_or_exit(&mut args_iter),
            "--f" => f = next_or_exit(&mut args_iter),
            _ => args.push(arg),
        }
    }

    if args.is_empty() {
        usage();

        std::process::exit(1);
    }

    let expressions: Vec<Expression> = args.iter().map(|s| Parser::new(s).parse()).collect();

    println!();
    print_table(&expressions, &t, &f);
    println!();
}