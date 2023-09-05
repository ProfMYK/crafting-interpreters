#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: i32
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: i32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }

    pub fn to_string(&self) -> String {
        match self.literal {
            Literal::String(ref s) => format!("{:?} {} {}", self.token_type, self.lexeme, s),
            Literal::Number(ref n) => format!("{:?} {} {}", self.token_type, self.lexeme, n),
            Literal::Boolean(ref b) => format!("{:?} {} {}", self.token_type, self.lexeme, b),
            Literal::Nil => format!("{:?} {}", self.token_type, self.lexeme)
        }
    }
}