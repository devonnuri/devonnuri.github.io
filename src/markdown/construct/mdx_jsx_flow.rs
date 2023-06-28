//! MDX JSX (flow) occurs in the [flow][] content type.
//!
//! ## Grammar
//!
//! MDX JSX (flow) forms with the following BNF
//! (<small>see [construct][crate::construct] for character groups</small>):
//!
//! ```bnf
//! mdx_jsx_flow ::= mdx_jsx *space_or_tab [mdx_jsx *space_or_tab]
//!
//! ; See the `partial_mdx_jsx` construct for the BNF of that part.
//! ```
//!
//! As this construct occurs in flow, like all flow constructs, it must be
//! followed by an eol (line ending) or eof (end of file).
//! It is allowed to use multiple tags after each other, optionally with only
//! whitespace between them.
//!
//! See [`mdx_jsx`][mdx_jsx] for more info.
//!
//! ## Tokens
//!
//! *   [`MdxJsxFlowTag`][Name::MdxJsxFlowTag]
//! *   [`SpaceOrTab`][Name::SpaceOrTab]
//! *   see [`mdx_jsx`][mdx_jsx] for more
//!
//! ## Recommendation
//!
//! See [`mdx_jsx`][mdx_jsx] for recommendations.
//!
//! ## References
//!
//! *   [`jsx-flow.js` in `micromark-extension-mdx-jsx`](https://github.com/micromark/micromark-extension-mdx-jsx/blob/main/dev/lib/jsx-flow.js)
//! *   [`mdxjs.com`](https://mdxjs.com)
//!
//! [flow]: crate::construct::flow
//! [mdx_jsx]: crate::construct::partial_mdx_jsx

use crate::markdown::construct::partial_space_or_tab::{space_or_tab, space_or_tab_min_max};
use crate::markdown::event::Name;
use crate::markdown::state::{Name as StateName, State};
use crate::markdown::tokenizer::Tokenizer;
use crate::markdown::util::constant::TAB_SIZE;

/// Start of MDX: JSX (flow).
///
/// ```markdown
/// > | <A />
///     ^
/// ```
pub fn start(tokenizer: &mut Tokenizer) -> State {
    if tokenizer.parse_state.options.constructs.mdx_jsx_flow {
        tokenizer.tokenize_state.token_1 = Name::MdxJsxFlowTag;
        tokenizer.concrete = true;
        if matches!(tokenizer.current, Some(b'\t' | b' ')) {
            tokenizer.attempt(State::Next(StateName::MdxJsxFlowBefore), State::Nok);
            State::Retry(space_or_tab_min_max(
                tokenizer,
                0,
                if tokenizer.parse_state.options.constructs.code_indented {
                    TAB_SIZE - 1
                } else {
                    usize::MAX
                },
            ))
        } else {
            State::Retry(StateName::MdxJsxFlowBefore)
        }
    } else {
        State::Nok
    }
}

/// After optional whitespace, before of MDX JSX (flow).
///
/// ```markdown
/// > | <A />
///     ^
/// ```
pub fn before(tokenizer: &mut Tokenizer) -> State {
    if Some(b'<') == tokenizer.current {
        tokenizer.attempt(
            State::Next(StateName::MdxJsxFlowAfter),
            State::Next(StateName::MdxJsxFlowNok),
        );
        State::Retry(StateName::MdxJsxStart)
    } else {
        State::Retry(StateName::MdxJsxFlowNok)
    }
}

/// After an MDX JSX (flow) tag.
///
/// ```markdown
/// > | <A>
///        ^
/// ```
pub fn after(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'\t' | b' ') => {
            tokenizer.attempt(State::Next(StateName::MdxJsxFlowEnd), State::Nok);
            State::Retry(space_or_tab(tokenizer))
        }
        _ => State::Retry(StateName::MdxJsxFlowEnd),
    }
}

/// After an MDX JSX (flow) tag, after optional whitespace.
///
/// ```markdown
/// > | <A> <B>
///         ^
/// ```
pub fn end(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        None | Some(b'\n') => {
            reset(tokenizer);
            State::Ok
        }
        // Another?
        Some(b'<') => {
            tokenizer.attempt(
                State::Next(StateName::MdxJsxFlowAfter),
                State::Next(StateName::MdxJsxFlowNok),
            );
            State::Retry(StateName::MdxJsxStart)
        }
        _ => {
            reset(tokenizer);
            State::Nok
        }
    }
}

/// At something that wasn’t an MDX JSX (flow) tag.
///
/// ```markdown
/// > | <A> x
///     ^
/// ```
pub fn nok(tokenizer: &mut Tokenizer) -> State {
    reset(tokenizer);
    State::Nok
}

/// Reset state.
fn reset(tokenizer: &mut Tokenizer) {
    tokenizer.concrete = false;
    tokenizer.tokenize_state.token_1 = Name::Data;
}
