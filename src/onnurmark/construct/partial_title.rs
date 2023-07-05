//! Title occurs in [definition][] and [label end][label_end].
//!
//! ## Grammar
//!
//! Title forms with the following BNF
//! (<small>see [construct][crate::construct] for character groups</small>):
//!
//! ```bnf
//! ; Restriction: no blank lines.
//! ; Restriction: markers must match (in case of `(` with `)`).
//! title ::= marker *(title_byte | title_escape) marker
//! title_byte ::= code - '\\' - marker
//! title_escape ::= '\\' ['\\' | marker]
//! marker ::= '"' | '\'' | '('
//! ```
//!
//! Titles can be double quoted (`"a"`), single quoted (`'a'`), or
//! parenthesized (`(a)`).
//!
//! Titles can contain line endings and whitespace, but they are not allowed to
//! contain blank lines.
//! They are allowed to be blank themselves.
//!
//! The title is interpreted as the [string][] content type.
//! That means that [character escapes][character_escape] and
//! [character references][character_reference] are allowed.
//!
//! ## References
//!
//! *   [`micromark-factory-title/index.js` in `micromark`](https://github.com/micromark/micromark/blob/main/packages/micromark-factory-title/dev/index.js)
//!
//! [definition]: crate::construct::definition
//! [string]: crate::construct::string
//! [character_escape]: crate::construct::character_escape
//! [character_reference]: crate::construct::character_reference
//! [label_end]: crate::construct::label_end

use crate::onnurmark::construct::partial_space_or_tab_eol::{
    space_or_tab_eol_with_options, Options,
};
use crate::onnurmark::event::{Content, Link, Name};
use crate::onnurmark::state::{Name as StateName, State};
use crate::onnurmark::subtokenize::link;
use crate::onnurmark::tokenizer::Tokenizer;

/// Start of title.
///
/// ```markdown
/// > | "a"
///     ^
/// ```
pub fn start(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'"' | b'\'' | b'(') => {
            let marker = tokenizer.current.unwrap();
            tokenizer.tokenize_state.marker = if marker == b'(' { b')' } else { marker };
            tokenizer.enter(tokenizer.tokenize_state.token_1.clone());
            tokenizer.enter(tokenizer.tokenize_state.token_2.clone());
            tokenizer.consume();
            tokenizer.exit(tokenizer.tokenize_state.token_2.clone());
            State::Next(StateName::TitleBegin)
        }
        _ => State::Nok,
    }
}

/// After opening marker.
///
/// This is also used at the closing marker.
///
/// ```markdown
/// > | "a"
///      ^
/// ```
pub fn begin(tokenizer: &mut Tokenizer) -> State {
    if tokenizer.current == Some(tokenizer.tokenize_state.marker) {
        tokenizer.enter(tokenizer.tokenize_state.token_2.clone());
        tokenizer.consume();
        tokenizer.exit(tokenizer.tokenize_state.token_2.clone());
        tokenizer.exit(tokenizer.tokenize_state.token_1.clone());
        tokenizer.tokenize_state.marker = 0;
        tokenizer.tokenize_state.connect = false;
        State::Ok
    } else {
        tokenizer.enter(tokenizer.tokenize_state.token_3.clone());
        State::Retry(StateName::TitleAtBreak)
    }
}

/// At something, before something else.
///
/// ```markdown
/// > | "a"
///      ^
/// ```
pub fn at_break(tokenizer: &mut Tokenizer) -> State {
    if let Some(byte) = tokenizer.current {
        if byte == tokenizer.tokenize_state.marker {
            tokenizer.exit(tokenizer.tokenize_state.token_3.clone());
            State::Retry(StateName::TitleBegin)
        } else if byte == b'\n' {
            tokenizer.attempt(
                State::Next(StateName::TitleAfterEol),
                State::Next(StateName::TitleNok),
            );
            State::Retry(space_or_tab_eol_with_options(
                tokenizer,
                Options {
                    content: Some(Content::String),
                    connect: tokenizer.tokenize_state.connect,
                },
            ))
        } else {
            tokenizer.enter_link(
                Name::Data,
                Link {
                    previous: None,
                    next: None,
                    content: Content::String,
                },
            );

            if tokenizer.tokenize_state.connect {
                let index = tokenizer.events.len() - 1;
                link(&mut tokenizer.events, index);
            } else {
                tokenizer.tokenize_state.connect = true;
            }

            State::Retry(StateName::TitleInside)
        }
    } else {
        State::Retry(StateName::TitleNok)
    }
}

/// In title, after whitespace.
///
/// ```markdown
///   | "a␊
/// > | b"
///     ^
/// ```
pub fn after_eol(tokenizer: &mut Tokenizer) -> State {
    tokenizer.tokenize_state.connect = true;
    State::Retry(StateName::TitleAtBreak)
}

/// In title, at something that isn’t allowed.
///
/// ```markdown
/// > | "a
///       ^
/// ```
pub fn nok(tokenizer: &mut Tokenizer) -> State {
    tokenizer.tokenize_state.marker = 0;
    tokenizer.tokenize_state.connect = false;
    State::Nok
}

/// In text.
///
/// ```markdown
/// > | "a"
///      ^
/// ```
pub fn inside(tokenizer: &mut Tokenizer) -> State {
    if tokenizer.current == Some(tokenizer.tokenize_state.marker)
        || matches!(tokenizer.current, None | Some(b'\n'))
    {
        tokenizer.exit(Name::Data);
        State::Retry(StateName::TitleAtBreak)
    } else {
        let name = if tokenizer.current == Some(b'\\') {
            StateName::TitleEscape
        } else {
            StateName::TitleInside
        };
        tokenizer.consume();
        State::Next(name)
    }
}

/// After `\`, at a special character.
///
/// ```markdown
/// > | "a\*b"
///      ^
/// ```
pub fn escape(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        Some(b'"' | b'\'' | b')' | b'\\') => {
            tokenizer.consume();
            State::Next(StateName::TitleInside)
        }
        _ => State::Retry(StateName::TitleInside),
    }
}
