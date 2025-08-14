use html_css_parser::css::{CssParser, CssTokenizer, Selector, Declaration};

fn main() {
    let css = r##"
        /* Global styles */
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        body {
            font-family: 'Helvetica Neue', Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background-color: #f8f9fa;
        }

        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 20px;
        }

        #header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 2rem 0;
            text-align: center;
        }

        .nav-menu {
            display: flex;
            justify-content: center;
            list-style: none;
            margin-top: 1rem;
        }

        .nav-menu li {
            margin: 0 1rem;
        }

        .nav-menu a {
            color: white;
            text-decoration: none;
            font-weight: 500;
            transition: opacity 0.3s ease;
        }

        .nav-menu a:hover {
            opacity: 0.8;
        }

        .feature-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 2rem;
            margin: 2rem 0;
        }

        .feature {
            background: white;
            padding: 1.5rem;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
            border-left: 4px solid #667eea;
        }

        .feature h3 {
            color: #667eea;
            margin-bottom: 0.5rem;
            font-size: 1.25rem;
        }

        .feature p {
            color: #666;
            line-height: 1.5;
        }

        @media (max-width: 768px) {
            .container {
                padding: 0 15px;
            }
            
            .feature-grid {
                grid-template-columns: 1fr;
                gap: 1rem;
            }
        }

        /* Utility classes */
        .text-center { text-align: center; }
        .text-bold { font-weight: bold !important; }
        .mb-1 { margin-bottom: 1rem; }
        .mb-2 { margin-bottom: 2rem; }
    "##;

    println!("=== CSS Tokenization Example ===");
    let tokenizer = CssTokenizer::new(css);
    let mut token_count = 0;
    
    for token in tokenizer {
        token_count += 1;
        if token_count <= 15 { // Show first 15 tokens
            println!("{:?}", token);
        }
    }
    println!("Total tokens: {}\n", token_count);

    println!("=== CSS Parsing Example ===");
    let mut parser = CssParser::new(css);
    let rules = parser.parse();
    
    println!("Parsed {} CSS rules:\n", rules.len());
    
    for (i, rule) in rules.iter().enumerate() {
        println!("Rule #{}: {} selector(s)", i + 1, rule.selectors.len());
        
        // Print selectors
        for selector in &rule.selectors {
            print!("  ");
            match selector {
                Selector::Type(name) => print!("{}", name),
                Selector::Class(name) => print!(".{}", name),
                Selector::Id(name) => print!("#{}", name),
                Selector::Universal => print!("*"),
                _ => print!("(complex selector)"),
            }
            println!();
        }
        
        // Print declarations
        println!("  {} declaration(s):", rule.declarations.len());
        for declaration in &rule.declarations {
            println!("    {}: {}{}", 
                declaration.property, 
                declaration.value,
                if declaration.important { " !important" } else { "" }
            );
        }
        println!();
    }

    // Demonstrate specific parsing features
    demonstrate_parsing_features();
}

fn demonstrate_parsing_features() {
    println!("=== Parsing Features Demonstration ===\n");

    // Test different selector types
    let selector_examples = vec![
        ("body", "Type selector"),
        (".container", "Class selector"),
        ("#header", "ID selector"),
        ("*", "Universal selector"),
    ];

    for (css_rule, description) in selector_examples {
        let full_css = format!("{} {{ color: red; }}", css_rule);
        let mut parser = CssParser::new(&full_css);
        let rules = parser.parse();
        
        if let Some(rule) = rules.first() {
            if let Some(selector) = rule.selectors.first() {
                println!("{}: {:?}", description, selector);
            }
        }
    }

    println!();

    // Test different value types
    let value_examples = vec![
        ("color: #ff0000;", "Hex color"),
        ("width: 100px;", "Dimension with unit"),
        ("opacity: 0.5;", "Decimal number"),
        ("margin: 50%;", "Percentage"),
        ("font-family: \"Arial\";", "Quoted string"),
        ("display: block !important;", "Important declaration"),
    ];

    for (css_decl, description) in value_examples {
        let full_css = format!("div {{ {} }}", css_decl);
        let mut parser = CssParser::new(&full_css);
        let rules = parser.parse();
        
        if let Some(rule) = rules.first() {
            if let Some(declaration) = rule.declarations.first() {
                println!("{}: {} = {}{}", 
                    description,
                    declaration.property, 
                    declaration.value,
                    if declaration.important { " !important" } else { "" }
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_css_parsing() {
        let css = ".test { color: blue; font-size: 14px; }";
        let mut parser = CssParser::new(css);
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 1);
        
        let rule = &rules[0];
        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 2);
        
        match &rule.selectors[0] {
            Selector::Class(name) => assert_eq!(name, "test"),
            _ => panic!("Expected class selector"),
        }
        
        assert_eq!(rule.declarations[0].property, "color");
        assert_eq!(rule.declarations[0].value, "blue");
        
        assert_eq!(rule.declarations[1].property, "font-size");
        assert_eq!(rule.declarations[1].value, "14px");
    }
}