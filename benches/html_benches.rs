use criterion::{black_box, criterion_group, criterion_main, Criterion};
use html_css_parser::html::{HtmlTokenizer, HtmlParser};

fn tokenize_html(c: &mut Criterion) {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Test Page</title>
            <meta charset="utf-8">
        </head>
        <body>
            <div class="container">
                <h1 id="title">Hello World</h1>
                <p>This is a test paragraph with <strong>bold</strong> text.</p>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                    <li>Item 3</li>
                </ul>
            </div>
        </body>
        </html>
    "#;

    c.bench_function("tokenize_html", |b| {
        b.iter(|| {
            let tokenizer = HtmlTokenizer::new(black_box(html));
            let _tokens: Vec<_> = tokenizer.collect();
        })
    });
}

fn parse_html(c: &mut Criterion) {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Test Page</title>
            <meta charset="utf-8">
        </head>
        <body>
            <div class="container">
                <h1 id="title">Hello World</h1>
                <p>This is a test paragraph with <strong>bold</strong> text.</p>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                    <li>Item 3</li>
                </ul>
            </div>
        </body>
        </html>
    "#;

    c.bench_function("parse_html", |b| {
        b.iter(|| {
            let mut parser = HtmlParser::new(black_box(html));
            let _document = parser.parse_document();
        })
    });
}

criterion_group!(benches, tokenize_html, parse_html);
criterion_main!(benches);