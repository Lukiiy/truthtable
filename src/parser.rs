use crate::expression::Expression;
use crate::token::Token;
use crate::token::tokenize;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    pub fn new(expression: &str) -> Self {
        Self {
            tokens: tokenize(expression),
            pos: 0
        }
    }

    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }

    fn next(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();

        self.pos += 1;

        t
    }

    pub fn parse(&mut self) -> Expression { self.parse_equiv() }

    fn parse_equiv(&mut self) -> Expression {
        let mut left = self.parse_impl();

        while matches!(self.peek(), Some(Token::Iff)) {
            self.next();

            left = Expression::Iff(Box::new(left), Box::new(self.parse_impl()));
        }

        left
    }

    fn parse_impl(&mut self) -> Expression {
        let left = self.parse_or();

        if matches!(self.peek(), Some(Token::Implies)) {
            self.next();

            Expression::Implies(Box::new(left), Box::new(self.parse_impl()))
        } else { left }
    }

    fn parse_or(&mut self) -> Expression {
        let mut left = self.parse_xor();

        while matches!(self.peek(), Some(Token::Or)) {
            self.next();

            left = Expression::Or(Box::new(left), Box::new(self.parse_xor()));
        }

        left
    }

    fn parse_xor(&mut self) -> Expression {
        let mut left = self.parse_and();

        while matches!(self.peek(), Some(Token::Xor)) {
            self.next();

            left = Expression::Xor(Box::new(left), Box::new(self.parse_and()));
        }

        left
    }

    fn parse_and(&mut self) -> Expression {
        let mut left = self.parse_not();

        while matches!(self.peek(), Some(Token::And)) {
            self.next();

            left = Expression::And(Box::new(left), Box::new(self.parse_not()));
        }

        left
    }

    fn parse_not(&mut self) -> Expression {
        if matches!(self.peek(), Some(Token::Not)) {
            self.next();

            Expression::Not(Box::new(self.parse_not()))
        } else {
            self.parse_primary()
        }
    }

    /// parenthesized expression/id
    fn parse_primary(&mut self) -> Expression {
        match self.peek() {
            Some(Token::LParen) => {
                self.next();

                let e = self.parse();

                if matches!(self.peek(), Some(Token::RParen)) { self.next(); }

                e
            }

            Some(Token::Ident(_)) => {
                if let Token::Ident(name) = self.next() {
                    Expression::Var(name)
                } else { unreachable!() }
            }

            other => panic!("Unexpected token: {other:?}")
        }
    }
}