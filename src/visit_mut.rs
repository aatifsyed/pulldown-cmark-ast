// ouch my wrists...

use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, CowStr, HeadingLevel, LinkType, MetadataBlockKind,
    Tag,
};

use crate::{Ast, Group, Span, Spanned, Tree};

pub trait VisitMut<'a> {
    fn visit_ast_mut(&mut self, node: &mut Ast<'a>) {
        visit_ast_mut(self, node)
    }
    fn visit_tree_mut(&mut self, node: &mut Tree<'a>) {
        visit_tree_mut(self, node)
    }
    fn visit_text_mut(&mut self, node: &mut Spanned<CowStr<'a>>) {
        visit_text_mut(self, node)
    }
    fn visit_cow_str_mut(&mut self, node: &mut CowStr<'a>) {
        visit_cow_str_mut(self, node)
    }
    fn visit_code_mut(&mut self, node: &mut Spanned<CowStr<'a>>) {
        visit_code_mut(self, node);
    }
    fn visit_html_mut(&mut self, node: &mut Spanned<CowStr<'a>>) {
        visit_html_mut(self, node)
    }
    fn visit_inline_html_mut(&mut self, node: &mut Spanned<CowStr<'a>>) {
        visit_inline_html_mut(self, node)
    }
    fn visit_footnote_reference_mut(&mut self, node: &mut Spanned<CowStr<'a>>) {
        visit_footnote_reference_mut(self, node)
    }
    fn visit_task_list_marker_mut(&mut self, node: &mut Spanned<bool>) {
        visit_task_list_marker_mut(self, node)
    }
    fn visit_inline_math_mut(&mut self, node: &mut Spanned<CowStr<'a>>) {
        visit_inline_math_mut(self, node)
    }
    fn visit_display_math_mut(&mut self, node: &mut Spanned<CowStr<'a>>) {
        visit_display_math_mut(self, node)
    }
    fn visit_soft_break_mut(&mut self, node: &mut Span) {
        visit_soft_break_mut(self, node)
    }
    fn visit_hard_break_mut(&mut self, node: &mut Span) {
        visit_hard_break_mut(self, node)
    }
    fn visit_rule_mut(&mut self, node: &mut Span) {
        visit_rule_mut(self, node)
    }
    fn visit_group_mut(&mut self, node: &mut Group<'a>) {
        visit_group_mut(self, node)
    }

    fn visit_strikethrough_mut(
        &mut self,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_strikethrough_mut(self, stream, begin_span, end_span)
    }

    fn visit_strong_mut(
        &mut self,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_strong_mut(self, stream, begin_span, end_span)
    }

    fn visit_emphasis_mut(
        &mut self,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_emphasis_mut(self, stream, begin_span, end_span)
    }

    fn visit_table_cell_mut(
        &mut self,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_table_cell_mut(self, stream, begin_span, end_span)
    }

    fn visit_table_row_mut(
        &mut self,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_table_row_mut(self, stream, begin_span, end_span)
    }

    fn visit_table_head_mut(
        &mut self,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_table_head_mut(self, stream, begin_span, end_span)
    }

    fn visit_table_mut(
        &mut self,
        node: &mut Vec<Alignment>,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_table_mut(self, node, stream, begin_span, end_span)
    }

    fn visit_footnote_definition_mut(
        &mut self,
        node: &mut CowStr<'a>,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_footnote_definition_mut(self, node, stream, begin_span, end_span)
    }

    fn visit_list_mut(
        &mut self,
        node: &mut Option<u64>,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_list_mut(self, node, stream, begin_span, end_span)
    }

    fn visit_code_block_mut(
        &mut self,
        node: &mut CodeBlockKind,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_code_block_mut(self, node, stream, begin_span, end_span)
    }

    fn visit_block_quote_mut(
        &mut self,
        node: &mut Option<BlockQuoteKind>,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_block_quote_mut(self, node, stream, begin_span, end_span)
    }

    #[allow(clippy::too_many_arguments)]
    fn visit_heading_mut(
        &mut self,
        level: &mut HeadingLevel,
        id: &mut Option<CowStr<'a>>,
        classes: &mut Vec<CowStr<'a>>,
        attrs: &mut Vec<(CowStr<'a>, Option<CowStr<'a>>)>,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_heading_mut(
            self, level, id, classes, attrs, stream, begin_span, end_span,
        )
    }

    fn visit_html_block_mut(
        &mut self,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_html_block_mut(self, stream, begin_span, end_span)
    }
    fn visit_paragraph_mut(
        &mut self,
        stream: &mut Ast<'a>,
        begin_span: &mut Span,
        end_span: &mut Span,
    ) {
        visit_paragraph_mut(self, stream, begin_span, end_span)
    }

    fn visit_item_mut(&mut self, stream: &mut Ast<'a>, begin_span: &mut Span, end_span: &mut Span) {
        visit_item_mut(self, stream, begin_span, end_span)
    }

    fn visit_metadata_block_mut(&mut self, node: &mut MetadataBlockKind) {
        visit_metadata_block_mut(self, node)
    }

    fn visit_image_mut(
        &mut self,
        link_type: &mut LinkType,
        dest_url: &mut CowStr<'a>,
        title: &mut CowStr<'a>,
        id: &mut CowStr<'a>,
    ) {
        visit_image_mut(self, link_type, dest_url, title, id)
    }

    fn visit_link_mut(
        &mut self,
        link_type: &mut LinkType,
        dest_url: &mut CowStr<'a>,
        title: &mut CowStr<'a>,
        id: &mut CowStr<'a>,
    ) {
        visit_link_mut(self, link_type, dest_url, title, id)
    }
}

