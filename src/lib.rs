//! Parse a gerber file using [nom](https://crates.io/crates/nom).
//!
//! ## Implementation Notes
//! 
//! The official grammar[^1] provided by Ucamco is a PEG, so conversion to a
//! parser-combinator is the quickest approach to a functional parser[^2].
//!
//! The majority of tokens in gerber are ASCII, thus a SIMD-based classifier
//! and hand-rolled parser will provide higher throughput than this `nom` approach.
//! But that will be left for a future revision since fully supporting the spec
//! is more valuable than creating the fastest parser for just a part of it.
//!
//! Initially I started with the [logos](https://crates.io/crates/logos) lexer
//! on the path toward building a traditional recursive-decent parser. However
//! there is a lot of overlap between tokens in the PEG grammar which would
//! require the parser to set modes on the lexer, which doesn't fit the `logos`
//! model. Since using a parser generator is merely a stepping stone before
//! writing a hand-rolled parser, and refactoring the grammar to avoid the issue
//! was moving much of the work out of the lexer and into the parser, that approach
//! was abandoned.
//! 
//! [^1]: "Gerber Layer Format Specification - Revision 2024.05" from
//!   [Ucamco Downloads](https://www.ucamco.com/en/gerber/downloads)
//! [^2]: Groan... I didn't notice the pun until later.

pub mod command;
pub mod primitive;

use crate::command::Command;

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub fn parse(input: &str) -> IResult<Vec<Command>> {
    todo!();
}
