//! Non-lazy continuation.
//!
//! This is a tiny helper that [flow][] constructs can use to make sure that
//! the following line is not lazy.
//! For example, [html (flow)][html_flow] and ([raw (flow)][raw_flow],
//! [indented][code_indented]), stop when the next line is lazy.
//!
//! [flow]: crate::onnurmark::construct::flow
//! [raw_flow]: crate::onnurmark::construct::raw_flow
//! [code_indented]: crate::onnurmark::construct::code_indented
//! [html_flow]: crate::onnurmark::construct::html_flow

use crate::onnurmark::event::Name;
use crate::onnurmark::state::{Name as StateName, State};
use crate::onnurmark::tokenizer::Tokenizer;

/// At eol, before continuation.
///
/// ```markdown
/// > | * ```js
///            ^
///   | b
/// ```
pub fn start(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'\n') => {
            tokenizer.enter(Name::LineEnding);
            tokenizer.consume();
            tokenizer.exit(Name::LineEnding);
            State::Next(StateName::NonLazyContinuationAfter)
        }
        _ => State::Nok,
    }
}

/// A continuation.
///
/// ```markdown
///   | * ```js
/// > | b
///     ^
/// ```
pub fn after(tokenizer: &mut Tokenizer) -> State {
    if tokenizer.lazy {
        State::Nok
    } else {
        State::Ok
    }
}
