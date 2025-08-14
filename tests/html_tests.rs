use html_css_parser::html::{HtmlTokenizer, HtmlParser, HtmlToken, Node, Element};
use std::collections::HashMap;

#[test]
fn test_html_tokenizer_basic() {
    let html = "<div>Hello</div>";
    let mut tokenizer = HtmlTokenizer::new(html);
    
    let tokens: Vec<HtmlToken> = tokenizer.collect();
    
    assert_eq!(tokens.len(), 3);
    
    match &tokens[0] {
        HtmlToken::StartTag { name, attributes, self_closing } => {
            assert_eq!(name, "div");
            assert!(attributes.is_empty());
            assert!(!self_closing);
        }
        _ => panic!("Expected StartTag"),
    }
    
    match &tokens[1] {
        HtmlToken::Text(text) => assert_eq!(text, "Hello"),
        _ => panic!("Expected Text"),
    }
    
    match &tokens[2] {
        HtmlToken::EndTag { name } => assert_eq!(name, "div"),
        _ => panic!("Expected EndTag"),
    }
}

#[test]
fn test_html_tokenizer_attributes() {
    let html = r#"<div class="container" id="main" data-value="test">Content</div>"#;
    let mut tokenizer = HtmlTokenizer::new(html);
    
    let tokens: Vec<HtmlToken> = tokenizer.collect();
    
    match &tokens[0] {
        HtmlToken::StartTag { name, attributes, self_closing } => {
            assert_eq!(name, "div");
            assert_eq!(attributes.len(), 3);
            assert_eq!(attributes[0], ("class".to_string(), "container".to_string()));
            assert_eq!(attributes[1], ("id".to_string(), "main".to_string()));
            assert_eq!(attributes[2], ("data-value".to_string(), "test".to_string()));
            assert!(!self_closing);
        }
        _ => panic!("Expected StartTag with attributes"),
    }
}

#[test]
fn test_html_tokenizer_self_closing() {
    let html = r#"<img src="test.jpg" alt="Test" />"#;
    let mut tokenizer = HtmlTokenizer::new(html);
    
    let tokens: Vec<HtmlToken> = tokenizer.collect();
    
    assert_eq!(tokens.len(), 1);
    
    match &tokens[0] {
        HtmlToken::StartTag { name, attributes, self_closing } => {
            assert_eq!(name, "img");
            assert_eq!(attributes.len(), 2);
            assert!(self_closing);
        }
        _ => panic!("Expected self-closing StartTag"),
    }
}

#[test]
fn test_html_tokenizer_comment() {
    let html = "<!-- This is a comment --><div>Content</div>";
    let mut tokenizer = HtmlTokenizer::new(html);
    
    let tokens: Vec<HtmlToken> = tokenizer.collect();
    
    match &tokens[0] {
        HtmlToken::Comment(comment) => assert_eq!(comment, " This is a comment "),
        _ => panic!("Expected Comment"),
    }
}

#[test]
fn test_html_tokenizer_doctype() {
    let html = "<!DOCTYPE html><html></html>";
    let mut tokenizer = HtmlTokenizer::new(html);
    
    let tokens: Vec<HtmlToken> = tokenizer.collect();
    
    match &tokens[0] {
        HtmlToken::Doctype(doctype) => assert_eq!(doctype, "DOCTYPE html"),
        _ => panic!("Expected Doctype"),
    }
}

#[test]
fn test_html_parser_simple() {
    let html = "<div>Hello World</div>";
    let mut parser = HtmlParser::new(html);
    
    let nodes = parser.parse();
    
    assert_eq!(nodes.len(), 1);
    
    match &nodes[0] {
        Node::Element(element) => {
            assert_eq!(element.tag_name, "div");
            assert_eq!(element.children.len(), 1);
            
            match &element.children[0] {
                Node::Text(text) => assert_eq!(text, "Hello World"),
                _ => panic!("Expected text node"),
            }
        }
        _ => panic!("Expected element node"),
    }
}

#[test]
fn test_html_parser_nested() {
    let html = r#"<div class="container"><h1>Title</h1><p>Paragraph</p></div>"#;
    let mut parser = HtmlParser::new(html);
    
    let nodes = parser.parse();
    
    assert_eq!(nodes.len(), 1);
    
    match &nodes[0] {
        Node::Element(element) => {
            assert_eq!(element.tag_name, "div");
            assert_eq!(element.attributes.get("class"), Some(&"container".to_string()));
            assert_eq!(element.children.len(), 2);
            
            match &element.children[0] {
                Node::Element(h1) => {
                    assert_eq!(h1.tag_name, "h1");
                    assert_eq!(h1.children.len(), 1);
                    
                    match &h1.children[0] {
                        Node::Text(text) => assert_eq!(text, "Title"),
                        _ => panic!("Expected text in h1"),
                    }
                }
                _ => panic!("Expected h1 element"),
            }
            
            match &element.children[1] {
                Node::Element(p) => {
                    assert_eq!(p.tag_name, "p");
                    assert_eq!(p.children.len(), 1);
                    
                    match &p.children[0] {
                        Node::Text(text) => assert_eq!(text, "Paragraph"),
                        _ => panic!("Expected text in p"),
                    }
                }
                _ => panic!("Expected p element"),
            }
        }
        _ => panic!("Expected div element"),
    }
}

#[test]
fn test_html_parser_void_elements() {
    let html = r#"<div><img src="test.jpg"><br><hr></div>"#;
    let mut parser = HtmlParser::new(html);
    
    let nodes = parser.parse();
    
    match &nodes[0] {
        Node::Element(div) => {
            assert_eq!(div.children.len(), 3);
            
            for child in &div.children {
                match child {
                    Node::Element(element) => {
                        assert!(matches!(element.tag_name.as_str(), "img" | "br" | "hr"));
                        assert!(element.children.is_empty());
                    }
                    _ => panic!("Expected element nodes for void elements"),
                }
            }
        }
        _ => panic!("Expected div element"),
    }
}

#[test]
fn test_html_parser_document() {
    let html = r#"<!DOCTYPE html><html><head><title>Test</title></head><body><h1>Hello</h1></body></html>"#;
    let mut parser = HtmlParser::new(html);
    
    let document = parser.parse_document();
    
    assert!(document.is_some());
    
    match document.unwrap() {
        Node::Element(html_element) => {
            assert_eq!(html_element.tag_name, "html");
            assert_eq!(html_element.children.len(), 2);
            
            match &html_element.children[0] {
                Node::Element(head) => {
                    assert_eq!(head.tag_name, "head");
                    assert_eq!(head.children.len(), 1);
                    
                    match &head.children[0] {
                        Node::Element(title) => {
                            assert_eq!(title.tag_name, "title");
                            assert_eq!(title.children.len(), 1);
                        }
                        _ => panic!("Expected title element"),
                    }
                }
                _ => panic!("Expected head element"),
            }
            
            match &html_element.children[1] {
                Node::Element(body) => {
                    assert_eq!(body.tag_name, "body");
                    assert_eq!(body.children.len(), 1);
                }
                _ => panic!("Expected body element"),
            }
        }
        _ => panic!("Expected html element"),
    }
}