#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
    Ident(String),
    String(String),
    Number(f64),
    Dimension { value: f64, unit: String },
    Percentage(f64),
    Hash(String),
    Delim(char),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semicolon,
    Whitespace,
    Comment(String),
    AtKeyword(String),
}

pub struct CssTokenizer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> CssTokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Self {
            input,
            position: 0,
            current_char: None,
        };
        tokenizer.current_char = tokenizer.input.chars().next();
        tokenizer
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += self.current_char.map_or(0, |c| c.len_utf8());
            self.current_char = self.input[self.position..].chars().next();
        } else {
            self.current_char = None;
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            self.input[self.position..].chars().nth(1)
        } else {
            None
        }
    }

    fn consume_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if predicate(ch) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    fn parse_string(&mut self, quote: char) -> String {
        let mut result = String::new();
        self.advance(); // Skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance(); // Skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    result.push(escaped);
                    self.advance();
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }
        
        result
    }

    fn parse_number(&mut self) -> f64 {
        let number_str = self.consume_while(|c| c.is_ascii_digit() || c == '.');
        number_str.parse().unwrap_or(0.0)
    }

    fn parse_ident(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }

    fn parse_comment(&mut self) -> String {
        let mut comment = String::new();
        self.advance(); // Skip '/'
        self.advance(); // Skip '*'
        
        while let Some(ch) = self.current_char {
            if ch == '*' && self.peek() == Some('/') {
                self.advance(); // Skip '*'
                self.advance(); // Skip '/'
                break;
            } else {
                comment.push(ch);
                self.advance();
            }
        }
        
        comment
    }

    pub fn next_token(&mut self) -> Option<CssToken> {
        match self.current_char? {
            ' ' | '\t' | '\n' | '\r' => {
                self.consume_while(|c| c.is_whitespace());
                Some(CssToken::Whitespace)
            }
            '/' if self.peek() == Some('*') => {
                let comment = self.parse_comment();
                Some(CssToken::Comment(comment))
            }
            '"' => {
                let string = self.parse_string('"');
                Some(CssToken::String(string))
            }
            '\'' => {
                let string = self.parse_string('\'');
                Some(CssToken::String(string))
            }
            '#' => {
                self.advance(); // Skip '#'
                let hash = self.parse_ident();
                Some(CssToken::Hash(hash))
            }
            '@' => {
                self.advance(); // Skip '@'
                let keyword = self.parse_ident();
                Some(CssToken::AtKeyword(keyword))
            }
            '(' => {
                self.advance();
                Some(CssToken::LeftParen)
            }
            ')' => {
                self.advance();
                Some(CssToken::RightParen)
            }
            '{' => {
                self.advance();
                Some(CssToken::LeftBrace)
            }
            '}' => {
                self.advance();
                Some(CssToken::RightBrace)
            }
            '[' => {
                self.advance();
                Some(CssToken::LeftBracket)
            }
            ']' => {
                self.advance();
                Some(CssToken::RightBracket)
            }
            ',' => {
                self.advance();
                Some(CssToken::Comma)
            }
            ':' => {
                self.advance();
                Some(CssToken::Colon)
            }
            ';' => {
                self.advance();
                Some(CssToken::Semicolon)
            }
            ch if ch.is_ascii_digit() => {
                let number = self.parse_number();
                
                if self.current_char == Some('%') {
                    self.advance();
                    Some(CssToken::Percentage(number))
                } else if let Some(ch) = self.current_char {
                    if ch.is_alphabetic() {
                        let unit = self.parse_ident();
                        Some(CssToken::Dimension { value: number, unit })
                    } else {
                        Some(CssToken::Number(number))
                    }
                } else {
                    Some(CssToken::Number(number))
                }
            }
            ch if ch.is_alphabetic() || ch == '-' || ch == '_' => {
                let ident = self.parse_ident();
                Some(CssToken::Ident(ident))
            }
            ch => {
                self.advance();
                Some(CssToken::Delim(ch))
            }
        }
    }
}

impl<'a> Iterator for CssTokenizer<'a> {
    type Item = CssToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}