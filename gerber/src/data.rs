//! Data types

use std::{borrow::Cow, cell::Cell, ops::Deref};

use crate::IResult;
use nom::{
    branch::alt,
    character::complete::{anychar, char, digit0, digit1, none_of, one_of},
    combinator::{map, not, opt, peek, recognize, verify},
    multi::{many0, many_m_n},
    sequence::{pair, preceded, terminated},
};

/// Parse a string into an i32
pub(crate) fn into_i32(x: &str) -> i32 {
    // TODO: optimize this knowing the caller always passes ASCII digits
    x.parse().unwrap()
}

/// Parse an non-negative integer to an i32
pub(crate) fn unsigned_integer(input: &str) -> IResult<i32> {
    map(digit1, into_i32)(input)
}

/// Parse a positive integer to an i32
pub(crate) fn positive_integer(input: &str) -> IResult<i32> {
    map(preceded(many0(char('0')), digit1), into_i32)(input)
}

/// Parse an integer to an i32
pub(crate) fn integer(input: &str) -> IResult<i32> {
    map(recognize(pair(opt(one_of("+-")), digit1)), into_i32)(input)
}

/// Parse a string into an f64
pub(crate) fn into_f64(x: &str) -> f64 {
    // TODO: optimize this knowing the caller always passes ASCII digits
    x.parse().unwrap()
}

/// Parse a positive decimal to an f64
pub(crate) fn unsigned_decimal(input: &str) -> IResult<f64> {
    map(
        alt((
            recognize(pair(digit1, opt(pair(char('.'), digit0)))),
            recognize(pair(char('.'), digit1)),
        )),
        into_f64,
    )(input)
}

/// Parse a decimal to an f64
pub(crate) fn decimal(input: &str) -> IResult<f64> {
    map(pair(opt(one_of("+-")), unsigned_decimal), |(sign, val)| {
        if sign == Some('-') {
            -val
        } else {
            val
        }
    })(input)
}

/// Aperture Identifier
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ApertureId(i32);

/// Convert an i32 into an ApertureId
pub(crate) fn into_aperture_id(x: i32) -> ApertureId {
    ApertureId(x)
}

/// Parse an aperture identifier
pub(crate) fn aperture_identifier(input: &str) -> IResult<ApertureId> {
    verify(
        map(preceded(char('D'), positive_integer), into_aperture_id),
        |id| id.0 >= 10,
    )(input)
}

/// Parse the first character in a name fragment (excludes '.')
pub(crate) fn name_fragment_first(input: &str) -> IResult<char> {
    verify(anychar, |&c| c.is_alphabetic() || c == '_' || c == '$')(input)
}

/// Parse non-first character in a name fragment (includes '.')
pub(crate) fn name_fragment_rest(input: &str) -> IResult<char> {
    verify(anychar, |&c| {
        c.is_alphanumeric() || c == '.' || c == '_' || c == '$'
    })(input)
}

/// Create a parser which parses a user defined name no longer than the provided `max` length
pub(crate) fn user_name_shorter_than(max: usize) -> impl Fn(&str) -> IResult<&str> {
    move |input| {
        if max == 0 {
            Ok((input, ""))
        } else {
            recognize(pair(
                // first user-defined name can't be a '.'
                name_fragment_first,
                terminated(
                    // remaining characters may include '.', but name can't be longer than max
                    many_m_n(0, max - 1, name_fragment_rest),
                    // ensure parsing stopped because of mismatch, not length
                    peek(not(name_fragment_rest)),
                ),
            ))(input)
        }
    }
}

/// Parse a user defined name
pub(crate) fn user_name(input: &str) -> IResult<&str> {
    user_name_shorter_than(127)(input)
}

/// Parse a system defined name
pub(crate) fn system_name(input: &str) -> IResult<&str> {
    // a system name just starts with a '.', but still can't be longer than 127 characters overall
    recognize(pair(char('.'), user_name_shorter_than(126)))(input)
}

/// Parse a system or user defined name
pub(crate) fn name(input: &str) -> IResult<&str> {
    alt((system_name, user_name))(input)
}

/// Strings in the Gerber specification may contain unicode escapes,
/// the expansion of which requires allocation. Allocating every string
/// would be inefficient, so EscapedString tracks if expansion is required
/// and delays that expansion until the `expand` function is called.
#[derive(Clone, PartialEq, PartialOrd, Debug, Hash)]
pub enum EscapedString<'a> {
    /// A string which does not contain escape sequences
    Unescaped(Cow<'a, str>),

    /// A string containing escape sequences
    Escaped(Cow<'a, str>),
}

impl<'a> EscapedString<'a> {
    /// Create an EscapedString which does not contain escape sequences
    pub fn new_unescaped(value: impl Into<Cow<'a, str>>) -> Self {
        Self::Unescaped(value.into())
    }

    /// Create an EscapedString which contains escape sequences
    pub fn new_escaped(value: impl Into<Cow<'a, str>>) -> Self {
        Self::Escaped(value.into())
    }

    /// Convert escape sequence, if present, and return the unescaped string
    pub fn unescape(&self) -> Cow<str> {
        match self {
            Self::Unescaped(s) => s.clone(),
            Self::Escaped(s) => {
                // TODO: expand unicode escape sequences
                s.clone()
            }
        }
    }
}

/// Parse a field
pub(crate) fn field(input: &str) -> IResult<EscapedString<'_>> {
    // TODO: use new_escaped if it contains escape sequences
    map(
        recognize(many0(none_of("%*,"))),
        EscapedString::new_unescaped,
    )(input)
}

