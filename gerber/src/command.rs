//! Commands and aliases

use crate::IResult;
use nom::{
    bytes::complete::tag,
    combinator::{map, value},
    sequence::{delimited, pair},
};
pub use Command::*;

/// Gerber Commands
///
/// Each variant is the "long name" listed in §2.8 of the specification.
/// Variants are also identified by [command code constants](crate::command#constants).
#[derive(Clone, PartialEq, PartialOrd, Hash, Debug)]
pub enum Command {
    /// [G04] A human readable comment, does not affect the image.
    Comment, // TODO: add comment string

    /// [MO] Sets the unit to mm or inch.
    Mode,

    /// [FS] Sets the coordinate format, e.g. the number of decimals.
    FormatSpecification,

    /// [AD] Defines a template-based aperture, assigns a D code to it.
    ApertureDefine,

    /// [AM] Defines a macro aperture template.
    ApertureMacro,

    /// [D] (Dnn for nn≥10) Sets the current aperture to D code nn.
    SetCurrentAperture,

    /// [D01] Outside a region statement [D01] creates a draw or arc
    /// object with the current aperture. Inside it adds a draw/arc
    /// segment to the contour under construction. The current
    /// point is moved to draw/arc end point after the creation of
    /// the draw/arc.
    Plot,

    /// [D02] Moves the current point to the coordinate in the
    /// command. It does not create an object.
    Move,

    /// [D03] Creates a flash object with the current aperture. The
    /// current point is moved to the flash point.
    Flash,

    /// [G01] Sets linear/circular mode to linear.
    SetLinear,

    /// [G02] Sets linear/circular mode to clockwise circular.
    SetCWCircular,

    /// [G03] Sets linear/circular mode to counterclockwise circular.
    SetCCWCircular,

    /// [G75] Must be called before creating the first arc.
    ArcInit,

    /// [LP] Loads the polarity object transformation parameter.
    LoadPolarity,

    /// [LM] Loads the mirror object transformation parameter.
    LoadMirroring,

    /// [LR] Loads the rotation object transformation parameter.
    LoadRotation,

    /// [LS] Loads the scale object transformation parameter.
    LoadScaling,

    /// [G36] Starts a region statement which creates a region by
    /// defining its contours.
    StartRegion,

    /// [G37] Ends the region statement.
    EndRegion,

    /// [AB] Opens a block aperture statement and assigns its aperture
    /// number or closes a block aperture statement.
    ApertureBlock,

    /// [SR] Open or closes a step and repeat statement.
    StepAndRepeat,

    /// [TF] Set a file attribute.
    AttributeOnFile,

    /// [TA] Add an aperture attribute to the dictionary or modify it.
    AttributeOnAperture,

    /// [TO] Add an object attribute to the dictionary or modify it.
    AttributeOnObject,

    /// [TD] Delete one or all attributes in the dictionary.
    AttributeDelete,

    /// [M02] End of file.
    EndOfFile,
}

pub(crate) fn extended_command<'a, T>(
    code: &'static str,
    parser: impl FnMut(&'a str) -> IResult<'a, T>,
    command: impl Fn(T) -> Command,
) -> impl FnMut(&'a str) -> IResult<'a, Command> {
    map(
        delimited(pair(tag("%"), tag(code)), parser, tag("*%")),
        command,
    )
}

pub(crate) fn word_command<'a, T>(
    code: &'static str,
    parser: impl FnMut(&'a str) -> IResult<'a, T>,
    command: impl Fn(T) -> Command,
) -> impl FnMut(&'a str) -> IResult<'a, Command> {
    map(delimited(tag(code), parser, tag("*")), command)
}

pub(crate) fn simple_word_command<'a>(
    code: &'static str,
    command: Command,
) -> impl FnMut(&'a str) -> IResult<'a, Command> {
    value(command, pair(tag(code), tag("*")))
}
