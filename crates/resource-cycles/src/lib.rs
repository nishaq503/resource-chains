//! Studying resource cycles using cyclic graphs.

pub mod process;
pub mod resource;
pub mod units;

pub use process::{Device, Process};
pub use resource::Resource;
pub use units::Units;

extern crate resource_cycles_derive;

pub use resource_cycles_derive::Reflective;

/// A `Reflective` type is one that can be named as a `String` and parsed from a `String`.
pub trait Reflective: Sized {
    /// The type of error that can occur when parsing an instance of the type from a string.
    type ParseError;

    /// The name of the type.
    fn type_name() -> &'static str;

    /// Parse an instance of the type from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed into an instance of the type.
    fn parse(s: &str) -> Result<Self, Self::ParseError>;
}

impl Reflective for () {
    type ParseError = anyhow::Error;

    fn type_name() -> &'static str {
        "()"
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "()" => Ok(()),
            _ => Err(anyhow::anyhow!("Invalid unit: {s}. Expected '()'.")),
        }
    }
}
