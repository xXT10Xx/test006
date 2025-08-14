use super::tokenizer::{CssTokenizer, CssToken};

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub property: String,
    pub value: String,
    pub important: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Type(String),
    Class(String),
    Id(String),
    Universal,
    Descendant(Box<Selector>, Box<Selector>),
    Child(Box<Selector>, Box<Selector>),
    Adjacent(Box<Selector>, Box<Selector>),
    GeneralSibling(Box<Selector>, Box<Selector>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub struct CssParser {
    tokens: Vec<CssToken>,
    position: usize,
}

impl CssParser {
    pub fn new(input: &str) -> Self {
        let tokenizer = CssTokenizer::new(input);
        let tokens: Vec<CssToken> = tokenizer.filter(|token| !matches!(token, CssToken::Whitespace)).collect();
        
        Self {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> Option<&CssToken> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn parse_selector(&mut self) -> Option<Selector> {
        match self.current_token()? {
            CssToken::Ident(name) => {
                let selector = Selector::Type(name.clone());
                self.advance();
                Some(selector)
            }
            CssToken::Hash(id) => {
                let selector = Selector::Id(id.clone());
                self.advance();
                Some(selector)
            }
            CssToken::Delim('.') => {
                self.advance();
                if let Some(CssToken::Ident(class)) = self.current_token() {
                    let selector = Selector::Class(class.clone());
                    self.advance();
                    Some(selector)
                } else {
                    None
                }
            }
            CssToken::Delim('*') => {
                self.advance();
                Some(Selector::Universal)
            }
            _ => None,
        }
    }

    fn parse_selector_list(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        
        while let Some(selector) = self.parse_selector() {
            selectors.push(selector);
            
            if matches!(self.current_token(), Some(CssToken::Comma)) {
                self.advance(); // Skip comma
            } else {
                break;
            }
        }
        
        selectors
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        if let Some(CssToken::Ident(property)) = self.current_token() {
            let property = property.clone();
            self.advance();
            
            if matches!(self.current_token(), Some(CssToken::Colon)) {
                self.advance(); // Skip colon
                
                let mut value_parts = Vec::new();
                let mut important = false;
                
                while let Some(token) = self.current_token() {
                    match token {
                        CssToken::Semicolon | CssToken::RightBrace => break,
                        CssToken::Delim('!') => {
                            self.advance();
                            if let Some(CssToken::Ident(ident)) = self.current_token() {
                                if ident == "important" {
                                    important = true;
                                    self.advance();
                                }
                            }
                        }
                        CssToken::Ident(s) => {
                            value_parts.push(s.clone());
                            self.advance();
                        }
                        CssToken::String(s) => {
                            value_parts.push(format!("\"{}\"", s));
                            self.advance();
                        }
                        CssToken::Number(n) => {
                            value_parts.push(n.to_string());
                            self.advance();
                        }
                        CssToken::Dimension { value, unit } => {
                            value_parts.push(format!("{}{}", value, unit));
                            self.advance();
                        }
                        CssToken::Percentage(p) => {
                            value_parts.push(format!("{}%", p));
                            self.advance();
                        }
                        CssToken::Hash(h) => {
                            value_parts.push(format!("#{}", h));
                            self.advance();
                        }
                        CssToken::Delim(c) => {
                            value_parts.push(c.to_string());
                            self.advance();
                        }
                        _ => {
                            self.advance();
                        }
                    }
                }
                
                let value = if value_parts.len() == 1 {
                    value_parts[0].clone()
                } else {
                    value_parts.join(" ").trim().to_string()
                };
                
                if matches!(self.current_token(), Some(CssToken::Semicolon)) {
                    self.advance(); // Skip semicolon
                }
                
                Some(Declaration {
                    property,
                    value,
                    important,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_rule(&mut self) -> Option<Rule> {
        let selectors = self.parse_selector_list();
        
        if selectors.is_empty() {
            return None;
        }
        
        if !matches!(self.current_token(), Some(CssToken::LeftBrace)) {
            return None;
        }
        
        self.advance(); // Skip opening brace
        
        let mut declarations = Vec::new();
        
        while !matches!(self.current_token(), Some(CssToken::RightBrace)) && self.position < self.tokens.len() {
            if let Some(declaration) = self.parse_declaration() {
                declarations.push(declaration);
            } else {
                self.advance(); // Skip unknown tokens
            }
        }
        
        if matches!(self.current_token(), Some(CssToken::RightBrace)) {
            self.advance(); // Skip closing brace
        }
        
        Some(Rule {
            selectors,
            declarations,
        })
    }

    pub fn parse(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        
        while self.position < self.tokens.len() {
            if let Some(rule) = self.parse_rule() {
                rules.push(rule);
            } else {
                self.advance(); // Skip unknown tokens
            }
        }
        
        rules
    }
}