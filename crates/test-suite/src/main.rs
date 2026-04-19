//! Tests for the workspace.

#![expect(
    clippy::unwrap_used,
    clippy::similar_names,
    clippy::disallowed_names,
    clippy::too_many_lines
)]

use resource_chains::Reflective;

use test_suite::reflective::{Bar, Baz, Baz2, Foo, Foo2, FooBar, FooBar2};

/// A simple test to ensure that the workspace is set up correctly.
fn main() {
    println!("Running tests...");

    assert_eq!(Foo::type_name(), "Foo");
    assert!(Foo::parse("Foo").is_ok());
    assert!(Foo::parse("foo").is_ok());
    assert!(Foo::parse("Bar").is_err());

    assert_eq!(Foo2::type_name(), "Foo2");
    assert!(Foo2::parse("Foo2").is_ok());
    assert!(Foo2::parse("FOO2").is_ok());
    assert!(Foo2::parse("Bar").is_err());

    let bar = Bar { a: 42 };
    assert_eq!(Bar::type_name(), "Bar");
    assert!(Bar::parse("Bar::a=42").is_ok());
    assert!(Bar::parse("Bar::a=-1").is_ok());
    assert!(Bar::parse("Bar::a=abc").is_err());
    let re_bar = Bar::parse(&bar.to_string());
    assert!(re_bar.is_ok());
    assert_eq!(re_bar.unwrap().a, bar.a);

    let foobar = FooBar { a: 7, b: 0.5 };
    assert_eq!(FooBar::type_name(), "FooBar");
    let ok_values = [
        "FooBar::a=7:b=0.5",
        "FooBar::a=-1:b=1.5",
        "FooBar::a=7:b=-0.5",
        "FooBar::a=-1:b=-0.5",
    ];
    for s in ok_values {
        let fb = FooBar::parse(s);
        assert!(
            fb.is_ok(),
            "Failed to parse valid input for FooBar: {:?}. Error: {:?}",
            s,
            fb.err()
        );
    }

    assert!(FooBar::parse("FooBar::a=7:b=abc").is_err());
    let re_foobar = FooBar::parse(&foobar.to_string());
    assert!(re_foobar.is_ok());
    let re_foobar = re_foobar.unwrap();
    assert_eq!(re_foobar.a, foobar.a);
    assert!(
        (re_foobar.b - foobar.b).abs() < f32::EPSILON,
        "Parsed value for b does not match original. Expected: {}, Got: {}",
        foobar.b,
        re_foobar.b
    );

    let foobar2 = FooBar2 {
        foo: Foo,
        bar: Bar { a: 42 },
    };
    assert_eq!(FooBar2::type_name(), "FooBar2");
    let ok_values = [
        "FooBar2::foo=Foo:bar=Bar::a=42",
        "FooBar2::foo=Foo:bar=Bar::a=-1",
    ];
    for s in ok_values {
        let fb2 = FooBar2::parse(s);
        assert!(
            fb2.is_ok(),
            "Failed to parse valid input for FooBar2: {:?}. Error: {:?}",
            s,
            fb2.err()
        );
    }
    let re_foobar2 = FooBar2::parse(&foobar2.to_string());
    assert!(re_foobar2.is_ok());
    let re_foobar2 = re_foobar2.unwrap();
    assert_eq!(re_foobar2.bar.a, foobar2.bar.a);

    let baz = Baz(42, 0.5);
    assert_eq!(Baz::type_name(), "Baz");
    let ok_values = [
        "Baz::42:0.5",
        "Baz::-1:-0.5",
        "Baz::42:-1.5",
        "Baz::-1:1.5",
        "Baz::0:0",
    ];
    for s in ok_values {
        let b = Baz::parse(s);
        assert!(
            b.is_ok(),
            "Failed to parse valid input for Baz: {:?}. Error: {:?}",
            s,
            b.err()
        );
    }
    let re_baz = Baz::parse(&baz.to_string());
    assert!(re_baz.is_ok());
    let re_baz = re_baz.unwrap();
    assert_eq!(re_baz.0, baz.0);
    assert!(
        (re_baz.1 - baz.1).abs() < f32::EPSILON,
        "Parsed value for second field does not match original. Expected: {}, Got: {}",
        baz.1,
        re_baz.1
    );

    let baz2 = Baz2(42, 0.5);
    assert_eq!(Baz2::type_name(), "Baz2");
    assert_eq!(baz2.to_string(), "Baz2::42:0.5");
    let ok_values = [
        "Baz2::42:0.5",
        "Baz2::-1:-0.5",
        "Baz2::42:-1.5",
        "Baz2::-1:1.5",
        "Baz2::0:0",
    ];
    for s in ok_values {
        let b = Baz2::parse(s);
        assert!(
            b.is_ok(),
            "Failed to parse valid input for Baz2: {:?}. Error: {:?}",
            s,
            b.err()
        );
    }
    let re_baz2 = Baz2::parse(&baz2.to_string());
    assert!(re_baz2.is_ok());
    let re_baz2 = re_baz2.unwrap();
    assert_eq!(re_baz2.0, baz2.0);
    assert!(
        (re_baz2.1 - baz2.1).abs() < f32::EPSILON,
        "Parsed value for second field does not match original. Expected: {}, Got: {}",
        baz2.1,
        re_baz2.1
    );

    println!("All tests passed!");
}
