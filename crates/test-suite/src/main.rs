//! Tests for the workspace.

use resource_chains::Reflective;

use test_suite::reflective::Foo;

/// A simple test to ensure that the workspace is set up correctly.
fn main() {
    println!("Running tests...");

    assert_eq!(Foo::type_name(), "Foo");
    assert!(Foo::parse("Foo").is_ok());
    assert!(Foo::parse("foo").is_ok());
    assert!(Foo::parse("Bar").is_err());

    println!("All tests passed!");
}
