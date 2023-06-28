extern crate alloc;

pub mod configuration;
pub mod construct;
pub mod event;
pub mod mdast;
pub mod parser;
pub mod resolve;
pub mod state;
pub mod subtokenize;
pub mod to_html;
pub mod tokenizer;
pub mod unist;
pub mod util;

#[doc(hidden)]
pub use util::identifier::{id_cont, id_start};

#[doc(hidden)]
pub use util::sanitize_uri::sanitize;

#[doc(hidden)]
pub use util::location::Location;

pub use util::line_ending::LineEnding;

pub use configuration::{CompileOptions, Constructs, Options, ParseOptions};
