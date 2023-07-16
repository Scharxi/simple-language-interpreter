use self::token::Token;

pub mod token;

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut pos = 0;
    let mut res: Vec<Token> = Vec::new();

    while pos < src.len() {
        let current = src.chars().nth(pos).unwrap();
        match current {
            '=' => res.push(Token {
                t_type: token::TokenType::Equal,
                lexeme: '='.to_string(),
            }),
            '/' => res.push(Token {
                t_type: token::TokenType::Slash,
                lexeme: '/'.to_string(),
            }),
            '+' => res.push(Token {
                t_type: token::TokenType::Plus,
                lexeme: '+'.to_string(),
            }),
            '-' => res.push(Token {
                t_type: token::TokenType::Minus,
                lexeme: '-'.to_string(),
            }),
            '*' => res.push(Token {
                t_type: token::TokenType::Star,
                lexeme: '*'.to_string(),
            }),
            '(' => res.push(Token {
                t_type: token::TokenType::LeftParen,
                lexeme: '('.to_string(),
            }),
            ')' => res.push(Token {
                t_type: token::TokenType::RightParen,
                lexeme: ')'.to_string(),
            }),
            x if x.is_digit(10) => {
                let mut number_lexeme = x.to_string();

                pos += 1;

                while pos < src.len() {
                    let next_char = src.chars().nth(pos).unwrap();

                    if next_char == ' ' || next_char == ')' || next_char == '\n' {
                        break;
                    }

                    if next_char.is_digit(10) {
                        number_lexeme.push(next_char);
                    } else {
                        panic!("Encountered invalid character: '{next_char}'");
                    }

                    pos += 1;
                }

                res.push(Token {
                    t_type: token::TokenType::NumberLiteral,
                    lexeme: number_lexeme,
                });
                continue; // we don't want to consume the last character
            }
            '\n' => res.push(Token {
                t_type: token::TokenType::Newline,
                lexeme: '\n'.to_string(),
            }),
            ' ' => {}
            ident => {
                // identifier
                let mut lexeme = ident.to_string();

                pos += 1;

                while pos < src.len() {
                    let next_char = src.chars().nth(pos).unwrap();
                    if !is_valid_identifier_char(next_char) {
                        break;
                    }

                    lexeme.push(next_char);
                    pos += 1;
                }

                res.push(Token {
                    t_type: token::TokenType::Ident,
                    lexeme,
                });
                continue;
            }
        }
        pos += 1;
    }

    res
}

#[inline]
fn is_valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
