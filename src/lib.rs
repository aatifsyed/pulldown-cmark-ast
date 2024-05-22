use pulldown_cmark::{BrokenLinkCallback, CowStr, Event, Options, Parser, Tag, TagEnd};
use std::{iter, ops::Range};
pub mod visit_mut;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}
impl<T> Spanned<T> {
    pub fn into_inner(self) -> T {
        self.item
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span(pub Range<usize>);

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Ast<'a>(pub Vec<Tree<'a>>);

impl<'a> IntoIterator for Ast<'a> {
    type Item = Spanned<Event<'a>>;

    type IntoIter = Box<dyn Iterator<Item = Spanned<Event<'a>>> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { 0: trees } = self;
        Box::new(trees.into_iter().flatten())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tree<'a> {
    Group(Group<'a>),
    Text(Spanned<CowStr<'a>>),
    /// An inline code node.
    Code(Spanned<CowStr<'a>>),
    /// An HTML node.
    Html(Spanned<CowStr<'a>>),
    /// An inline HTML node.
    InlineHtml(Spanned<CowStr<'a>>),
    /// A reference to a footnote with given label, which may or may not be defined
    /// by an event with a `Tag::FootnoteDefinition` tag. Definitions and references to them may
    /// occur in any order.
    FootnoteReference(Spanned<CowStr<'a>>),
    /// A soft line break.
    SoftBreak(Span),
    /// A hard line break.
    HardBreak(Span),
    /// A horizontal ruler.
    Rule(Span),
    /// A task list marker, rendered as a checkbox in HTML. Contains a true when it is checked.
    TaskListMarker(Spanned<bool>),
    /// An inline math environment node.
    InlineMath(Spanned<CowStr<'a>>),
    /// A display math environment node.
    DisplayMath(Spanned<CowStr<'a>>),
}

impl<'a> IntoIterator for Tree<'a> {
    type Item = Spanned<Event<'a>>;

    type IntoIter = Box<dyn Iterator<Item = Spanned<Event<'a>>> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        /// Need to return a [`Box<dyn Iterator>`] to break the mutually recursive cycle in our return types
        fn once<'a, T: 'a>(item: T, span: Span) -> Box<dyn Iterator<Item = Spanned<T>> + 'a> {
            Box::new(iter::once(Spanned { item, span }))
        }
        match self {
            Tree::Group(it) => it.into_iter(),
            Tree::Text(Spanned { item, span }) => once(Event::Text(item), span),
            Tree::Code(Spanned { item, span }) => once(Event::Code(item), span),
            Tree::Html(Spanned { item, span }) => once(Event::Html(item), span),
            Tree::InlineHtml(Spanned { item, span }) => once(Event::InlineHtml(item), span),
            Tree::FootnoteReference(Spanned { item, span }) => {
                once(Event::FootnoteReference(item), span)
            }
            Tree::SoftBreak(span) => once(Event::SoftBreak, span),
            Tree::HardBreak(span) => once(Event::HardBreak, span),
            Tree::Rule(span) => once(Event::Rule, span),
            Tree::TaskListMarker(Spanned { item, span }) => once(Event::TaskListMarker(item), span),
            Tree::InlineMath(Spanned { item, span }) => once(Event::InlineMath(item), span),
            Tree::DisplayMath(Spanned { item, span }) => once(Event::DisplayMath(item), span),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group<'a> {
    pub tag: Spanned<Tag<'a>>,
    pub stream: Ast<'a>,
    pub end_span: Span,
}

impl<'a> IntoIterator for Group<'a> {
    type Item = Spanned<Event<'a>>;

    type IntoIter = Box<dyn Iterator<Item = Spanned<Event<'a>>> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        let Group {
            tag: Spanned { item: tag, span },
            stream,
            end_span,
        } = self;
        let end = Spanned {
            item: Event::End(tag.to_end()),
            span: end_span,
        };
        Box::new(
            iter::once(Spanned {
                item: Event::Start(tag),
                span,
            })
            .chain(stream)
            .chain(iter::once(end)),
        )
    }
}

impl<'a> Ast<'a> {
    pub fn new(text: &'a str) -> Self {
        Self::new_ext(text, Options::empty())
    }
    pub fn new_ext(text: &'a str, options: Options) -> Self {
        Self::new_with_broken_link_callback(text, options, Some(|_| None))
    }
    pub fn new_with_broken_link_callback<C: BrokenLinkCallback<'a>>(
        text: &'a str,
        options: Options,
        broken_link_callback: Option<C>,
    ) -> Self {
        match Self::from_events(
            &mut Parser::new_with_broken_link_callback(text, options, broken_link_callback)
                .into_offset_iter()
                .map(|(item, range)| Spanned {
                    item,
                    span: Span(range),
                }),
        ) {
            Ok((this, None)) => this,
            Ok((_, Some(_))) | Err(_) => {
                unreachable!("pulldown_cmark guarantees delimters are matched")
            }
        }
    }
    fn from_events(
        evts: &mut dyn Iterator<Item = Spanned<Event<'a>>>,
    ) -> Result<(Self, Option<Spanned<TagEnd>>), Mismatched> {
        let mut this = Self::default();
        while let Some(Spanned { item, span }) = evts.next() {
            match item {
                Event::Start(tag) => match Self::from_events(evts)? {
                    (
                        stream,
                        Some(Spanned {
                            item,
                            span: end_span,
                        }),
                    ) if tag.to_end() == item => this.0.push(Tree::Group(Group {
                        tag: Spanned { item: tag, span },
                        stream,
                        end_span,
                    })),
                    _ => return Err(Mismatched),
                },
                Event::End(item) => return Ok((this, Some(Spanned { item, span }))),
                Event::Text(item) => this.0.push(Tree::Text(Spanned { item, span })),
                Event::Code(item) => this.0.push(Tree::Code(Spanned { item, span })),
                Event::Html(item) => this.0.push(Tree::Html(Spanned { item, span })),
                Event::InlineHtml(item) => this.0.push(Tree::InlineHtml(Spanned { item, span })),
                Event::FootnoteReference(item) => {
                    this.0.push(Tree::FootnoteReference(Spanned { item, span }))
                }
                Event::SoftBreak => this.0.push(Tree::SoftBreak(span)),
                Event::HardBreak => this.0.push(Tree::HardBreak(span)),
                Event::Rule => this.0.push(Tree::Rule(span)),
                Event::TaskListMarker(item) => {
                    this.0.push(Tree::TaskListMarker(Spanned { item, span }))
                }
                Event::InlineMath(item) => this.0.push(Tree::InlineMath(Spanned { item, span })),
                Event::DisplayMath(item) => this.0.push(Tree::DisplayMath(Spanned { item, span })),
            }
        }
        Ok((this, None))
    }
}

struct Mismatched;
