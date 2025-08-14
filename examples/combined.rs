use html_css_parser::html::{HtmlParser, Node, Element};
use html_css_parser::css::{CssParser, Selector};
use std::collections::HashMap;

fn main() {
    let html = r##"        <!DOCTYPE html>
        <html>
        <head>
            <title>Combined Parser Example</title>
            <style>
                body { font-family: Arial; margin: 0; }
                .header { background: #333; color: white; padding: 20px; }
                .content { max-width: 800px; margin: 0 auto; padding: 20px; }
                #footer { background: #f0f0f0; text-align: center; padding: 10px; }
                .highlight { background-color: yellow; font-weight: bold; }
            </style>
        </head>
        <body>
            <div class="header">
                <h1>Website Title</h1>
            </div>
            <div class="content">
                <p>This is the main content area.</p>
                <p class="highlight">This paragraph is highlighted.</p>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                    <li>Item 3</li>
                </ul>
            </div>
            <div id="footer">
                <p>&copy; 2024 Example Website</p>
            </div>
        </body>
        </html>
    "##;

    println!("=== Combined HTML & CSS Parser Example ===\n");

    // Parse the HTML
    let mut html_parser = HtmlParser::new(html);
    let document = html_parser.parse_document();

    if let Some(Node::Element(html_element)) = document {
        println!("Successfully parsed HTML document!");
        println!("HTML element has {} children\n", html_element.children.len());

        // Extract CSS from style tags
        let css_content = extract_css_from_html(&html_element);
        
        if !css_content.is_empty() {
            println!("Found CSS content in <style> tags:");
            println!("{}\n", css_content);

            // Parse the extracted CSS
            let mut css_parser = CssParser::new(&css_content);
            let css_rules = css_parser.parse();
            
            println!("Parsed {} CSS rules:", css_rules.len());
            for (i, rule) in css_rules.iter().enumerate() {
                println!("  Rule {}: {} selector(s), {} declaration(s)", 
                    i + 1, rule.selectors.len(), rule.declarations.len());
            }
            println!();

            // Analyze the relationship between HTML and CSS
            analyze_html_css_relationship(&html_element, &css_rules);
        }

        // Print HTML structure
        println!("HTML Structure:");
        print_html_structure(&html_element, 0);
    } else {
        println!("Failed to parse HTML document");
    }
}

fn extract_css_from_html(element: &Element) -> String {
    let mut css_content = String::new();
    
    // Check if this is a style element
    if element.tag_name.to_lowercase() == "style" {
        for child in &element.children {
            if let Node::Text(text) = child {
                css_content.push_str(text);
                css_content.push('\n');
            }
        }
    }
    
    // Recursively check children
    for child in &element.children {
        if let Node::Element(child_element) = child {
            css_content.push_str(&extract_css_from_html(child_element));
        }
    }
    
    css_content
}

fn analyze_html_css_relationship(html_element: &Element, css_rules: &[html_css_parser::css::Rule]) {
    println!("=== HTML-CSS Relationship Analysis ===");
    
    // Collect all classes and IDs from HTML
    let mut html_classes = std::collections::HashSet::new();
    let mut html_ids = std::collections::HashSet::new();
    let mut html_tags = std::collections::HashSet::new();
    
    collect_html_identifiers(html_element, &mut html_classes, &mut html_ids, &mut html_tags);
    
    println!("HTML contains:");
    println!("  Tags: {:?}", html_tags);
    println!("  Classes: {:?}", html_classes);
    println!("  IDs: {:?}", html_ids);
    println!();
    
    // Analyze CSS selectors
    let mut css_classes = std::collections::HashSet::new();
    let mut css_ids = std::collections::HashSet::new();
    let mut css_tags = std::collections::HashSet::new();
    
    for rule in css_rules {
        for selector in &rule.selectors {
            match selector {
                Selector::Type(tag) => { css_tags.insert(tag.clone()); }
                Selector::Class(class) => { css_classes.insert(class.clone()); }
                Selector::Id(id) => { css_ids.insert(id.clone()); }
                _ => {}
            }
        }
    }
    
    println!("CSS targets:");
    println!("  Tags: {:?}", css_tags);
    println!("  Classes: {:?}", css_classes);
    println!("  IDs: {:?}", css_ids);
    println!();
    
    // Find matches and mismatches
    println!("Analysis:");
    
    let matching_tags: Vec<_> = html_tags.intersection(&css_tags).collect();
    let matching_classes: Vec<_> = html_classes.intersection(&css_classes).collect();
    let matching_ids: Vec<_> = html_ids.intersection(&css_ids).collect();
    
    println!("  Matching tags: {:?}", matching_tags);
    println!("  Matching classes: {:?}", matching_classes);
    println!("  Matching IDs: {:?}", matching_ids);
    
    let unused_css_classes: Vec<_> = css_classes.difference(&html_classes).collect();
    let unused_css_ids: Vec<_> = css_ids.difference(&html_ids).collect();
    
    if !unused_css_classes.is_empty() {
        println!("  Unused CSS classes: {:?}", unused_css_classes);
    }
    if !unused_css_ids.is_empty() {
        println!("  Unused CSS IDs: {:?}", unused_css_ids);
    }
    
    println!();
}

fn collect_html_identifiers(
    element: &Element,
    classes: &mut std::collections::HashSet<String>,
    ids: &mut std::collections::HashSet<String>,
    tags: &mut std::collections::HashSet<String>,
) {
    tags.insert(element.tag_name.clone());
    
    if let Some(class_attr) = element.attributes.get("class") {
        for class in class_attr.split_whitespace() {
            classes.insert(class.to_string());
        }
    }
    
    if let Some(id_attr) = element.attributes.get("id") {
        ids.insert(id_attr.clone());
    }
    
    for child in &element.children {
        if let Node::Element(child_element) = child {
            collect_html_identifiers(child_element, classes, ids, tags);
        }
    }
}

fn print_html_structure(element: &Element, depth: usize) {
    let indent = "  ".repeat(depth);
    
    print!("{}<{}", indent, element.tag_name);
    
    // Print important attributes
    if let Some(class) = element.attributes.get("class") {
        print!(" class=\"{}\"", class);
    }
    if let Some(id) = element.attributes.get("id") {
        print!(" id=\"{}\"", id);
    }
    
    println!(">");
    
    // Print text content (simplified)
    for child in &element.children {
        match child {
            Node::Element(child_element) => {
                print_html_structure(child_element, depth + 1);
            }
            Node::Text(text) => {
                let trimmed = text.trim();
                if !trimmed.is_empty() && trimmed.len() < 50 {
                    println!("{}  \"{}\"", indent, trimmed);
                }
            }
            Node::Comment(_) => {} // Skip comments for brevity
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_extraction() {
        let html = r#"<html><head><style>body { color: red; }</style></head></html>"#;
        let mut parser = HtmlParser::new(html);
        
        if let Some(Node::Element(html_element)) = parser.parse_document() {
            let css = extract_css_from_html(&html_element);
            assert!(css.contains("body"));
            assert!(css.contains("color: red"));
        }
    }
}