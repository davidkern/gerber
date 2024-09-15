//! Parse a gerber file using [nom](https://crates.io/crates/nom).
//!
//! ## Current Limitations
//!
//! * Does not implement the full specification
//! * Does not expand unicode escape sequences
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

pub mod attribute;
pub mod command;
pub mod data;

use attribute::FileAttributeName;
use thiserror::Error;

use crate::command::Command::{self, *};
use crate::data::*;
use nom::character::complete::char;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, line_ending, one_of},
    combinator::{all_consuming, map, map_res, opt, recognize, value},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    Err,
};

pub(crate) type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Error, Debug)]
pub enum GerberError {
    #[error("coodinate digits invalid")]
    CoodinateDigits,
}

/// Parse a gerber file into a list of [Command]s
pub fn gerber(input: &str) -> IResult<Vec<Command>> {
    map(
        all_consuming(pair(
            many0(delimited(
                many0(line_ending),
                alt((
                    comment,
                    mode,
                    format_specification,
                    // aperture_define,
                    // aperture_macro,
                    // set_current_aperture,
                    arc_init,
                    set_linear,
                    set_cw_circular,
                    set_ccw_circular,
                    // load_polarity,
                    // load_mirroring,
                    // load_rotation,
                    // load_scaling,
                    // region_statement,
                    // ab_statement,
                    // sr_statement,
                    attribute_on_file,
                    // attribute_on_aperture,
                    // attribute_on_object,
                    // attribute_delete,
                )),
                many0(line_ending),
            )),
            terminated(end_of_file, many0(line_ending)),
        )),
        // include the EndOfFile command in the list
        |(mut commands, eof)| {
            commands.push(eof);
            commands
        },
    )(input)
}

fn comment(input: &str) -> IResult<Command> {
    map(delimited(tag("G04"), string, char('*')), |_| Comment)(input)
}

fn mode(input: &str) -> IResult<Command> {
    map(
        delimited(tag("%MO"), alt((tag("MM"), tag("IN"))), tag("*%")),
        |_| Mode,
    )(input)
}

fn coordinate_digits(input: &str) -> IResult<u8> {
    map_res(pair(anychar, char('6')), |(x, _)| match x {
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        _ => Err(GerberError::CoodinateDigits),
    })(input)
}

fn format_specification(input: &str) -> IResult<Command> {
    map(
        delimited(
            tag("%FSLAX"),
            separated_pair(coordinate_digits, tag("Y"), coordinate_digits),
            tag("*%"),
        ),
        |(_, _)| FormatSpecification,
    )(input)
}

fn aperture_define_circle(input: &str) -> IResult<Command> {
    map(
        delimited(
            delimited(
                tag("%AD"),
                aperture_identifier,
                pair(tag("C,"), many0(line_ending)),
            ),
            pair(decimal, opt(preceded(char('X'), decimal))),
            tag("*%"),
        ),
        |(_, _)| ApertureDefine,
    )(input)
}

fn aperture_define_rectangle(input: &str) -> IResult<Command> {
    map(
        delimited(
            delimited(
                tag("%AD"),
                aperture_identifier,
                pair(tag("R,"), many0(line_ending)),
            ),
            pair(
                separated_pair(decimal, char('X'), decimal),
                opt(preceded(char('X'), decimal)),
            ),
            tag("*%"),
        ),
        |(_, _)| ApertureDefine,
    )(input)
}

fn aperture_define_obround(input: &str) -> IResult<Command> {
    map(
        delimited(
            delimited(
                tag("%AD"),
                aperture_identifier,
                pair(tag("O,"), many0(line_ending)),
            ),
            pair(
                separated_pair(decimal, char('X'), decimal),
                opt(preceded(char('X'), decimal)),
            ),
            tag("*%"),
        ),
        |(_, _)| ApertureDefine,
    )(input)
}

fn aperture_define_polygon(input: &str) -> IResult<Command> {
    map(
        delimited(
            delimited(
                tag("%AD"),
                aperture_identifier,
                pair(tag("P,"), many0(line_ending)),
            ),
            pair(
                separated_pair(decimal, char('X'), decimal),
                opt(preceded(
                    char('X'),
                    pair(decimal, opt(preceded(char('X'), decimal))),
                )),
            ),
            tag("*%"),
        ),
        |(_, _)| ApertureDefine,
    )(input)
}

fn aperture_define_macro(input: &str) -> IResult<Command> {
    map(
        delimited(
            tag("%AD"),
            tuple((
                aperture_identifier,
                name,
                opt(preceded(
                    char(','),
                    pair(decimal, opt(preceded(char('X'), decimal)))
                )),
            )),
            tag("*%"),
        ),
        |(_, _, _)| ApertureDefine,
    )(input)
}

fn aperture_define(input: &str) -> IResult<Command> {
    alt((
        aperture_define_circle,
        aperture_define_rectangle,
        aperture_define_obround,
        aperture_define_polygon,
        aperture_define_macro,
    ))(input)
}

fn aperture_macro(input: &str) -> IResult<Command> {
    todo!()
}

fn set_current_aperture(input: &str) -> IResult<Command> {
    todo!()
}

fn arc_init(input: &str) -> IResult<Command> {
    value(ArcInit, tag("G75*"))(input)
}

fn set_linear(input: &str) -> IResult<Command> {
    value(SetLinear, tag("G01*"))(input)
}