/// Parse a string
pub(crate) fn string(input: &str) -> IResult<EscapedString<'_>> {
    // TODO: use new_escaped if it contains escape sequences
    map(
        recognize(many0(none_of("%*"))),
        EscapedString::new_unescaped,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integers() {
        // Unsigned Integers
        assert_eq!(unsigned_integer("0"), Ok(("", 0)));
        assert_eq!(unsigned_integer("123"), Ok(("", 123)));
        assert!(unsigned_integer("+123").is_err());
        assert!(unsigned_integer("-123").is_err());

        // Positive Integers
        assert!(positive_integer("0").is_err());
        assert_eq!(positive_integer("123"), Ok(("", 123)));
        // NOTE: grammar doesn't permit '+' on positive_integer (may want to relax this)
        assert!(positive_integer("+123").is_err());
        assert!(positive_integer("-123").is_err());

        // Integers
        assert_eq!(integer("0"), Ok(("", 0)));
        assert_eq!(integer("123"), Ok(("", 123)));
        assert_eq!(integer("+123"), Ok(("", 123)));
        assert_eq!(integer("-123"), Ok(("", -123)));
    }

    #[test]
    fn test_decimals() {
        // Unsigned Decimals
        assert_eq!(unsigned_decimal("0"), Ok(("", 0.)));
        assert_eq!(unsigned_decimal("0."), Ok(("", 0.)));
        assert_eq!(unsigned_decimal(".0"), Ok(("", 0.)));
        assert_eq!(unsigned_decimal("0.0"), Ok(("", 0.)));
        assert_eq!(unsigned_decimal("12.34"), Ok(("", 12.34)));
        assert!(unsigned_decimal(".").is_err());

        // Decimals
        assert_eq!(decimal("0"), Ok(("", 0.)));
        assert_eq!(decimal("0."), Ok(("", 0.)));
        assert_eq!(decimal(".0"), Ok(("", 0.)));
        assert_eq!(decimal("0.0"), Ok(("", 0.)));
        assert_eq!(decimal("1"), Ok(("", 1.)));
        assert_eq!(decimal("1."), Ok(("", 1.)));
        assert_eq!(decimal(".1"), Ok(("", 0.1)));
        assert_eq!(decimal("1.0"), Ok(("", 1.)));
        assert_eq!(decimal("-1"), Ok(("", -1.)));
        assert_eq!(decimal("-1."), Ok(("", -1.)));
        assert_eq!(decimal("-.1"), Ok(("", -0.1)));
        assert_eq!(decimal("-1.0"), Ok(("", -1.)));
        assert!(decimal(".").is_err());
    }

    #[test]
    fn test_aperture_id() {
        assert_eq!(aperture_identifier("D0123"), Ok(("", ApertureId(123))));
    }

    #[test]
    fn test_name() {
        // User-defined Name
        assert_eq!(user_name("foo!"), Ok(("!", "foo")));
        assert_eq!(user_name("_"), Ok(("", "_")));
        assert_eq!(user_name("$"), Ok(("", "$")));
        assert_eq!(user_name("a"), Ok(("", "a")));
        assert_eq!(user_name("A"), Ok(("", "A")));
        assert_eq!(user_name("__$Some.01__Name"), Ok(("", "__$Some.01__Name")));

        let valid_long = "x".repeat(127);
        assert_eq!(
            user_name(valid_long.as_str()),
            Ok(("", valid_long.as_str()))
        );

        let invalid_long = "x".repeat(128);
        assert!(user_name(invalid_long.as_str()).is_err());

        assert!(user_name(".Nope").is_err());
        assert!(user_name("1Nope").is_err());

        // System-defined Name
        assert_eq!(system_name(".foo!"), Ok(("!", ".foo")));
        assert_eq!(system_name("._"), Ok(("", "._")));
        assert_eq!(system_name(".$"), Ok(("", ".$")));
        assert_eq!(system_name(".a"), Ok(("", ".a")));
        assert_eq!(system_name(".A"), Ok(("", ".A")));
        assert_eq!(
            system_name(".__$Some.01__Name"),
            Ok(("", ".__$Some.01__Name"))
        );

        let valid_long = format!(".{}", "x".repeat(126));
        assert_eq!(
            system_name(valid_long.as_str()),
            Ok(("", valid_long.as_str()))
        );

        let invalid_long = format!(".{}", "x".repeat(127));
        assert!(system_name(invalid_long.as_str()).is_err());

        assert!(system_name("Nope").is_err());
        assert!(system_name(".1Nope").is_err());
    }

    #[test]
    fn test_field() {
        let valid_field = "Can be anything 😀; except for a comma!\nEven a newline is ok.";
        assert_eq!(
            field(valid_field),
            Ok(("", EscapedString::new_unescaped(valid_field)))
        );

        let with_comma = "before,after";
        assert_eq!(
            field(with_comma),
            Ok((",after", EscapedString::new_unescaped("before")))
        );

        let with_percent = "before%after";
        assert_eq!(
            field(with_percent),
            Ok(("%after", EscapedString::new_unescaped("before")))
        );

        let with_star = "before*after";
        assert_eq!(
            field(with_star),
            Ok(("*after", EscapedString::new_unescaped("before")))
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(string(""), Ok(("", EscapedString::new_unescaped(""))));
        assert_eq!(string(" "), Ok(("", EscapedString::new_unescaped(" "))));
        assert_eq!(
            string("a\nb"),
            Ok(("", EscapedString::new_unescaped("a\nb")))
        );

        let with_percent = "before%after";
        assert_eq!(
            string(with_percent),
            Ok(("%after", EscapedString::new_unescaped("before")))
        );

        let with_star = "before*after";
        assert_eq!(
            string(with_star),
            Ok(("*after", EscapedString::new_unescaped("before")))
        );
    }
}
