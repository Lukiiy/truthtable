use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expression {
    Var(String),
    Not(Box<Expression>), // Invert
    And(Box<Expression>, Box<Expression>), // Conjunction
    Or(Box<Expression>, Box<Expression>), // Disjuction
    Xor(Box<Expression>, Box<Expression>), // Exclusive Disjuction
    Implies(Box<Expression>, Box<Expression>), // Conditional
    Iff(Box<Expression>, Box<Expression>), // Biconditional
}

impl Expression {
    /// Evaluates this expression
    ///
    /// `env` Mapping of variable names to boolean values
    ///
    /// Returns the boolean for the expression
    pub fn evaluate(&self, env: &BTreeMap<String, bool>) -> bool {
        match self {
            Expression::Var(n) => *env.get(n).expect("???"),
            Expression::Not(e) => !e.evaluate(env),
            Expression::And(a, b) => a.evaluate(env) && b.evaluate(env),
            Expression::Or(a, b) => a.evaluate(env) || b.evaluate(env),
            Expression::Xor(a, b) => a.evaluate(env) ^ b.evaluate(env),
            Expression::Implies(a, b) => !a.evaluate(env) || b.evaluate(env),
            Expression::Iff(a, b) => a.evaluate(env) == b.evaluate(env)
        }
    }

    /// Gets all vars used in the expression
    ///
    /// `out` Target set getting discovered var names
    pub fn get_variables(&self, out: &mut BTreeSet<String>) {
        match self {
            Expression::Var(n) => { out.insert(n.clone()); }

            Expression::Not(e) => e.get_variables(out),

            Expression::And(a, b) | Expression::Or(a, b) | Expression::Xor(a, b) | Expression::Implies(a, b) | Expression::Iff(a, b) => {
                a.get_variables(out);
                b.get_variables(out);
            }
        }
    }

    /// Gets non-var subexpressions
    ///
    /// Each is added once based on its display string. The root expression is excluded.
    ///
    /// `collected` List of collected display keys
    /// `out` Collected expressions
    pub fn get_subexpressions<'a>(&'a self, collected: &mut Vec<String>, out: &mut Vec<&'a Expression>) {
        match self {
            Expression::Var(_) => { }

            Expression::Not(e) => e.get_subexpressions(collected, out),

            Expression::And(a, b) | Expression::Or(a, b) | Expression::Xor(a, b) | Expression::Implies(a, b) | Expression::Iff(a, b) => {
                a.get_subexpressions(collected, out);
                b.get_subexpressions(collected, out);
            }
        }

        if !matches!(self, Expression::Var(_)) {
            let key = self.to_string();

            if !collected.contains(&key) {
                collected.push(key);
                out.push(self);
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Var(n) => write!(f, "{n}"),
            Expression::Not(e) => write!(f, "~{e}"),
            Expression::And(a, b) => write!(f, "({a} ∧ {b})"),
            Expression::Or(a, b) => write!(f, "({a} ∨ {b})"),
            Expression::Xor(a, b) => write!(f, "({a} ⊕ {b})"),
            Expression::Implies(a, b) => write!(f, "({a} → {b})"),
            Expression::Iff(a, b) => write!(f, "({a} ↔ {b})"),
        }
    }
}