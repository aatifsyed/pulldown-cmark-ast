use std::{
    io::{self, Read as _},
    process,
};

use ariadne::{Color, Label, Report, ReportKind, Source};
use pulldown_cmark::{Options, Tag};
use pulldown_cmark_ast::{Ast, Group, Span, Spanned, Tree};

fn main() {
    let mut txt = String::new();

    eprintln!("reading from stdin...");
    if let Err(e) = io::stdin().read_to_string(&mut txt) {
        eprintln!("Error reading from stdin: {}", e);
        process::exit(1)
    };

    let ast = pulldown_cmark_ast::Ast::new_ext(&txt, Options::all());
    let mut labels = vec![];
    visit_ast(&mut labels, &ast);

    Report::build(ReportKind::Custom("info", Color::Blue), (), 0)
        .with_labels(labels)
        .finish()
        .print(Source::from(txt))
        .unwrap();
}

fn visit_ast(labels: &mut Vec<Label>, ast: &Ast) {
    let Ast(trees) = ast;
    for tree in trees {
        let span;
        let (text, Span(span)) = match tree {
            Tree::Group(Group {
                tag:
                    Spanned {
                        item: tag,
                        span: Span(begin_span),
                    },
                stream,
                end_span: Span(end_span),
            }) => {
                let text = match tag {
                    Tag::Paragraph => "paragraph",
                    Tag::Heading { .. } => "heading",
                    Tag::BlockQuote(_) => "block-quote",
                    Tag::CodeBlock(_) => "code-block",
                    Tag::HtmlBlock => "html-block",
                    Tag::List(_) => "list",
                    Tag::Item => "item",
                    Tag::FootnoteDefinition(_) => "footnote-definition",
                    Tag::Table(_) => "table",
                    Tag::TableHead => "table-head",
                    Tag::TableRow => "table-row",
                    Tag::TableCell => "table-cell",
                    Tag::Emphasis => "emphasis",
                    Tag::Strong => "strong",
                    Tag::Strikethrough => "strikethrough",
                    Tag::Link { .. } => "link",
                    Tag::Image { .. } => "image",
                    Tag::MetadataBlock(_) => "metadata-block",
                };
                span = Span(begin_span.start..end_span.end);
                visit_ast(labels, stream);

                (text, &span)
            }
            Tree::Text(Spanned { span, .. }) => ("text", span),
            Tree::Code(Spanned { span, .. }) => ("code", span),
            Tree::Html(Spanned { span, .. }) => ("html", span),
            Tree::InlineHtml(Spanned { span, .. }) => ("inline-html", span),
            Tree::FootnoteReference(Spanned { span, .. }) => ("footnote-reference", span),
            Tree::SoftBreak(span) => ("soft-break", span),
            Tree::HardBreak(span) => ("hard-break", span),
            Tree::Rule(span) => ("rule", span),
            Tree::TaskListMarker(Spanned { span, .. }) => ("task-list-marker", span),
            Tree::InlineMath(Spanned { span, .. }) => ("inline-math", span),
            Tree::DisplayMath(Spanned { span, .. }) => ("display-math", span),
        };
        labels.push(Label::new(span.clone()).with_message(text))
    }
}