pub fn visit_group_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut Group<'a>) {
    let Group {
        tag: Spanned {
            item: tag,
            span: begin_span,
        },
        stream,
        end_span,
    } = node;

    match tag {
        Tag::Paragraph => v.visit_paragraph_mut(stream, begin_span, end_span),
        Tag::Heading {
            level,
            id,
            classes,
            attrs,
        } => v.visit_heading_mut(level, id, classes, attrs, stream, begin_span, end_span),
        Tag::BlockQuote(node) => v.visit_block_quote_mut(node, stream, begin_span, end_span),
        Tag::CodeBlock(node) => v.visit_code_block_mut(node, stream, begin_span, end_span),
        Tag::HtmlBlock => v.visit_html_block_mut(stream, begin_span, end_span),
        Tag::List(node) => v.visit_list_mut(node, stream, begin_span, end_span),
        Tag::Item => v.visit_item_mut(stream, begin_span, end_span),
        Tag::FootnoteDefinition(node) => {
            v.visit_footnote_definition_mut(node, stream, begin_span, end_span)
        }
        Tag::Table(node) => v.visit_table_mut(node, stream, begin_span, end_span),
        Tag::TableHead => v.visit_table_head_mut(stream, begin_span, end_span),
        Tag::TableRow => v.visit_table_row_mut(stream, begin_span, end_span),
        Tag::TableCell => v.visit_table_cell_mut(stream, begin_span, end_span),
        Tag::Emphasis => v.visit_emphasis_mut(stream, begin_span, end_span),
        Tag::Strong => v.visit_strong_mut(stream, begin_span, end_span),
        Tag::Strikethrough => v.visit_strikethrough_mut(stream, begin_span, end_span),
        Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        } => visit_link_mut(v, link_type, dest_url, title, id),
        Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        } => visit_image_mut(v, link_type, dest_url, title, id),
        Tag::MetadataBlock(node) => visit_metadata_block_mut(v, node),
    }
}

fn visit_metadata_block_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut MetadataBlockKind) {
    let _ = v;
    let _ = node;
}

fn visit_image_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    link_type: &mut LinkType,
    dest_url: &mut CowStr<'a>,
    title: &mut CowStr<'a>,
    id: &mut CowStr<'a>,
) {
    let _ = link_type;
    v.visit_cow_str_mut(dest_url);
    v.visit_cow_str_mut(title);
    v.visit_cow_str_mut(id);
}

fn visit_link_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    link_type: &mut LinkType,
    dest_url: &mut CowStr<'a>,
    title: &mut CowStr<'a>,
    id: &mut CowStr<'a>,
) {
    let _ = link_type;
    v.visit_cow_str_mut(dest_url);
    v.visit_cow_str_mut(title);
    v.visit_cow_str_mut(id);
}

fn visit_strikethrough_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_ast_mut(stream)
}

fn visit_strong_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_ast_mut(stream)
}

fn visit_emphasis_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_ast_mut(stream)
}

fn visit_table_cell_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_ast_mut(stream)
}

