#![allow(dead_code)]

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{stdin, stdout, Error, Read, Write};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 2 {
        lox.run_file(&args[0])?;
    } else {
        lox.run_prompt()?;
    }
    Ok(())
}

struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.run(&contents);
        if self.had_error {
            eprintln!("failed to execute the given file");
            // exit here
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), Error> {
        loop {
            print!("> ");
            stdout().flush().unwrap();
            let mut line = String::new();
            stdin().read_line(&mut line)?;
            self.run(&line);
            self.had_error = false;
        }
    }

    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source.into());
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(line: u32, message: String) {
        eprintln!("[line {}] Error: {}", line, message);
        //self.report(line, "".into(), message);
    }

    pub fn report(&mut self, line: u32, where_in_code: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, where_in_code, message);
        self.had_error = true;
    }
}
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert("and".into(), TokenType::And);
        keywords.insert("class".into(), TokenType::Class);
        keywords.insert("else".into(), TokenType::Else);
        keywords.insert("false".into(), TokenType::False);
        keywords.insert("for".into(), TokenType::For);
        keywords.insert("fun".into(), TokenType::Fun);
        keywords.insert("if".into(), TokenType::If);
        keywords.insert("nil".into(), TokenType::Nil);
        keywords.insert("or".into(), TokenType::Or);
        keywords.insert("print".into(), TokenType::Print);
        keywords.insert("return".into(), TokenType::Return);
        keywords.insert("super".into(), TokenType::Super);
        keywords.insert("this".into(), TokenType::This);
        keywords.insert("true".into(), TokenType::True);
        keywords.insert("var".into(), TokenType::Var);
        keywords.insert("while".into(), TokenType::While);
        Scanner {
            source,
            tokens: Vec::new().into(),
            keywords: keywords,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".into(), "".into(), self.line));

        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

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
                self.add_token(if self.match_token('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                });
            }
            '=' => {
                self.add_token(if self.match_token('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                });
            }
            '<' => {
                self.add_token(if self.match_token('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                });
            }
            '>' => {
                self.add_token(if self.match_token('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                });
            }
            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                };
            }
            ' ' | '\r' | '\t' => {
                // ignore
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            'o' => {
                if self.match_token('r') {
                    self.add_token(TokenType::Or);
                }
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    eprintln!("unrecognizable token {}", c);
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].into();
        let token_type = self.keywords.get(&text).unwrap_or(&TokenType::Identifier);
        self.add_token(token_type.clone());
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let number: String = self.source[self.start..self.current].into();

        self.add_token_with_literal(TokenType::Number, number);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Lox::error(self.line, "Unterminated string.".into());
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1].into();
        self.add_token_with_literal(TokenType::String, value);
    }

    fn match_token(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        // TODO handle outOfBound
        let ret = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ret
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, "".into())
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.into(), literal, self.line));
    }
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{} {} {}", self.token_type, self.lexeme ,self.literal)
        write!(f, "{}", self.lexeme)
    }
}

pub enum Literals {


}

#[derive(Clone, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}