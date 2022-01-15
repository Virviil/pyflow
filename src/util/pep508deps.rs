// Quick link: https://www.python.org/dev/peps/pep-0508/
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while, take_while1},
    character::{
        complete::{alphanumeric1, one_of, satisfy, space0},
        is_alphanumeric,
    },
    error::VerboseError,
    multi::{many0, many1, many_m_n, separated_list1},
    sequence::{self, delimited, pair, tuple},
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

// Root level structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Specification {
    UrlReq(UrlReq),
    NameReq(NameReq),
}


// Entire specification
fn specification(input: &str) -> Res<&str, Specification> {
    delimited(
        space0,
        alt((
            nom::combinator::map(url_req, |url_req: UrlReq| Specification::UrlReq(url_req)),
            nom::combinator::map(name_req, |name_req: NameReq| {
                Specification::NameReq(name_req)
            }),
        )),
        space0,
    )(input)
}


// URL Req part - one of 2 global parts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlReq {
    pub unimplemented: String,
}

fn url_req(i: &str) -> Res<&str, UrlReq> {
    tuple((
        name, // name
        space0,
        opt(extras), // extras
        space0,
        urlspec, // urlspecq
        
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameReq {
    pub unimplemented: String,
}

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

#[derive(Debug, PartialEq, Eq, Clone)]
enum URIReference {
    URI(URI),
    RelativeReference,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
    separated_list1(
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

fn identifier(input: &str) -> Res<&str, String> {
    pair(
        satisfy(|c| c.is_alphanumeric()),
        many0(alt((
            nom::combinator::map(satisfy(|c| c.is_alphanumeric()), |c: char| c.to_string()),
            nom::combinator::map(
                pair(
                    take_while(|c| "-_.".contains(c)),
                    satisfy(|c| c.is_alphanumeric()),
                ),
                |(a, b)| format!("{}{}", a, b),
            ),
        ))),
    )(input)
    .map(|(next_input, parsed)| (next_input, format!("{}{}", parsed.0, parsed.1.join(""))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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

    #[rstest(input, expected,
        case("pyflow", Ok(("", "pyflow".to_string()))),
        case("py-flow", Ok(("", "py-flow".to_string()))),
        case("py_flow", Ok(("", "py_flow".to_string()))),
        case("py.flow", Ok(("", "py.flow".to_string()))),
        case("py.flow2", Ok(("", "py.flow2".to_string()))),
        case("py.flow2???", Ok(("???", "py.flow2".to_string()))),
        case("py.flow2.", Ok((".", "py.flow2".to_string()))),
        case("py.flow2-asdf_asdf-", Ok(("-", "py.flow2-asdf_asdf".to_string()))),
        case("py.flow2-asdf_asdf_", Ok(("_", "py.flow2-asdf_asdf".to_string()))),
    )]
    fn test_parse_identifier(input: &str, expected: Res<&str, String>) {
        assert_eq!(identifier(input), expected);
    }
}
