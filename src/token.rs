#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Not,
    And,
    Or,
    Xor,
    Implies, // ->
    Iff,     // <->
    LParen,
    RParen
}

/// Converts a string into logical tokens.
///
/// Normalizes operator styles and keywords into a small set of tokens used by the parser.
///
/// `src` Expression as string
/// Returns a list of normalized tokens
pub fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut it = expression.chars().peekable();

    while let Some(&c) = it.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => { it.next(); }

            '(' | '[' | '{' => {
                it.next();
                tokens.push(Token::LParen);
            }

            ')' | ']' | '}' => {
                it.next();
                tokens.push(Token::RParen);
            }

            '!' | '~' => {
                it.next();
                tokens.push(Token::Not);
            }

            '^' | '%' => {
                it.next();
                tokens.push(Token::Xor);
            }

            '&' => {
                it.next();

                if it.peek() == Some(&'&') { it.next(); } // remove extra &

                tokens.push(Token::And);
            }

            '|' => {
                it.next();

                if it.peek() == Some(&'|') { it.next(); } // remove extra |

                tokens.push(Token::Or);
            }

            '-' => { // ->
                it.next();

                if it.peek() == Some(&'>') {
                    it.next();
                    tokens.push(Token::Implies);
                }
            }

            '<' => { // <-> or <=>
                it.next();

                if it.peek() == Some(&'-') {
                    it.next();

                    if it.peek() == Some(&'>') { it.next(); }

                    tokens.push(Token::Iff);
                } else if it.peek() == Some(&'=') {
                    it.next();

                    if it.peek() == Some(&'>') { it.next(); }

                    tokens.push(Token::Iff);
                }
            }

            '=' => {
                it.next();

                if it.peek() == Some(&'>') {
                    it.next();
                    tokens.push(Token::Implies);
                }
            }

            // identifiers / keywords: letters or '_' start, then alnum or '_'
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut word = String::new();

                while let Some(&ch) = it.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        word.push(ch);
                        it.next();
                    } else { break; }
                }

                match word.to_lowercase().as_str() {
                    "and" => tokens.push(Token::And),
                    "or" | "v" => tokens.push(Token::Or),
                    "not" => tokens.push(Token::Not),
                    "xor" => tokens.push(Token::Xor),
                    "impl" | "implies" => tokens.push(Token::Implies),
                    "iff" => tokens.push(Token::Iff),
                    other => tokens.push(Token::Ident(other.to_string()))
                }
            }

            _ => { it.next(); } // skip unknown chars
        }
    }

    tokens
}