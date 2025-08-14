use html_css_parser::html::{HtmlParser, HtmlTokenizer, Node};
use html_css_parser::css::{CssParser, CssTokenizer, Selector};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }
    
    let command = &args[1];
    let file_path = &args[2];
    
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", file_path, err);
            process::exit(1);
        }
    };
    
    match command.as_str() {
        "html-tokenize" => tokenize_html(&content),
        "html-parse" => parse_html(&content),
        "css-tokenize" => tokenize_css(&content),
        "css-parse" => parse_css(&content),
        "demo" => run_demo(),
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("HTML & CSS Parser CLI");
    println!();
    println!("Usage:");
    println!("  {} <command> <file>", env::args().next().unwrap_or_else(|| "html-css-parser".to_string()));
    println!();
    println!("Commands:");
    println!("  html-tokenize <file>  Tokenize HTML file");
    println!("  html-parse <file>     Parse HTML file into DOM tree");
    println!("  css-tokenize <file>   Tokenize CSS file");
    println!("  css-parse <file>      Parse CSS file into rules");
    println!("  demo                  Run built-in demo (no file needed)");
    println!();
    println!("Examples:");
    println!("  {} html-parse index.html", env::args().next().unwrap_or_else(|| "html-css-parser".to_string()));
    println!("  {} css-parse styles.css", env::args().next().unwrap_or_else(|| "html-css-parser".to_string()));
}

fn tokenize_html(content: &str) {
    println!("=== HTML Tokenization ===");
    let tokenizer = HtmlTokenizer::new(content);
    let mut count = 0;
    
    for token in tokenizer {
        count += 1;
        println!("{}: {:?}", count, token);
    }
    
    println!("\nTotal tokens: {}", count);
}

fn parse_html(content: &str) {
    println!("=== HTML Parsing ===");
    let mut parser = HtmlParser::new(content);
    
    match parser.parse_document() {
        Some(document) => {
            println!("Successfully parsed HTML document!");
            print_node(&document, 0);
        }
        None => {
            println!("Failed to parse HTML document");
            let nodes = parser.parse();
            if !nodes.is_empty() {
                println!("Found {} top-level nodes:", nodes.len());
                for node in &nodes {
                    print_node(node, 0);
                }
            }
        }
    }
}

fn tokenize_css(content: &str) {
    println!("=== CSS Tokenization ===");
    let tokenizer = CssTokenizer::new(content);
    let mut count = 0;
    
    for token in tokenizer {
        count += 1;
        println!("{}: {:?}", count, token);
    }
    
    println!("\nTotal tokens: {}", count);
}

fn parse_css(content: &str) {
    println!("=== CSS Parsing ===");
    let mut parser = CssParser::new(content);
    let rules = parser.parse();
    
    println!("Parsed {} CSS rules:", rules.len());
    
    for (i, rule) in rules.iter().enumerate() {
        println!("\nRule #{}: {} selector(s)", i + 1, rule.selectors.len());
        
        for selector in &rule.selectors {
            print!("  ");
            match selector {
                Selector::Type(name) => println!("Type: {}", name),
                Selector::Class(name) => println!("Class: .{}", name),
                Selector::Id(name) => println!("ID: #{}", name),
                Selector::Universal => println!("Universal: *"),
                _ => println!("Complex selector"),
            }
        }
        
        println!("  {} declaration(s):", rule.declarations.len());
        for declaration in &rule.declarations {
            println!("    {}: {}{}", 
                declaration.property, 
                declaration.value,
                if declaration.important { " !important" } else { "" }
            );
        }
    }
}

fn print_node(node: &Node, depth: usize) {
    let indent = "  ".repeat(depth);
    
    match node {
        Node::Element(element) => {
            print!("{}<{}", indent, element.tag_name);
            
            for (name, value) in &element.attributes {
                print!(" {}=\"{}\"", name, value);
            }
            
            if element.children.is_empty() {
                println!(" />");
            } else {
                println!(">");
                
                for child in &element.children {
                    print_node(child, depth + 1);
                }
                
                println!("{}</{}>", indent, element.tag_name);
            }
        }
        Node::Text(text) => {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                println!("{}{}", indent, trimmed);
            }
        }
        Node::Comment(comment) => {
            println!("{}<!-- {} -->", indent, comment);
        }
    }
}

fn run_demo() {
    println!("=== HTML & CSS Parser Demo ===\n");
    
    let html = r##"<!DOCTYPE html>
<html>
<head>
    <title>Demo Page</title>
</head>
<body>
    <div class="container">
        <h1 id="title">Hello World</h1>
        <p>This is a <strong>demo</strong> page.</p>
    </div>
</body>
</html>"##;

    let css = r##"body {
    font-family: Arial, sans-serif;
    margin: 0;
}

.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

#title {
    color: #333;
    font-size: 2em;
}"##;

    println!("HTML Demo:");
    parse_html(html);
    
    println!("\n{}\n", "=".repeat(50));
    
    println!("CSS Demo:");
    parse_css(css);
}
