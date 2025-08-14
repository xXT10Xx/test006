#[derive(Debug, Clone, PartialEq)]
pub enum HtmlToken {
    StartTag {
        name: String,
        attributes: Vec<(String, String)>,
        self_closing: bool,
    },
    EndTag {
        name: String,
    },
    Text(String),
    Comment(String),
    Doctype(String),
}

pub struct HtmlTokenizer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> HtmlTokenizer<'a> {
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

    fn skip_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }

    fn parse_attribute_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ':')
    }

    fn parse_attribute_value(&mut self) -> String {
        self.skip_whitespace();
        
        if self.current_char == Some('"') {
            self.advance(); // Skip opening quote
            let value = self.consume_while(|c| c != '"');
            if self.current_char == Some('"') {
                self.advance(); // Skip closing quote
            }
            value
        } else if self.current_char == Some('\'') {
            self.advance(); // Skip opening quote
            let value = self.consume_while(|c| c != '\'');
            if self.current_char == Some('\'') {
                self.advance(); // Skip closing quote
            }
            value
        } else {
            self.consume_while(|c| !c.is_whitespace() && c != '>' && c != '/')
        }
    }

    fn parse_attributes(&mut self) -> Vec<(String, String)> {
        let mut attributes = Vec::new();
        
        while let Some(ch) = self.current_char {
            if ch == '>' || ch == '/' {
                break;
            }
            
            self.skip_whitespace();
            
            if self.current_char.is_none() || self.current_char == Some('>') || self.current_char == Some('/') {
                break;
            }
            
            let name = self.parse_attribute_name();
            if name.is_empty() {
                break;
            }
            
            self.skip_whitespace();
            
            let value = if self.current_char == Some('=') {
                self.advance(); // Skip '='
                self.parse_attribute_value()
            } else {
                String::new()
            };
            
            attributes.push((name, value));
        }
        
        attributes
    }

    fn parse_comment(&mut self) -> String {
        let mut comment = String::new();
        
        while let Some(ch) = self.current_char {
            if ch == '-' && self.peek() == Some('-') {
                self.advance(); // Skip first '-'
                self.advance(); // Skip second '-'
                if self.current_char == Some('>') {
                    self.advance(); // Skip '>'
                    break;
                }
                comment.push_str("--");
            } else {
                comment.push(ch);
                self.advance();
            }
        }
        
        comment
    }

    fn parse_doctype(&mut self) -> String {
        self.consume_while(|c| c != '>')
    }

    pub fn next_token(&mut self) -> Option<HtmlToken> {
        self.skip_whitespace();
        
        match self.current_char? {
            '<' => {
                self.advance(); // Skip '<'
                
                if self.current_char == Some('!') {
                    self.advance(); // Skip '!'
                    
                    if self.current_char == Some('-') && self.peek() == Some('-') {
                        self.advance(); // Skip first '-'
                        self.advance(); // Skip second '-'
                        let comment = self.parse_comment();
                        Some(HtmlToken::Comment(comment))
                    } else {
                        let doctype = self.parse_doctype();
                        if self.current_char == Some('>') {
                            self.advance(); // Skip '>'
                        }
                        Some(HtmlToken::Doctype(doctype))
                    }
                } else if self.current_char == Some('/') {
                    self.advance(); // Skip '/'
                    let name = self.parse_tag_name();
                    self.skip_whitespace();
                    if self.current_char == Some('>') {
                        self.advance(); // Skip '>'
                    }
                    Some(HtmlToken::EndTag { name })
                } else {
                    let name = self.parse_tag_name();
                    let attributes = self.parse_attributes();
                    
                    let mut self_closing = false;
                    if self.current_char == Some('/') {
                        self_closing = true;
                        self.advance(); // Skip '/'
                    }
                    
                    if self.current_char == Some('>') {
                        self.advance(); // Skip '>'
                    }
                    
                    Some(HtmlToken::StartTag {
                        name,
                        attributes,
                        self_closing,
                    })
                }
            }
            _ => {
                let text = self.consume_while(|c| c != '<');
                if !text.is_empty() {
                    Some(HtmlToken::Text(text))
                } else {
                    None
                }
            }
        }
    }
}

impl<'a> Iterator for HtmlTokenizer<'a> {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}