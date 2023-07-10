use crate::onnurmark::construct::partial_space_or_tab::space_or_tab;
use crate::onnurmark::event::{Content, Link, Name};
use crate::onnurmark::state::{Name as StateName, State};
use crate::onnurmark::tokenizer::Tokenizer;
use crate::onnurmark::util::constant::{ENVIRONMENT_SEQUENCE_SIZE_MIN, TAB_SIZE};

use super::partial_space_or_tab::space_or_tab_min_max;

/// Start of environment container.
///
/// ```markdown
/// > | ~~~theorem title="Pythagorean Theorem"
///     ^
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
///   | ~~~
/// ```
pub fn start(tokenizer: &mut Tokenizer) -> State {
    if matches!(tokenizer.current, Some(b'\t' | b' ')) {
        tokenizer.attempt(
            State::Next(StateName::EnvironmentBeforeSequenceOpen),
            State::Nok,
        );
        State::Retry(space_or_tab_min_max(
            tokenizer,
            1,
            if tokenizer.parse_state.options.constructs.code_indented {
                TAB_SIZE - 1
            } else {
                usize::MAX
            },
        ))
    } else {
        State::Retry(StateName::EnvironmentBeforeSequenceOpen)
    }
}

/// In opening fence, after prefix, at sequence.
///
/// ```markdown
/// > | ~~~theorem title="Pythagorean Theorem"
///     ^
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
///   | ~~~
/// ```
pub fn before_sequence_open(tokenizer: &mut Tokenizer) -> State {
    if matches!(tokenizer.current, Some(b'~')) {
        tokenizer.tokenize_state.marker = tokenizer.current.unwrap();
        tokenizer.tokenize_state.token_1 = Name::Environment;
        tokenizer.tokenize_state.token_2 = Name::EnvironmentFence;
        tokenizer.tokenize_state.token_3 = Name::EnvironmentFenceSequence;
        if tokenizer.tokenize_state.environment_opened {
            State::Retry(StateName::EnvironmentCloseStart)
        } else {
            tokenizer.tokenize_state.token_4 = Name::EnvironmentName;
            tokenizer.tokenize_state.token_5 = Name::EnvironmentOptions;

            tokenizer.enter(tokenizer.tokenize_state.token_1.clone());
            tokenizer.enter(tokenizer.tokenize_state.token_2.clone());
            tokenizer.enter(tokenizer.tokenize_state.token_3.clone());
            State::Retry(StateName::EnvironmentSequenceOpen)
        }
    } else {
        State::Nok
    }
}

/// In opening fence sequence.
///
/// ```markdown
/// > | ~~~theorem title="Pythagorean Theorem"
///      ^
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
///   | ~~~
/// ```
pub fn sequence_open(tokenizer: &mut Tokenizer) -> State {
    if tokenizer.current == Some(tokenizer.tokenize_state.marker) {
        tokenizer.tokenize_state.size += 1;
        tokenizer.consume();
        State::Next(StateName::EnvironmentSequenceOpen)
    } else if tokenizer.tokenize_state.size < ENVIRONMENT_SEQUENCE_SIZE_MIN {
        tokenizer.tokenize_state.marker = 0;
        tokenizer.tokenize_state.size = 0;
        tokenizer.tokenize_state.token_1 = Name::Data;
        tokenizer.tokenize_state.token_2 = Name::Data;
        tokenizer.tokenize_state.token_3 = Name::Data;
        tokenizer.tokenize_state.token_4 = Name::Data;
        tokenizer.tokenize_state.token_5 = Name::Data;
        State::Nok
    } else {
        tokenizer.exit(tokenizer.tokenize_state.token_3.clone());
        State::Retry(StateName::EnvironmentNameBefore)
    }
}

/// In opening fence, after the sequence (and optional whitespace), before info.
///
/// ```markdown
/// > | ~~~theorem title="Pythagorean Theorem"
///        ^
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
///   | ~~~
/// ```
pub fn name_before(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        None | Some(b'\n') => {
            tokenizer.exit(tokenizer.tokenize_state.token_2.clone());
            tokenizer.tokenize_state.environment_opened = true;
            State::Ok
        }
        _ => {
            tokenizer.enter(tokenizer.tokenize_state.token_4.clone());
            tokenizer.enter_link(
                Name::Data,
                Link {
                    previous: None,
                    next: None,
                    content: Content::String,
                },
            );
            State::Retry(StateName::EnvironmentName)
        }
    }
}

/// In name.
///
/// ```markdown
/// > | ~~~theorem title="Pythagorean Theorem"
///        ^
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
///   | ~~~
/// ```
pub fn name(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        None | Some(b'\n') => {
            tokenizer.exit(Name::Data);
            tokenizer.exit(tokenizer.tokenize_state.token_4.clone());
            State::Retry(StateName::EnvironmentNameBefore)
        }
        Some(b'\t' | b' ') => {
            tokenizer.exit(Name::Data);
            tokenizer.exit(tokenizer.tokenize_state.token_4.clone());
            tokenizer.attempt(State::Next(StateName::EnvironmentOptionsBefore), State::Nok);
            State::Retry(space_or_tab(tokenizer))
        }
        Some(_) => {
            // Note: no need to worry about `~` as strikethrough,
            // because 3 of them can‘t be used as strikethrough in text.
            tokenizer.consume();
            State::Next(StateName::EnvironmentName)
        }
    }
}

