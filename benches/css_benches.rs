use criterion::{black_box, criterion_group, criterion_main, Criterion};
use html_css_parser::css::{CssTokenizer, CssParser};

fn tokenize_css(c: &mut Criterion) {
    let css = r#"
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f0f0f0;
        }

        .container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        }

        #title {
            color: #333;
            font-size: 2em;
            margin-bottom: 1em;
        }

        p {
            line-height: 1.6;
            color: #666;
        }

        ul li {
            margin-bottom: 0.5em;
        }
    "#;

    c.bench_function("tokenize_css", |b| {
        b.iter(|| {
            let tokenizer = CssTokenizer::new(black_box(css));
            let _tokens: Vec<_> = tokenizer.collect();
        })
    });
}

fn parse_css(c: &mut Criterion) {
    let css = r#"
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f0f0f0;
        }

        .container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        }

        #title {
            color: #333;
            font-size: 2em;
            margin-bottom: 1em;
        }

        p {
            line-height: 1.6;
            color: #666;
        }

        ul li {
            margin-bottom: 0.5em;
        }
    "#;

    c.bench_function("parse_css", |b| {
        b.iter(|| {
            let mut parser = CssParser::new(black_box(css));
            let _rules = parser.parse();
        })
    });
}

criterion_group!(benches, tokenize_css, parse_css);
criterion_main!(benches);