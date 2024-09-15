use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::{branch::alt, combinator::map};

use crate::{system_name, user_name, IResult};

#[derive(Clone, PartialEq)]
pub enum FileAttributeName<'a> {
    Part,
    FileFunction,
    FilePolarity,
    SameCoordinates,
    CreationDate,
    GenerationSoftware,
    ProjectId,
    MD5,
    UnknownStandardName(&'a str),
    UserDefinedName(&'a str),
}

impl<'a> FileAttributeName<'a> {
    pub(crate) fn parse(input: &'a str) -> IResult<Self> {
        alt((
            value(Self::Part, tag(".Part")),
            value(Self::FileFunction, tag(".FileFunction")),
            value(Self::FilePolarity, tag(".FilePolarity")),
            value(Self::SameCoordinates, tag(".SameCoordinates")),
            value(Self::CreationDate, tag(".CreationDate")),
            value(Self::GenerationSoftware, tag(".GenerationSoftware")),
            value(Self::ProjectId, tag(".ProjectId")),
            value(Self::MD5, tag(".MD5")),
            map(system_name, |s| Self::UnknownStandardName(s)),
            map(user_name, |s| Self::UserDefinedName(s)),
        ))(input)
    }
}
