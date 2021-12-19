use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{one_of, space0},
    error::VerboseError,
    multi::{many1, separated_nonempty_list},
    sequence::{self, delimited, pair, tuple},
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum VersionCmp {
    LessThanEqual,
    LessThan,
    NotEqual,
    Equal,
    GreaterThanEqual,
    GreaterThan,
    ApproxEqual,
    StrictEqual,
}

impl From<&str> for VersionCmp {
    fn from(s: &str) -> Self {
        match s {
            "<=" => VersionCmp::LessThanEqual,
            "<" => VersionCmp::LessThan,
            "!=" => VersionCmp::NotEqual,
            "==" => VersionCmp::Equal,
            ">=" => VersionCmp::GreaterThanEqual,
            ">" => VersionCmp::GreaterThan,
            "~=" => VersionCmp::ApproxEqual,
            "===" => VersionCmp::StrictEqual,
            _ => panic!("invalid version comparison operator: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum URIReference {
    URI(URI),
    RelativeReference,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct URI {
    scheme: String,
    hier_part: HierPart,
    query: Option<String>,
    fragment: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HierPart {
    AuthorityPathAbempty,
    PathAbsolute,
    BathRootless,
    PathEmpty,
}

fn version_cmp(input: &str) -> Res<&str, VersionCmp> {
    pair(
        space0,
        alt((
            tag("<="),
            tag("<"),
            tag("!="),
            tag("=="),
            tag(">="),
            tag(">"),
            tag("~="),
            tag("==="),
        )),
    )(input)
    .map(|(next_input, parsed)| (next_input, VersionCmp::from(parsed.1)))
}

fn version(input: &str) -> Res<&str, &str> {
    pair(
        space0,
        take_while1(|char: char| char.is_alphanumeric() || ("-_.*+!").contains(char)),
    )(input)
    .map(|(next_input, parsed)| (next_input, parsed.1))
}

#[derive(Debug, PartialEq)]
struct Version<'a>(VersionCmp, &'a str);

fn version_one(input: &str) -> Res<&str, Version> {
    tuple((version_cmp, version, space0))(input)
        .map(|(next_input, parsed)| (next_input, Version(parsed.0, parsed.1)))
}

fn version_many(input: &str) -> Res<&str, Vec<Version>> {
    separated_nonempty_list(
        pair(space0, nom::character::complete::char(',')),
        version_one,
    )(input)
    .map(|(next_input, parsed)| (next_input, parsed))
}

fn versionspec(input: &str) -> Res<&str, Vec<Version>> {
    alt((
        delimited(
            nom::character::complete::char('('),
            version_many,
            nom::character::complete::char(')'),
        ),
        version_many,
    ))(input)
    .map(|(next_input, parsed)| (next_input, parsed))
}

fn urlspec(input: &str) -> Res<&str, &str> {
    unimplemented!()
}

// fn marker_op(input: &str) -> Res<&str, &str> {
//     alt((version_cmp,))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_cmp() {
        assert_eq!(
            version_cmp("<= 1234"),
            Ok((" 1234", VersionCmp::LessThanEqual))
        );
        assert_eq!(
            version_cmp("    <= 1234"),
            Ok((" 1234", VersionCmp::LessThanEqual))
        );
    }

    #[test]
    fn test_version() {
        assert_eq!(version("1234"), Ok(("", "1234")));
        assert_eq!(
            version("1-2_3.4*5+6!7-asdf"),
            Ok(("", "1-2_3.4*5+6!7-asdf"))
        );
        assert_eq!(
            version("1-2_3.4*5+6 !7-asdf"),
            Ok((" !7-asdf", "1-2_3.4*5+6"))
        );
    }

    #[test]
    fn test_version_one() {
        assert_eq!(
            version_one("<= 1234"),
            Ok(("", Version(VersionCmp::LessThanEqual, "1234")))
        )
    }

    #[test]
    fn test_version_many_test() {
        assert_eq!(
            version_many("<= 1234, ~= 1.2.3b1"),
            Ok((
                "",
                vec![
                    Version(VersionCmp::LessThanEqual, "1234"),
                    Version(VersionCmp::ApproxEqual, "1.2.3b1")
                ]
            ))
        );
        assert_eq!(
            version_many("  >= 2.8.1,     == 2.8.* "),
            Ok((
                "",
                vec![
                    Version(VersionCmp::GreaterThanEqual, "2.8.1"),
                    Version(VersionCmp::Equal, "2.8.*")
                ]
            ))
        );
    }
}
