use html_css_parser::html::{HtmlParser, HtmlTokenizer, Node, Element};

fn main() {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>HTML Parser Example</title>
        </head>
        <body>
            <header>
                <h1 id="main-title">Welcome to HTML Parser</h1>
                <nav>
                    <ul>
                        <li><a href="#home">Home</a></li>
                        <li><a href="#about">About</a></li>
                        <li><a href="#contact">Contact</a></li>
                    </ul>
                </nav>
            </header>
            
            <main>
                <section class="intro">
                    <h2>Introduction</h2>
                    <p>This is an example of parsing HTML with our <strong>custom parser</strong>.</p>
                    <p>It supports:</p>
                    <ul>
                        <li>Nested elements</li>
                        <li>Attributes with values</li>
                        <li>Text content</li>
                        <li>Comments <!-- like this one --></li>
                        <li>Self-closing tags like <img src="example.jpg" alt="Example" /></li>
                    </ul>
                </section>
                
                <section class="features">
                    <h2>Features</h2>
                    <div class="feature-grid">
                        <div class="feature">
                            <h3>Fast</h3>
                            <p>Optimized for performance</p>
                        </div>
                        <div class="feature">
                            <h3>Accurate</h3>
                            <p>Handles complex HTML structures</p>
                        </div>
                        <div class="feature">
                            <h3>Memory Efficient</h3>
                            <p>Minimal memory footprint</p>
                        </div>
                    </div>
                </section>
            </main>
            
            <footer>
                <p>&copy; 2024 HTML Parser Example</p>
            </footer>
        </body>
        </html>
    "##;

    println!("=== HTML Tokenization Example ===");
    let tokenizer = HtmlTokenizer::new(html);
    let mut token_count = 0;
    
    for token in tokenizer {
        token_count += 1;
        if token_count <= 10 { // Show first 10 tokens
            println!("{:?}", token);
        }
    }
    println!("Total tokens: {}\n", token_count);

    println!("=== HTML Parsing Example ===");
    let mut parser = HtmlParser::new(html);
    
    if let Some(document) = parser.parse_document() {
        print_node(&document, 0);
    } else {
        println!("Failed to parse HTML document");
    }
}

fn print_node(node: &Node, depth: usize) {
    let indent = "  ".repeat(depth);
    
    match node {
        Node::Element(element) => {
            print!("{}<{}", indent, element.tag_name);
            
            // Print attributes
            for (name, value) in &element.attributes {
                print!(" {}=\"{}\"", name, value);
            }
            println!(">");
            
            // Print children
            for child in &element.children {
                print_node(child, depth + 1);
            }
            
            println!("{}</{}>", indent, element.tag_name);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parsing() {
        let html = "<div><p>Test</p></div>";
        let mut parser = HtmlParser::new(html);
        let nodes = parser.parse();
        
        assert_eq!(nodes.len(), 1);
        
        if let Node::Element(div) = &nodes[0] {
            assert_eq!(div.tag_name, "div");
            assert_eq!(div.children.len(), 1);
            
            if let Node::Element(p) = &div.children[0] {
                assert_eq!(p.tag_name, "p");
                assert_eq!(p.children.len(), 1);
                
                if let Node::Text(text) = &p.children[0] {
                    assert_eq!(text, "Test");
                }
            }
        }
    }
}