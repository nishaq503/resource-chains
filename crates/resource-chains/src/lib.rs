//! Studying resource creation and consumption pathways using graphs.

pub mod prelude;
pub mod process;
pub mod resource;
pub mod units;

extern crate resource_chains_derive;

pub use resource_chains_derive::Reflective;

// Re-exports of some crates because the `Reflective` trait and its derive macro require it.
pub use anyhow;
pub use lazy_regex;

/// A `Reflective` type is one that can be named as, and parsed from, a string.
///
/// This is useful for defining resources and processes in a way that can be easily serialized and deserialized, and can be used in cross-language contexts.
///
/// We provide a default implementation of `Reflective` for the unit type `()`, and for many primitive types. We also provide a macro to derive `Reflective`
/// for custom types.
///
/// # Example
///
/// You can implement `Reflective` for your own types as follows.
///
/// ```rust
/// use resource_chains::Reflective;
///
/// struct Foo;
///
/// impl Reflective for Foo {
///     type ParseError = String;
///
///     fn type_name() -> &'static str {
///        "foo"
///     }
///
///     fn regex_pattern<'a>() -> &'a lazy_regex::Regex {
///         lazy_regex::regex!(r"^(?i)foo$") // Case-insensitive match for "foo"
///     }
///
///     fn parse(s: &str) -> Result<Self, Self::ParseError> {
///         match s {
///             "foo" | "Foo" => Ok(Self),
///             _ => Err(format!("Invalid input: {s}. Expected 'foo' or 'Foo'.")),
///         }
///     }
/// }
///
/// assert_eq!(Foo::type_name(), "foo");
/// assert!(Foo::parse("foo").is_ok());
/// assert!(Foo::parse("Foo").is_ok());
/// assert!(Foo::parse("bar").is_err());
/// ```
///
/// You can also use the `#[derive(Reflective)]` macro to automatically
/// implement `Reflective` for your types.
///
/// ```rust
/// use resource_chains::Reflective;
///
/// #[derive(Reflective)]
/// #[extra_names(extra_names = ["fb", "FB"])]
/// struct FooBar;
///
/// // The `type_name` is derived from the struct name, and is converted to
/// // kebab-case by default.
/// assert_eq!(FooBar::type_name(), "foo-bar");
/// // The `parse` method accepts the `type_name` in kebab-case, as well as the
/// // original struct name.
/// assert!(FooBar::parse("foo-bar").is_ok());
/// assert!(FooBar::parse("FooBar").is_ok());
/// // The `extra_names` attribute allows you to specify additional names that
/// // can be parsed into the type.
/// assert!(FooBar::parse("fb").is_ok());
/// assert!(FooBar::parse("FB").is_ok());
/// // Any other input will result in an error.
/// assert!(FooBar::parse("foobar").is_err());
/// ```
pub trait Reflective: Sized {
    /// The type of error that can occur when parsing an instance of the type from a string.
    type ParseError;

    /// The name of the type.
    fn type_name() -> &'static str;

    /// A regex pattern that matches valid string representations of instances of the type.
    fn regex_pattern<'a>() -> &'a lazy_regex::Regex;

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

    #[expect(clippy::trivial_regex)]
    fn regex_pattern<'a>() -> &'a lazy_regex::Regex {
        lazy_regex::regex!(r"^\(\)$")
    }

    fn parse(s: &str) -> Result<Self, Self::ParseError> {
        match s {
            "()" => Ok(()),
            _ => Err(anyhow::anyhow!("Invalid unit: {s}. Expected '()'.")),
        }
    }
}
