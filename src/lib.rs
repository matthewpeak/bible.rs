//! A literary adaptation of all 66 books of the Protestant Bible.
//!
//! Direct quotations use the King James Version. Narration and Rust metaphors
//! are original adaptations. Each book runs in its own narrative context.
#![forbid(unsafe_code)]

pub mod books;
pub mod canon;
pub mod miracles;
pub mod scripture;
pub mod story;

/// There is, inevitably, a lifetime joke.
pub const THE_WORD: &str =
    "In the beginning was the Word, and the Word was with God, and the Word was God.";