/// In opening fence, after info and whitespace, before meta.
///
/// ```markdown
/// > | ~~~theorem title="Pythagorean Theorem"
///                ^
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
///   | ~~~
/// ```
pub fn options_before(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        None | Some(b'\n') => State::Retry(StateName::EnvironmentNameBefore),
        _ => {
            tokenizer.enter(tokenizer.tokenize_state.token_5.clone());
            tokenizer.enter_link(
                Name::Data,
                Link {
                    previous: None,
                    next: None,
                    content: Content::String,
                },
            );
            State::Retry(StateName::EnvironmentOptions)
        }
    }
}

/// In meta.
///
/// ```markdown
/// > | ~~~theorem title="Pythagorean Theorem"
///                ^
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
///   | ~~~
/// ```
pub fn options(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        None | Some(b'\n') => {
            tokenizer.exit(Name::Data);
            tokenizer.exit(tokenizer.tokenize_state.token_5.clone());
            State::Retry(StateName::EnvironmentNameBefore)
        }
        Some(_) => {
            // Note: no need to worry about `~` as strikethrough,
            // because 3 of them can‘t be used as strikethrough in text.
            tokenizer.consume();
            State::Next(StateName::EnvironmentOptions)
        }
    }
}

/// Before closing fence.
///
/// ```markdown
///   | ~~~theorem title="Pythagorean Theorem"
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
/// > | ~~~
///     ^
/// ```
pub fn close_start(tokenizer: &mut Tokenizer) -> State {
    tokenizer.enter(tokenizer.tokenize_state.token_2.clone());

    State::Retry(StateName::EnvironmentBeforeSequenceClose)
}

/// In closing fence, after optional whitespace, at sequence.
///
/// ```markdown
///   | ~~~theorem title="Pythagorean Theorem"
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
/// > | ~~~
///     ^
/// ```
pub fn before_sequence_close(tokenizer: &mut Tokenizer) -> State {
    if tokenizer.current == Some(tokenizer.tokenize_state.marker) {
        tokenizer.enter(tokenizer.tokenize_state.token_3.clone());
        State::Retry(StateName::EnvironmentSequenceClose)
    } else {
        State::Nok
    }
}

/// In closing fence sequence.
///
/// ```markdown
///   | ~~~theorem title="Pythagorean Theorem"
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
/// > | ~~~
///     ^
/// ```
pub fn sequence_close(tokenizer: &mut Tokenizer) -> State {
    if tokenizer.current == Some(tokenizer.tokenize_state.marker) {
        tokenizer.tokenize_state.size_b += 1;
        tokenizer.consume();
        State::Next(StateName::EnvironmentSequenceClose)
    } else if tokenizer.tokenize_state.size_b >= tokenizer.tokenize_state.size {
        tokenizer.tokenize_state.size_b = 0;
        tokenizer.exit(tokenizer.tokenize_state.token_3.clone());

        if matches!(tokenizer.current, Some(b'\t' | b' ')) {
            tokenizer.attempt(
                State::Next(StateName::EnvironmentAfterSequenceClose),
                State::Nok,
            );
            State::Retry(space_or_tab(tokenizer))
        } else {
            State::Retry(StateName::EnvironmentAfterSequenceClose)
        }
    } else {
        tokenizer.tokenize_state.size_b = 0;
        State::Nok
    }
}

/// After closing fence sequence, after optional whitespace.
///
/// ```markdown
///   | ~~~theorem title="Pythagorean Theorem"
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
/// > | ~~~
///        ^
/// ```
pub fn sequence_close_after(tokenizer: &mut Tokenizer) -> State {
    match tokenizer.current {
        None | Some(b'\n') => {
            tokenizer.exit(tokenizer.tokenize_state.token_2.clone());
            State::Retry(StateName::EnvironmentAfter)
        }
        _ => State::Nok,
    }
}

/// After raw.
///
/// ```markdown
///   | ~~~theorem title="Pythagorean Theorem"
///   | The sum of the squares on the legs of a right triangle is equal to the square on the hypotenuse.
/// > | ~~~
///        ^
/// ```
pub fn after(tokenizer: &mut Tokenizer) -> State {
    tokenizer.exit(tokenizer.tokenize_state.token_1.clone());
    tokenizer.tokenize_state.environment_opened = false;
    tokenizer.tokenize_state.marker = 0;
    tokenizer.tokenize_state.size = 0;
    tokenizer.tokenize_state.token_1 = Name::Data;
    tokenizer.tokenize_state.token_2 = Name::Data;
    tokenizer.tokenize_state.token_3 = Name::Data;
    tokenizer.tokenize_state.token_4 = Name::Data;
    tokenizer.tokenize_state.token_5 = Name::Data;
    // Feel free to interrupt.
    tokenizer.interrupt = false;
    // No longer concrete.
    tokenizer.concrete = false;
    State::Ok
}
