# HTML & CSS Parser

A high-performance, resource-efficient HTML and CSS parser written in Rust from scratch.

## Features

- **Fast HTML Tokenization & Parsing**: Efficient tokenizer and DOM tree construction
- **CSS Tokenization & Rule Parsing**: Complete CSS tokenizer with rule and selector parsing
- **Zero Dependencies**: Built from scratch with no external parsing dependencies
- **Memory Efficient**: Minimal allocations and optimized data structures
- **Comprehensive Testing**: Extensive test coverage for both HTML and CSS parsing
- **Benchmarked Performance**: Microsecond-level parsing performance

## Performance

Based on criterion benchmarks:

- **HTML Tokenizing**: ~4.5 µs
- **HTML Parsing**: ~10.3 µs  
- **CSS Tokenizing**: ~7.2 µs
- **CSS Parsing**: ~11.7 µs

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
html-css-parser = "0.1.0"
```

### HTML Parsing

```rust
use html_css_parser::html::{HtmlParser, Node};

fn main() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Example</title>
        </head>
        <body>
            <div class="container">
                <h1 id="title">Hello World</h1>
                <p>This is a <strong>test</strong> paragraph.</p>
            </div>
        </body>
        </html>
    "#;

    let mut parser = HtmlParser::new(html);
    let document = parser.parse_document();

    match document {
        Some(Node::Element(html_element)) => {
            println!("Parsed HTML document with tag: {}", html_element.tag_name);
            println!("Number of children: {}", html_element.children.len());
        }
        _ => println!("Failed to parse HTML document"),
    }
}
```

### HTML Tokenization

```rust
use html_css_parser::html::{HtmlTokenizer, HtmlToken};

fn main() {
    let html = r#"<div class="example">Hello <strong>World</strong></div>"#;
    let tokenizer = HtmlTokenizer::new(html);

    for token in tokenizer {
        match token {
            HtmlToken::StartTag { name, attributes, self_closing } => {
                println!("Start tag: {} (self-closing: {})", name, self_closing);
                for (attr_name, attr_value) in attributes {
                    println!("  Attribute: {} = {}", attr_name, attr_value);
                }
            }
            HtmlToken::EndTag { name } => {
                println!("End tag: {}", name);
            }
            HtmlToken::Text(text) => {
                println!("Text: {}", text);
            }
            HtmlToken::Comment(comment) => {
                println!("Comment: {}", comment);
            }
            HtmlToken::Doctype(doctype) => {
                println!("Doctype: {}", doctype);
            }
        }
    }
}
```

### CSS Parsing

```rust
use html_css_parser::css::{CssParser, Selector};

fn main() {
    let css = r#"
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 20px;
        }

        .container {
            max-width: 800px;
            margin: 0 auto;
        }

        #header {
            background-color: #f0f0f0;
            padding: 10px;
        }

        h1, h2, h3 {
            color: #333;
        }
    "#;

    let mut parser = CssParser::new(css);
    let rules = parser.parse();

    for rule in rules {
        println!("Rule with {} selectors:", rule.selectors.len());
        
        for selector in &rule.selectors {
            match selector {
                Selector::Type(name) => println!("  Type selector: {}", name),
                Selector::Class(name) => println!("  Class selector: .{}", name),
                Selector::Id(name) => println!("  ID selector: #{}", name),
                Selector::Universal => println!("  Universal selector: *"),
                _ => println!("  Complex selector"),
            }
        }

        println!("  Declarations:");
        for declaration in &rule.declarations {
            println!("    {}: {} {}", 
                declaration.property, 
                declaration.value,
                if declaration.important { "!important" } else { "" }
            );
        }
        println!();
    }
}
```

### CSS Tokenization

```rust
use html_css_parser::css::{CssTokenizer, CssToken};

fn main() {
    let css = "body { color: #ff0000; font-size: 16px; }";
    let tokenizer = CssTokenizer::new(css);

    for token in tokenizer {
        match token {
            CssToken::Ident(name) => println!("Identifier: {}", name),
            CssToken::Hash(color) => println!("Hash: #{}", color),
            CssToken::Dimension { value, unit } => {
                println!("Dimension: {}{}", value, unit);
            }
            CssToken::LeftBrace => println!("Left brace: {{"),
            CssToken::RightBrace => println!("Right brace: }}"),
            CssToken::Colon => println!("Colon: :"),
            CssToken::Semicolon => println!("Semicolon: ;"),
            CssToken::Whitespace => {}, // Skip whitespace for cleaner output
            _ => println!("Other token: {:?}", token),
        }
    }
}
```

## Architecture

### HTML Parser

The HTML parser consists of two main components:

1. **HtmlTokenizer**: Converts raw HTML text into a stream of tokens
   - Handles start tags, end tags, text content, comments, and doctypes
   - Supports attributes with quoted and unquoted values
   - Recognizes self-closing tags and void elements

2. **HtmlParser**: Builds a DOM tree from the token stream
   - Creates a hierarchical structure of `Node` elements
   - Handles nested elements and text content
   - Supports document parsing with automatic HTML element detection

### CSS Parser

The CSS parser also has two main components:

1. **CssTokenizer**: Converts CSS text into tokens
   - Handles identifiers, strings, numbers, dimensions, percentages
   - Supports hash colors, comments, and at-keywords
   - Recognizes all CSS punctuation and delimiters

2. **CssParser**: Builds CSS rules from the token stream
   - Parses selectors (type, class, ID, universal)
   - Handles declarations with property-value pairs
   - Supports `!important` declarations and complex values

## Data Structures

### HTML

```rust
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
}

pub struct Element {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}
```

### CSS

```rust
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub struct Declaration {
    pub property: String,
    pub value: String,
    pub important: bool,
}

pub enum Selector {
    Type(String),
    Class(String),
    Id(String),
    Universal,
    // Complex selectors for future expansion
}
```

## Testing

Run the test suite:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

## Examples

See the `examples/` directory for more comprehensive usage examples:

- `examples/html_parser.rs` - Complete HTML parsing example
- `examples/css_parser.rs` - Complete CSS parsing example
- `examples/combined.rs` - Using both parsers together

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.