use html_css_parser::css::{CssTokenizer, CssParser, CssToken, Rule, Selector, Declaration};

#[test]
fn test_css_tokenizer_basic() {
    let css = "body { color: red; }";
    let tokenizer = CssTokenizer::new(css);
    
    let tokens: Vec<CssToken> = tokenizer.collect();
    
    assert!(tokens.len() > 5);
    
    assert_eq!(tokens[0], CssToken::Ident("body".to_string()));
    assert_eq!(tokens[1], CssToken::Whitespace);
    assert_eq!(tokens[2], CssToken::LeftBrace);
    assert_eq!(tokens[3], CssToken::Whitespace);
    assert_eq!(tokens[4], CssToken::Ident("color".to_string()));
    assert_eq!(tokens[5], CssToken::Colon);
}

#[test]
fn test_css_tokenizer_numbers() {
    let css = "width: 100px; height: 50%; opacity: 0.5;";
    let tokenizer = CssTokenizer::new(css);
    
    let tokens: Vec<CssToken> = tokenizer.collect();
    
    let dimension_token = tokens.iter().find(|t| matches!(t, CssToken::Dimension { .. }));
    assert!(dimension_token.is_some());
    
    if let Some(CssToken::Dimension { value, unit }) = dimension_token {
        assert_eq!(*value, 100.0);
        assert_eq!(unit, "px");
    }
    
    let percentage_token = tokens.iter().find(|t| matches!(t, CssToken::Percentage(_)));
    assert!(percentage_token.is_some());
    
    if let Some(CssToken::Percentage(value)) = percentage_token {
        assert_eq!(*value, 50.0);
    }
    
    let number_token = tokens.iter().find(|t| matches!(t, CssToken::Number(_)));
    assert!(number_token.is_some());
    
    if let Some(CssToken::Number(value)) = number_token {
        assert_eq!(*value, 0.5);
    }
}

#[test]
fn test_css_tokenizer_strings() {
    let css = r#"content: "Hello World"; font-family: 'Arial';"#;
    let tokenizer = CssTokenizer::new(css);
    
    let tokens: Vec<CssToken> = tokenizer.collect();
    
    let string_tokens: Vec<&CssToken> = tokens.iter().filter(|t| matches!(t, CssToken::String(_))).collect();
    assert_eq!(string_tokens.len(), 2);
    
    if let CssToken::String(s) = &string_tokens[0] {
        assert_eq!(s, "Hello World");
    }
    
    if let CssToken::String(s) = &string_tokens[1] {
        assert_eq!(s, "Arial");
    }
}

#[test]
fn test_css_tokenizer_hash() {
    let css = "color: #ff0000; background: #abc;";
    let tokenizer = CssTokenizer::new(css);
    
    let tokens: Vec<CssToken> = tokenizer.collect();
    
    let hash_tokens: Vec<&CssToken> = tokens.iter().filter(|t| matches!(t, CssToken::Hash(_))).collect();
    assert_eq!(hash_tokens.len(), 2);
    
    if let CssToken::Hash(h) = &hash_tokens[0] {
        assert_eq!(h, "ff0000");
    }
    
    if let CssToken::Hash(h) = &hash_tokens[1] {
        assert_eq!(h, "abc");
    }
}

#[test]
fn test_css_tokenizer_comment() {
    let css = "/* This is a comment */ body { color: red; }";
    let tokenizer = CssTokenizer::new(css);
    
    let tokens: Vec<CssToken> = tokenizer.collect();
    
    let comment_token = tokens.iter().find(|t| matches!(t, CssToken::Comment(_)));
    assert!(comment_token.is_some());
    
    if let Some(CssToken::Comment(comment)) = comment_token {
        assert_eq!(comment, " This is a comment ");
    }
}

#[test]
fn test_css_tokenizer_at_keyword() {
    let css = "@media screen { body { color: red; } }";
    let tokenizer = CssTokenizer::new(css);
    
    let tokens: Vec<CssToken> = tokenizer.collect();
    
    let at_keyword_token = tokens.iter().find(|t| matches!(t, CssToken::AtKeyword(_)));
    assert!(at_keyword_token.is_some());
    
    if let Some(CssToken::AtKeyword(keyword)) = at_keyword_token {
        assert_eq!(keyword, "media");
    }
}