fn visit_table_row_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_ast_mut(stream)
}

fn visit_table_head_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_ast_mut(stream)
}

fn visit_table_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut Vec<Alignment>,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    let _ = node;
    v.visit_ast_mut(stream)
}

fn visit_footnote_definition_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut CowStr<'a>,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_cow_str_mut(node);
    v.visit_ast_mut(stream)
}

fn visit_item_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_ast_mut(stream)
}

fn visit_list_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut Option<u64>,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    let _ = node;
    v.visit_ast_mut(stream)
}

fn visit_html_block_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_ast_mut(stream)
}

fn visit_code_block_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut CodeBlockKind,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    let _ = node;
    v.visit_ast_mut(stream)
}

fn visit_block_quote_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut Option<BlockQuoteKind>,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    let _ = node;
    v.visit_ast_mut(stream)
}

#[allow(clippy::too_many_arguments)]
fn visit_heading_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    level: &mut HeadingLevel,
    id: &mut Option<CowStr<'a>>,
    classes: &mut Vec<CowStr<'a>>,
    attrs: &mut Vec<(CowStr<'a>, Option<CowStr<'a>>)>,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    let _ = level;
    let _ = id;
    let _ = classes;
    let _ = attrs;
    v.visit_ast_mut(stream)
}

fn visit_paragraph_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    stream: &mut Ast<'a>,
    begin_span: &mut Span,
    end_span: &mut Span,
) {
    let _ = begin_span;
    let _ = end_span;
    v.visit_ast_mut(stream)
}

pub fn visit_code_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut Spanned<CowStr<'a>>) {
    v.visit_cow_str_mut(&mut node.item)
}

pub fn visit_ast_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut Ast<'a>) {
    let Ast(trees) = node;
    for tree in trees {
        v.visit_tree_mut(tree)
    }
}
pub fn visit_html_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut Spanned<CowStr<'a>>) {
    v.visit_cow_str_mut(&mut node.item)
}
pub fn visit_inline_html_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut Spanned<CowStr<'a>>,
) {
    v.visit_cow_str_mut(&mut node.item)
}
pub fn visit_footnote_reference_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut Spanned<CowStr<'a>>,
) {
    v.visit_cow_str_mut(&mut node.item)
}
pub fn visit_task_list_marker_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut Spanned<bool>,
) {
    let _ = v;
    let _ = node;
}
pub fn visit_inline_math_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut Spanned<CowStr<'a>>,
) {
    v.visit_cow_str_mut(&mut node.item)
}
pub fn visit_display_math_mut<'a, V: VisitMut<'a> + ?Sized>(
    v: &mut V,
    node: &mut Spanned<CowStr<'a>>,
) {
    v.visit_cow_str_mut(&mut node.item)
}

pub fn visit_soft_break_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut Span) {
    let _ = v;
    let _ = node;
}
pub fn visit_hard_break_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut Span) {
    let _ = v;
    let _ = node;
}
pub fn visit_rule_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut Span) {
    let _ = v;
    let _ = node;
}

pub fn visit_tree_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut Tree<'a>) {
    match node {
        Tree::Group(node) => v.visit_group_mut(node),
        Tree::Text(node) => v.visit_text_mut(node),
        Tree::Code(node) => v.visit_code_mut(node),
        Tree::Html(node) => v.visit_html_mut(node),
        Tree::InlineHtml(node) => v.visit_inline_html_mut(node),
        Tree::FootnoteReference(node) => v.visit_footnote_reference_mut(node),
        Tree::TaskListMarker(node) => v.visit_task_list_marker_mut(node),
        Tree::InlineMath(node) => v.visit_inline_math_mut(node),
        Tree::DisplayMath(node) => v.visit_display_math_mut(node),
        Tree::SoftBreak(node) => v.visit_soft_break_mut(node),
        Tree::HardBreak(node) => v.visit_hard_break_mut(node),
        Tree::Rule(node) => v.visit_rule_mut(node),
    }
}
pub fn visit_text_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut Spanned<CowStr<'a>>) {
    v.visit_cow_str_mut(&mut node.item)
}
pub fn visit_cow_str_mut<'a, V: VisitMut<'a> + ?Sized>(v: &mut V, node: &mut CowStr<'a>) {
    let _ = v;
    let _ = node;
}
