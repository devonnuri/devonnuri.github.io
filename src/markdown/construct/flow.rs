//! The flow content type.
//!
//! **Flow** represents the sections, such as headings and code, which are
//! parsed per line.
//! An example is HTML, which has a certain starting condition (such as
//! `<script>` on its own line), then continues for a while, until an end
//! condition is found (such as `</style>`).
//! If that line with an end condition is never found, that flow goes until
//! the end.
//!
//! The constructs found in flow are:
//!
//! *   [Blank line][crate::construct::blank_line]
//! *   [Code (indented)][crate::construct::code_indented]
//! *   [Heading (atx)][crate::construct::heading_atx]
//! *   [Heading (setext)][crate::construct::heading_setext]
//! *   [HTML (flow)][crate::construct::html_flow]
//! *   [MDX esm][crate::construct::mdx_esm]
//! *   [MDX expression (flow)][crate::construct::mdx_expression_flow]
//! *   [MDX JSX (flow)][crate::construct::mdx_jsx_flow]
//! *   [Raw (flow)][crate::construct::raw_flow] (code (fenced), math (flow))
//! *   [Thematic break][crate::construct::thematic_break]

use crate::markdown::event::Name;
use crate::markdown::state::{Name as StateName, State};
use crate::markdown::tokenizer::Tokenizer;

/// Start of flow.
//
/// ```markdown
/// > | ## alpha
///     ^
/// > |     bravo
///     ^
/// > | ***
///     ^
/// ```
pub fn start(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'#') => {
            tokenizer.attempt(
                State::Next(StateName::FlowAfter),
                State::Next(StateName::FlowBeforeContent),
            );
            State::Retry(StateName::HeadingAtxStart)
        }
        Some(b'$' | b'`' | b'~') => {
            tokenizer.attempt(
                State::Next(StateName::FlowAfter),
                State::Next(StateName::FlowBeforeContent),
            );
            State::Retry(StateName::RawFlowStart)
        }
        // Note: `-` is also used in setext heading underline so it’s not
        // included here.
        Some(b'*' | b'_') => {
            tokenizer.attempt(
                State::Next(StateName::FlowAfter),
                State::Next(StateName::FlowBeforeContent),
            );
            State::Retry(StateName::ThematicBreakStart)
        }
        Some(b'<') => {
            tokenizer.attempt(
                State::Next(StateName::FlowAfter),
                State::Next(StateName::FlowBeforeHeadingAtx),
            );
            State::Retry(StateName::HtmlFlowStart)
        }
        // Actual parsing: blank line? Indented code? Indented anything?
        // Tables, setext heading underlines, definitions, and Contents are
        // particularly weird.
        _ => State::Retry(StateName::FlowBlankLineBefore),
    }
}

/// At blank line.
///
/// ```markdown
/// > | ␠␠␊
///     ^
/// ```
pub fn blank_line_before(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::FlowBlankLineAfter),
        State::Next(StateName::FlowBeforeCodeIndented),
    );
    State::Retry(StateName::BlankLineStart)
}

/// At code (indented).
///
/// ```markdown
/// > | ␠␠␠␠a
///     ^
/// ```
pub fn before_code_indented(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::FlowAfter),
        State::Next(StateName::FlowBeforeRaw),
    );
    State::Retry(StateName::CodeIndentedStart)
}

/// At raw.
///
/// ````markdown
/// > | ```
///     ^
/// ````
pub fn before_raw(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::FlowAfter),
        State::Next(StateName::FlowBeforeHtml),
    );
    State::Retry(StateName::RawFlowStart)
}

/// At html (flow).
///
/// ```markdown
/// > | <a>
///     ^
/// ```
pub fn before_html(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::FlowAfter),
        State::Next(StateName::FlowBeforeHeadingAtx),
    );
    State::Retry(StateName::HtmlFlowStart)
}

/// At heading (atx).
///
/// ```markdown
/// > | # a
///     ^
/// ```
pub fn before_heading_atx(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::FlowAfter),
        State::Next(StateName::FlowBeforeHeadingSetext),
    );
    State::Retry(StateName::HeadingAtxStart)
}

/// At heading (setext).
///
/// ```markdown
///   | a
/// > | =
///     ^
/// ```
pub fn before_heading_setext(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::FlowAfter),
        State::Next(StateName::FlowBeforeThematicBreak),
    );
    State::Retry(StateName::HeadingSetextStart)
}

/// At thematic break.
///
/// ```markdown
/// > | ***
///     ^
/// ```
pub fn before_thematic_break(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::FlowAfter),
        State::Next(StateName::FlowBeforeGfmTable),
    );
    State::Retry(StateName::ThematicBreakStart)
}

/// At GFM table.
///
/// ```markdown
/// > | | a |
///     ^
/// ```
pub fn before_gfm_table(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(
        State::Next(StateName::FlowAfter),
        State::Next(StateName::FlowBeforeContent),
    );
    State::Retry(StateName::GfmTableStart)
}

/// At content.
///
/// ```markdown
/// > | a
///     ^
/// ```
pub fn before_content(tokenizer: &mut Tokenizer) -> State {
    tokenizer.attempt(State::Next(StateName::FlowAfter), State::Nok);
    State::Retry(StateName::ContentChunkStart)
}

/// After blank line.
///
/// ```markdown
/// > | ␠␠␊
///       ^
/// ```
pub fn blank_line_after(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        None => State::Ok,
        Some(b'\n') => {
            tokenizer.enter(Name::BlankLineEnding);
            tokenizer.consume();
            tokenizer.exit(Name::BlankLineEnding);
            // Feel free to interrupt.
            tokenizer.interrupt = false;
            State::Next(StateName::FlowStart)
        }
        _ => unreachable!("expected eol/eof"),
    }
}

/// After flow.
///
/// ```markdown
/// > | # a␊
///        ^
/// ```
pub fn after(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        None => State::Ok,
        Some(b'\n') => {
            tokenizer.enter(Name::LineEnding);
            tokenizer.consume();
            tokenizer.exit(Name::LineEnding);
            State::Next(StateName::FlowStart)
        }
        _ => unreachable!("expected eol/eof"),
    }
}
