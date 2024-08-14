# Gerber Parser

The goal of this parser is to parse Gerber X3 layer files.
Performance optimization and lossless parsing are not goals until everything in
the spec can be parsed correctly.

## Status

Incomplete.

* Can parse all data types described in section 3.4 of the 2024.05 spec
* Accepts, but does not expand unicode escapes in strings and fields
  (/u0000 and /U000000000)


## Reference


* [Ucamco Downloads](https://www.ucamco.com/en/gerber/downloads), specifically
  "Gerber Layer Format Specification - Revision 2024.05"
* [nom](https://docs.rs/nom/latest/nom/) parser-combinator docs
