//! abstract syntax trees: [unist][].
//!
//! [unist]: https://github.com/syntax-tree/unist

use crate::onnurmark::alloc::fmt;

/// One place in a source file.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    /// 1-indexed integer representing a line in a source file.
    pub line: usize,
    /// 1-indexed integer representing a column in a source file.
    pub column: usize,
    /// 0-indexed integer representing a character in a source file.
    pub offset: usize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} ({})", self.line, self.column, self.offset)
    }
}

/// Location of a node in a source file.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    /// Represents the place of the first character of the parsed source region.
    pub start: Point,
    /// Represents the place of the first character after the parsed source
    /// region, whether it exists or not.
    pub end: Point,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}-{}:{} ({}-{})",
            self.start.line,
            self.start.column,
            self.end.line,
            self.end.column,
            self.start.offset,
            self.end.offset
        )
    }
}