#[test]
fn test_css_parser_simple_rule() {
    let css = "body { color: red; font-size: 16px; }";
    let mut parser = CssParser::new(css);
    
    let rules = parser.parse();
    
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.selectors.len(), 1);
    
    match &rule.selectors[0] {
        Selector::Type(name) => assert_eq!(name, "body"),
        _ => panic!("Expected type selector"),
    }
    
    assert_eq!(rule.declarations.len(), 2);
    
    let color_decl = &rule.declarations[0];
    assert_eq!(color_decl.property, "color");
    assert_eq!(color_decl.value, "red");
    assert!(!color_decl.important);
    
    let font_size_decl = &rule.declarations[1];
    assert_eq!(font_size_decl.property, "font-size");
    assert_eq!(font_size_decl.value, "16px");
    assert!(!font_size_decl.important);
}

#[test]
fn test_css_parser_class_selector() {
    let css = ".container { margin: 0 auto; }";
    let mut parser = CssParser::new(css);
    
    let rules = parser.parse();
    
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.selectors.len(), 1);
    
    match &rule.selectors[0] {
        Selector::Class(name) => assert_eq!(name, "container"),
        _ => panic!("Expected class selector"),
    }
}

#[test]
fn test_css_parser_id_selector() {
    let css = "#header { background: blue; }";
    let mut parser = CssParser::new(css);
    
    let rules = parser.parse();
    
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.selectors.len(), 1);
    
    match &rule.selectors[0] {
        Selector::Id(name) => assert_eq!(name, "header"),
        _ => panic!("Expected id selector"),
    }
}

#[test]
fn test_css_parser_universal_selector() {
    let css = "* { box-sizing: border-box; }";
    let mut parser = CssParser::new(css);
    
    let rules = parser.parse();
    
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.selectors.len(), 1);
    
    match &rule.selectors[0] {
        Selector::Universal => {},
        _ => panic!("Expected universal selector"),
    }
}

#[test]
fn test_css_parser_multiple_selectors() {
    let css = "h1, h2, h3 { font-weight: bold; }";
    let mut parser = CssParser::new(css);
    
    let rules = parser.parse();
    
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.selectors.len(), 3);
    
    for (i, selector) in rule.selectors.iter().enumerate() {
        match selector {
            Selector::Type(name) => {
                let expected = format!("h{}", i + 1);
                assert_eq!(name, &expected);
            }
            _ => panic!("Expected type selector"),
        }
    }
}

#[test]
fn test_css_parser_important_declaration() {
    let css = "p { color: red !important; }";
    let mut parser = CssParser::new(css);
    
    let rules = parser.parse();
    
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.declarations.len(), 1);
    
    let declaration = &rule.declarations[0];
    assert_eq!(declaration.property, "color");
    assert_eq!(declaration.value, "red");
    assert!(declaration.important);
}

#[test]
fn test_css_parser_complex_values() {
    let css = r#"div { 
        background: url("image.jpg") no-repeat center;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        font-family: "Helvetica Neue", Arial, sans-serif;
    }"#;
    let mut parser = CssParser::new(css);
    
    let rules = parser.parse();
    
    assert_eq!(rules.len(), 1);
    
    let rule = &rules[0];
    assert_eq!(rule.declarations.len(), 3);
    
    let background_decl = &rule.declarations[0];
    assert_eq!(background_decl.property, "background");
    assert!(background_decl.value.contains("url"));
    assert!(background_decl.value.contains("no-repeat"));
    assert!(background_decl.value.contains("center"));
    
    let box_shadow_decl = &rule.declarations[1];
    assert_eq!(box_shadow_decl.property, "box-shadow");
    assert!(box_shadow_decl.value.contains("rgba"));
    
    let font_family_decl = &rule.declarations[2];
    assert_eq!(font_family_decl.property, "font-family");
    assert!(font_family_decl.value.contains("Helvetica Neue"));
}