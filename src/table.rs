use std::collections::{BTreeMap, BTreeSet};

use crate::expression::Expression;

fn bool2str<'a>(val: bool, t: &'a str, f: &'a str) -> &'a str {
    if val { t } else { f }
}

/// Pretty print a truth table
///
/// Each column is either a variable or a subexpression
pub fn print_table(expressions: &[Expression], truesym: &str, falsesym: &str) {
    let mut var_set = BTreeSet::new();

    for expr in expressions {
        expr.get_variables(&mut var_set);
    }

    let variables: Vec<String> = var_set.into_iter().collect();

    let mut already = Vec::new();
    let mut columns = Vec::new();

    for expr in expressions {
        let root = expr.to_string();

        expr.get_subexpressions(&mut already, &mut columns);

        if !already.contains(&root) {
            already.push(root);
            columns.push(expr);
        }
    }

    // headers
    let var_headers = &variables;
    let col_headers: Vec<String> = columns.iter().map(|e| e.to_string()).collect();

    let width = |s: &str| s.chars().count().max(1); // column widths

    let var_widths: Vec<_> = var_headers.iter().map(|h| width(h)).collect();
    let col_widths: Vec<_> = col_headers.iter().map(|h| width(h)).collect();

    let mut header = String::from("│");
    let mut divider = String::from("┼");

    let mut add_column = |text: &str, w: usize| {
        header.push_str(&format!(" {:^w$} │", text, w = w));
        divider.push_str(&"─".repeat(w + 2));
        divider.push('┼');
    };

    for (i, h) in var_headers.iter().enumerate() {
        add_column(h, var_widths[i]);
    }

    for (i, h) in col_headers.iter().enumerate() {
        add_column(h, col_widths[i]);
    }

    divider.pop();
    divider.push('┤');

    let top = divider.replace('┼', "┬").replace('┤', "┐").replacen('┬', "┌", 1);
    let bottom = divider.replace('┼', "┴").replace('┤', "┘").replacen('┴', "└", 1);

    println!("{top}");
    println!("{header}");
    println!("{divider}");

    // rows
    let vars_size = variables.len();
    let row_count = 1 << vars_size;

    for row_idx in 0..row_count {
        let mut assignment = BTreeMap::new();

        for (i, var) in variables.iter().enumerate() {
            let bit = (row_idx >> (vars_size - 1 - i)) & 1;

            assignment.insert(var.clone(), bit == 1);
        }

        let mut row_line = String::from("│");

        for (i, var) in variables.iter().enumerate() { // variable columns
            let cell = bool2str(*assignment.get(var).unwrap(), truesym, falsesym);

            row_line.push_str(&format!(" {:^w$} │", cell, w = var_widths[i]));
        }

        for (i, expr) in columns.iter().enumerate() { // expression columns
            let cell = bool2str(expr.evaluate(&assignment), truesym, falsesym);

            row_line.push_str(&format!(" {:^w$} │", cell, w = col_widths[i]));
        }

        println!("{row_line}");
    }

    println!("{bottom}");
}