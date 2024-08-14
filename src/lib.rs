//! Parse a gerber file using `nom`.
//!
//! The official grammar provided by Ucamco is a PEG, so conversion to a
//! parser-combinator is the quickest approach to get this working.
//!
//! Initially I started with `logos` to build a traditional recursive-decent
//! parser. However there is a lot of overlap with the tokens in the defined
//! grammar which would need the parser to set modes on the lexer, which
//! doesn't fit `logos` model. Refactoring the grammar to avoid this became
//! more trouble than it was worth.
//!
//! Because the majority of tokens in gerber are ASCII, a SIMD-based classification
//! and hand-rolled parser will ultimately provide the best performance. But
//! that will be left for a future revision.

pub mod data_types;

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

