//! Tests for the `Reflective` trait and its derive macro.

use resource_chains::Reflective;

/// `Foo`
pub struct Foo;

impl Reflective for Foo {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "Foo"
    }

    fn regex_pattern<'a>() -> &'a resource_chains::lazy_regex::Regex {
        resource_chains::lazy_regex::regex!(r"^(?i)foo$")
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "foo" | "Foo" => Ok(Self),
            _ => Err(anyhow::anyhow!("Invalid input for Foo: {s}")),
        }
    }
}