fn set_cw_circular(input: &str) -> IResult<Command> {
    value(SetCWCircular, tag("G02*"))(input)
}

fn set_ccw_circular(input: &str) -> IResult<Command> {
    value(SetCCWCircular, tag("G03*"))(input)
}

fn plot_operation(input: &str) -> IResult<Command> {
    todo!()
}

fn move_operation(input: &str) -> IResult<Command> {
    todo!()
}

fn load_polarity(input: &str) -> IResult<Command> {
    todo!()
}

fn load_mirroring(input: &str) -> IResult<Command> {
    todo!()
}

fn load_rotation(input: &str) -> IResult<Command> {
    todo!()
}

fn load_scaling(input: &str) -> IResult<Command> {
    todo!()
}

fn region_statement(input: &str) -> IResult<Command> {
    todo!()
}

fn ab_statement(input: &str) -> IResult<Command> {
    todo!()
}

fn sr_statement(input: &str) -> IResult<Command> {
    todo!()
}

fn attribute_on_file(input: &str) -> IResult<Command> {
    map(
        delimited(
            tag("%TF"),
            pair(
                FileAttributeName::parse,
                many0(preceded(tag(","), field))
            ),
            tag("*%")
        ),
        |_| AttributeOnFile
    )(input)
}

fn attribute_on_aperture(input: &str) -> IResult<Command> {
    todo!()
}

fn attribute_on_object(input: &str) -> IResult<Command> {
    todo!()
}

fn attribute_delete(input: &str) -> IResult<Command> {
    todo!()
}

fn end_of_file(input: &str) -> IResult<Command> {
    value(EndOfFile, tag("M02*"))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        assert_eq!(
            // gerber(indoc! {"
            //     G04 Different command styles*
            //     %FSLAX26Y26*%
            //     %MOMM*%
            //     %AMDonut*
            //     1,1,$1,$2,$3*
            //     $4=$1x0.75*
            //     1,0,$4,$2,$3*
            //     %
            //     %ADD11Donut,0.30X0X0*%
            //     %ADD10C,0.1*%
            //     G75*
            //     G02*
            //     D10*
            //     X0Y0D02*
            //     X2000000Y0I1000000J0D01*
            //     D11*
            //     X0Y2000000D03*
            //     M02*
            // "}),
            gerber(indoc! {"
                G04 Different command styles*
                %FSLAX26Y26*%
                %MOMM*%
                M02*
            "}),
            Ok(("", vec![Comment, FormatSpecification, Mode, EndOfFile,]))
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(comment("G04 Single line comment*"), Ok(("", Comment)));
        assert_eq!(comment("G04*"), Ok(("", Comment)));
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode("%MOMM*%"), Ok(("", Mode)));
        assert_eq!(mode("%MOIN*%"), Ok(("", Mode)));
    }

    #[test]
    fn test_coordinate_digits() {
        assert!(coordinate_digits("06").is_err());
        assert_eq!(coordinate_digits("16"), Ok(("", 1)));
        assert_eq!(coordinate_digits("26"), Ok(("", 2)));
        assert_eq!(coordinate_digits("36"), Ok(("", 3)));
        assert_eq!(coordinate_digits("46"), Ok(("", 4)));
        assert_eq!(coordinate_digits("56"), Ok(("", 5)));
        assert_eq!(coordinate_digits("66"), Ok(("", 6)));
        assert!(coordinate_digits("76").is_err());
        assert!(coordinate_digits("18").is_err());
    }

    #[test]
    fn test_format_specification() {
        assert_eq!(
            format_specification("%FSLAX16Y66*%"),
            Ok(("", FormatSpecification))
        )
    }

    #[test]
    fn test_set_linear() {
        assert_eq!(set_linear("G01*"), Ok(("", SetLinear)));
    }

    #[test]
    fn test_set_cw_circular() {
        assert_eq!(set_cw_circular("G02*"), Ok(("", SetCWCircular)));
    }

    #[test]
    fn test_set_ccw_circular() {
        assert_eq!(set_ccw_circular("G03*"), Ok(("", SetCCWCircular)));
    }

    #[test]
    fn test_arc_init() {
        assert_eq!(arc_init("G75*"), Ok(("", ArcInit)));
    }

    #[test]
    fn test_aperture_define() {
        assert_eq!(aperture_define("%ADD10C,0.1*%"), Ok(("", ApertureDefine)));
        assert_eq!(aperture_define("%ADD11C,0.6*%"), Ok(("", ApertureDefine)));
        assert_eq!(
            aperture_define("%ADD12R,0.6X0.6*%"),
            Ok(("", ApertureDefine))
        );
        assert_eq!(
            aperture_define("%ADD13R,0.4X1.00*%"),
            Ok(("", ApertureDefine))
        );
        assert_eq!(
            aperture_define("%ADD14R,1.00X0.4*%"),
            Ok(("", ApertureDefine))
        );
        assert_eq!(
            aperture_define("%ADD15O,0.4X01.00*%"),
            Ok(("", ApertureDefine))
        );
        assert_eq!(
            aperture_define("%ADD16P,1.00X3*%"),
            Ok(("", ApertureDefine))
        );
        assert_eq!(
            aperture_define("%ADD19THERMAL80*%"),
            Ok(("", ApertureDefine))
        );
    }
}
