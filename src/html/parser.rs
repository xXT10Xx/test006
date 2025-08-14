use super::tokenizer::{HtmlTokenizer, HtmlToken};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
}

pub struct HtmlParser {
    tokens: Vec<HtmlToken>,
    position: usize,
}

impl HtmlParser {
    pub fn new(input: &str) -> Self {
        let tokenizer = HtmlTokenizer::new(input);
        let tokens: Vec<HtmlToken> = tokenizer.collect();
        
        Self {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> Option<&HtmlToken> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn is_void_element(tag_name: &str) -> bool {
        matches!(
            tag_name.to_lowercase().as_str(),
            "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" |
            "link" | "meta" | "param" | "source" | "track" | "wbr"
        )
    }

    fn parse_element(&mut self, start_tag: HtmlToken) -> Option<Node> {
        if let HtmlToken::StartTag { name, attributes, self_closing } = start_tag {
            let mut attr_map = HashMap::new();
            for (key, value) in attributes {
                attr_map.insert(key, value);
            }

            let mut element = Element {
                tag_name: name.clone(),
                attributes: attr_map,
                children: Vec::new(),
            };

            if self_closing || Self::is_void_element(&name) {
                return Some(Node::Element(element));
            }

            while let Some(token) = self.current_token() {
                match token {
                    HtmlToken::EndTag { name: end_name } if end_name == &name => {
                        self.advance();
                        break;
                    }
                    HtmlToken::StartTag { .. } => {
                        if let Some(child) = self.parse_node() {
                            element.children.push(child);
                        }
                    }
                    HtmlToken::Text(text) => {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            element.children.push(Node::Text(trimmed.to_string()));
                        }
                        self.advance();
                    }
                    HtmlToken::Comment(comment) => {
                        element.children.push(Node::Comment(comment.clone()));
                        self.advance();
                    }
                    HtmlToken::EndTag { .. } => {
                        break;
                    }
                    HtmlToken::Doctype(_) => {
                        self.advance();
                    }
                }
            }

            Some(Node::Element(element))
        } else {
            None
        }
    }

    fn parse_node(&mut self) -> Option<Node> {
        match self.current_token()?.clone() {
            HtmlToken::StartTag { .. } => {
                let token = self.current_token()?.clone();
                self.advance();
                self.parse_element(token)
            }
            HtmlToken::Text(text) => {
                self.advance();
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    Some(Node::Text(trimmed.to_string()))
                } else {
                    self.parse_node()
                }
            }
            HtmlToken::Comment(comment) => {
                self.advance();
                Some(Node::Comment(comment))
            }
            HtmlToken::EndTag { .. } => None,
            HtmlToken::Doctype(_) => {
                self.advance();
                self.parse_node()
            }
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.position < self.tokens.len() {
            if let Some(node) = self.parse_node() {
                nodes.push(node);
            }
        }

        nodes
    }

    pub fn parse_document(&mut self) -> Option<Node> {
        let nodes = self.parse();
        
        for node in &nodes {
            if let Node::Element(element) = node {
                if element.tag_name.to_lowercase() == "html" {
                    return Some(node.clone());
                }
            }
        }

        nodes.into_iter().find(|n| matches!(n, Node::Element(_)))
    }
}