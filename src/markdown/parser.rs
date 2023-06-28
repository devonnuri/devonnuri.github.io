//! Turn bytes of markdown into events.

use crate::markdown::alloc::{string::String, vec, vec::Vec};
use crate::markdown::event::{Event, Point};
use crate::markdown::state::{Name as StateName, State};
use crate::markdown::subtokenize::subtokenize;
use crate::markdown::tokenizer::Tokenizer;
use crate::markdown::ParseOptions;

/// Info needed, in all content types, when parsing markdown.
///
/// Importantly, this contains a set of known definitions.
/// It also references the input value as bytes (`u8`).
#[derive(Debug)]
pub struct ParseState<'a> {
    /// Configuration.
    pub options: &'a ParseOptions,
    /// List of chars.
    pub bytes: &'a [u8],
    /// Set of defined definition identifiers.
    pub definitions: Vec<String>,
    /// Set of defined GFM footnote definition identifiers.
    pub gfm_footnote_definitions: Vec<String>,
}

/// Turn a string of markdown into events.
///
/// Passes the bytes back so the compiler can access the source.
pub fn parse<'a>(
    value: &'a str,
    options: &'a ParseOptions,
) -> Result<(Vec<Event>, ParseState<'a>), String> {
    let bytes = value.as_bytes();

    let mut parse_state = ParseState {
        options,
        bytes,
        definitions: vec![],
        gfm_footnote_definitions: vec![],
    };

    let start = Point {
        line: 1,
        column: 1,
        index: 0,
        vs: 0,
    };
    let mut tokenizer = Tokenizer::new(start, &parse_state);

    let state = tokenizer.push(
        (0, 0),
        (parse_state.bytes.len(), 0),
        State::Next(StateName::DocumentStart),
    );
    let mut result = tokenizer.flush(state, true)?;
    let mut events = tokenizer.events;

    loop {
        let fn_defs = &mut parse_state.gfm_footnote_definitions;
        let defs = &mut parse_state.definitions;
        fn_defs.append(&mut result.gfm_footnote_definitions);
        defs.append(&mut result.definitions);

        if result.done {
            return Ok((events, parse_state));
        }

        result = subtokenize(&mut events, &parse_state, &None)?;
    }
}
