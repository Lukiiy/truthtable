use std::collections::{BTreeMap, BTreeSet};

use crate::expression::Expression;

fn bool2str<'a>(val: bool, t: &'a str, f: &'a str) -> &'a str {
    if val { t } else { f }
}

/// Pretty print a truth table
///
/// Each column is either a variable or a subexpression
pub fn print_table(expressions: &[Expression], truesym: &str, falsesym: &str) {
    let variables: Vec<String> = {
        let mut set = BTreeSet::new();

        expressions.iter().for_each(|e| e.get_variables(&mut set));
        set.into_iter().collect()
    };

    let columns: Vec<&Expression> = {
        let mut already = Vec::new();
        let mut cols = Vec::new();

        for expr in expressions {
            expr.get_subexpressions(&mut already, &mut cols);

            let root = expr.to_string();

            if !already.contains(&root) {
                already.push(root);
                cols.push(expr);
            }
        }

        cols
    };

    let col_headers: Vec<String> = columns.iter().map(|e| e.to_string()).collect();

    let width = |s: &str| s.chars().count().max(1).max(truesym.chars().count().max(falsesym.chars().count()));

    let var_widths: Vec<usize> = variables.iter().map(|h| width(h)).collect();
    let col_widths: Vec<usize> = col_headers.iter().map(|h| width(h)).collect();

    let mut header = String::from("│");
    let mut divider = String::from("┼");

    let all_headers = variables.iter().map(String::as_str).zip(var_widths.iter().copied()).chain(col_headers.iter().map(String::as_str).zip(col_widths.iter().copied()));

    for (text, w) in all_headers {
        header.push_str(&format!(" {:^w$} │", text));
        divider.push_str(&"─".repeat(w + 2));
        divider.push('┼');
    }

    divider.pop();
    divider.push('┤');

    let top = divider.replace('┼', "┬").replace('┤', "┐").replacen('┬', "┌", 1);
    let bottom = divider.replace('┼', "┴").replace('┤', "┘").replacen('┴', "└", 1);

    println!("{top}\n{header}\n{divider}");

    let vars_size = variables.len();

    for row_idx in 0..(1 << vars_size) {
        let assignment: BTreeMap<_, _> = variables.iter().enumerate()
            .map(|(i, var)| (var.clone(), ((row_idx >> (vars_size - 1 - i)) & 1) == 1))
            .collect();

        let mut row = String::from("│");

        let var_cells = variables.iter().zip(&var_widths).map(|(var, &w)| (bool2str(*assignment.get(var).unwrap(), truesym, falsesym), w));
        let col_cells = columns.iter().zip(&col_widths).map(|(expr, &w)| (bool2str(expr.evaluate(&assignment), truesym, falsesym), w));

        for (cell, w) in var_cells.chain(col_cells) {
            row.push_str(&format!(" {:^w$} │", cell));
        }

        println!("{row}");
    }

    println!("{bottom}");
}