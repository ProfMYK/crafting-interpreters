use crate::token::*;
use crate::error;

use std::collections::HashMap;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: i32,
    current: i32,
    pub keywords: HashMap<String, TokenType>,
    pub line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut keywords = HashMap::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            keywords,
            line: 1,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, Literal::Nil)
    }

    pub fn add_token_literal(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.source.get(self.start as usize..self.current as usize).unwrap();
        self.tokens.push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    pub fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    pub fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '"' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Underterminated string.".to_string());
            return;
        }

        self.advance();

        let value = self.source.get((self.start + 1) as usize..(self.current - 1) as usize).unwrap().to_string();
        let literal = Literal::String(value);
        self.add_token_literal(TokenType::String, literal);
    }

    pub fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() as i32 {
            return '\0';
        }
        self.source.chars().nth(self.current as usize + 1).unwrap()
    }

    pub fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();
            
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let number: f64 = self.source.get(self.start as usize..self.current as usize).unwrap().to_string().parse().unwrap();
        let literal = Literal::Number(number);
        self.add_token_literal(TokenType::Number, literal);
    }

    pub fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    pub fn is_alphanumeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    pub fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.source.get(self.start as usize..self.current as usize).unwrap().to_string();
        let token_result = self.keywords.get(&text);
        let mut token_type = TokenType::Identifier;
        if token_result != None {
            token_type = *token_result.unwrap();
        }

        if token_type == TokenType::True {
            self.add_token_literal(token_type, Literal::Boolean(true));
            return;
        } else if token_type == TokenType::False {
            self.add_token_literal(token_type, Literal::Boolean(false));
            return;
        }
        self.add_token(token_type);
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            '"' => self.string(),
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            _ => {
                if Scanner::is_digit(c) {
                    self.number();
                }else if Scanner::is_alpha(c) {
                    self.identifier();
                } else {
                    error(self.line, String::from("Unexpected character."))
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::from(""), Literal::Nil, self.line));
        self.tokens.clone()
    }
}