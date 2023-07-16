#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    NumberLiteral,
    Ident,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
}
