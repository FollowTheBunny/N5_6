#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Real(f64),
    Variable(String),
    To,
    Assing,
    Define,
    For,
    Term,
    Begin,
    End,
    Print,
    Pow,
    Plus,
    Minus,
    Asterisk,
    Slash,
    IntegerDivide,
    LeftParen,
    RightParen,
    Bad,
    Whitespace,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            let eof_char: char = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, eof_char.to_string()),
            ));
        }
        let c = self.current_char();
        return c.map(|c| {
            let start = self.current_pos;
            let mut kind = TokenKind::Bad;
            if Self::is_number_start(&c) {
                let number: f64 = self.consume_number();
                kind = TokenKind::Real(number);
            } else if Self::is_whitespace(&c) {
                self.consume();
                kind = TokenKind::Whitespace;
            } else if c.is_alphabetic() {
                let mut variable_name = String::new();
                while let Some(c) = self.current_char() {
                    if c.is_alphanumeric() {
                        self.consume().unwrap();
                        variable_name.push(c);
                    } else {
                        break;
                    }
                }
                match variable_name.as_str() {
                    "for" => kind = TokenKind::For,
                    "print" => kind = TokenKind::Print,
                    "var" => kind = TokenKind::Define,
                    _ => kind = TokenKind::Variable(variable_name),
                }
            } else {
                kind = self.consume_punctuation();
            }

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            Token::new(kind, span)
        });
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '=' => TokenKind::Assing,
            ';' => TokenKind::Term,
            '/' => {
                if let Some(next_char) = self.current_char() {
                    if next_char == '/' {
                        self.consume();
                        TokenKind::IntegerDivide
                    } else {
                        TokenKind::Slash
                    }
                } else {
                    TokenKind::Slash
                }
            }
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::Begin,
            '}' => TokenKind::End,
            '^' => TokenKind::Pow,
            _ => TokenKind::Bad,
        }
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        c
    }
    fn consume_number(&mut self) -> f64 {
        let mut number_str = String::new();
        let mut is_whole_number = true;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number_str.push(c);
            } else if c == '.' {
                self.consume().unwrap();
                number_str.push(c);
                is_whole_number = false;
            } else {
                break;
            }
        }
        if is_whole_number && !number_str.is_empty() {
            number_str.push_str(".0");
        }
        number_str.parse::<f64>().unwrap_or(0.0)
    }
}
