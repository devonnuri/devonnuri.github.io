extern crate alloc;

pub mod construct;
pub mod util;
pub mod configuration;
pub mod event;
pub mod mdast;
pub mod parser;
pub mod resolve;
pub mod state;
pub mod subtokenize;
pub mod to_html;
pub mod to_mdast;
pub mod tokenizer;
pub mod unist;

#[doc(hidden)]
pub use util::identifier::{id_cont, id_start};

#[doc(hidden)]
pub use util::sanitize_uri::sanitize;

#[doc(hidden)]
pub use util::location::Location;

pub use util::line_ending::LineEnding;

pub use util::mdx::{
    EsmParse as MdxEsmParse, ExpressionKind as MdxExpressionKind,
    ExpressionParse as MdxExpressionParse, Signal as MdxSignal,
};

pub use configuration::{CompileOptions, Constructs, Options, ParseOptions};