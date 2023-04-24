use std::{ops::Range, str::Chars};

/// An enum listing all tokens the lexer can encounter
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Dot,
    OpeningParenthesis,
    ClosingParenthesis,
    Equals,
    Space,
    Less,
    Greater,

    // Two-characters tokens
    LessEqual,
    GreaterEqual,
    NotEqual,

    // Function call like sin.
    Function(String),

    // Values.
    Constant(f64),
    Variable(String),

    EndOfExpression,
}

pub type LexerError = String;

/// A full token representation.
pub struct Token {
    pub token_type: TokenType,
    pub span: Range<usize>,
}

impl Token {
    pub fn new(token_type: TokenType, span: Range<usize>) -> Self {
        Self { token_type, span }
    }

    pub fn some_token(token_type: TokenType, span: Range<usize>) -> Result<Option<Self>, LexerError> {
        Ok(Some(Self::new(token_type, span)))
    }
}

pub struct Lexer<'a> {
    full_str: &'a str,
    source: Vec<char>,
    index: usize,
    tokens_list: Vec<Token>,
}

impl<'a> Lexer<'a> {
    /// Creates a new and operational Lexer.
    pub fn new(source: &'a str) -> Self {
        Self {
            full_str: source,
            source: source.chars().collect(),
            index: 0,
            tokens_list: Vec::new(),
        }
    }

    /// Returns the next character of the source and advances by one
    pub fn next(&mut self) -> Option<char> {
        self.index += 1;

        if self.index > self.source.len() {
            None
        }
        else {
            Some(self.source[self.index - 1])
        }
    }

    /// Returns the next character of the source without advancing the lexer
    pub fn peek(&mut self) -> Option<char> {
        if self.index >= self.source.len() {
            None
        }
        else {
            Some(self.source[self.index])
        }
    }

    pub fn add_singlechar_token(&mut self, token_type: TokenType) {
        self.tokens_list
            .push(Token::new(token_type, (self.index - 1)..self.index));
    }

    /// Scans the next token and returns it.
    pub fn scan_token(&mut self) -> Result<Option<Token>, String> {
        let span_start = self.index + 1;

        while let Some(character) = self.next() {
            let single_char_span = self.index - 1 .. self.index;

            match character {
                // One-character tokens
                ' ' => return Token::some_token(TokenType::Space, single_char_span),
                '+' => return Token::some_token(TokenType::Plus, single_char_span),
                '-' => return Token::some_token(TokenType::Minus, single_char_span),
                '*' => return Token::some_token(TokenType::Multiply, single_char_span),
                '/' => return Token::some_token(TokenType::Divide, single_char_span),
                '^' => return Token::some_token(TokenType::Power, single_char_span),
                '.' => return Token::some_token(TokenType::Dot, single_char_span),
                '(' => return Token::some_token(TokenType::OpeningParenthesis, single_char_span),
                ')' => return Token::some_token(TokenType::ClosingParenthesis, single_char_span),
                '=' => return Token::some_token(TokenType::Equals, single_char_span),
                
                // Two-characters tokens
                '>' => {
                    if let Some(next) = self.peek() && next == '=' {
                        let _ = self.next();
                        return Token::some_token(TokenType::GreaterEqual, span_start..self.index + 1);
                    }
                    else {
                        return Ok(Some(Token::new(TokenType::Greater, single_char_span)));
                    }
                },

                '<' => {
                    if let Some(next) = self.peek() && next == '=' {
                        let _ = self.next();
                        return Token::some_token(TokenType::LessEqual, span_start..self.index + 1);
                    }
                    else {
                        return Token::some_token(TokenType::Less, single_char_span);
                    }
                },

                '!' => {
                    if let Some(next) = self.peek() {
                        if next == '=' {
                            let _ = self.next();
                            return Token::some_token(TokenType::NotEqual, span_start..self.index + 1);
                        }
                        else {
                            return Err(format!("Bang is not supposed to be combinated with something else than = at position {}, currently {}", self.index - 1, next));
                        }
                        
                    }
                },

                // Parsing constants
                '0'..='9' => {
                    let span_start = self.index - 1;
                    let mut span_end = self.index;
                    loop {
                        match self.peek() {
                            Some('0'..='9') | Some('.') | Some('_') => span_end += 1,
                            _ => {
                                let parsed_value = self.full_str[span_start..span_end].parse::<f64>();
                                if parsed_value.is_err() {
                                    let string = self.full_str[span_start..span_end].to_owned();
                                    return Err(format!("Bad-formatted number starting at position {}: {}", span_start, string));
                                }
                                return Token::some_token(TokenType::Constant(parsed_value.unwrap()), span_start..span_end);
                            }
                        }
                        let _ = self.next();
                    }
                }
                // End parsing constants

                // Parsing variables
                'a'..='z' | 'A'..='Z' => {
                    let span_start = self.index - 1;
                    let mut span_end = self.index;
                    loop {
                        match self.peek() {
                            Some('a'..='z') | Some('A'..='Z') => span_end += 1,
                            _ => {
                                let variable_name = self.full_str[span_start..span_end].to_owned();
                                return Token::some_token(TokenType::Variable(variable_name), span_start..span_end);
                            }
                        }
                        self.next();
                    }
                }
                _ => {
                    return Err(format!(
                        "Unexpected character at position {}: {}",
                        self.index - 1,
                        character
                    ))
                }
            }
        }
        Ok(Some(Token::new(TokenType::EndOfExpression, self.index - 1 .. self.index)))
    }
}
