//! Derive macros for the `resource-cycles` crate.

#![expect(clippy::missing_panics_doc, clippy::unwrap_used)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;

/// Any extra attributes for the struct on which we will derive the `Reflective` trait.
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(extra_names))]
struct ReflectiveStructAttributes {
    /// `extra_names` is a list of additional string representations for the type, in addition to the default type name.
    #[deluxe(default = Vec::new())]
    extra_names: Vec<String>,
}

/// A helper for deriving the `Reflective` trait for a struct.
fn reflective_derive2(item: TokenStream2) -> deluxe::Result<TokenStream2> {
    // parse into ast
    let mut ast: DeriveInput = syn::parse2(item)?;

    // Get the extra names from the attributes.
    let ReflectiveStructAttributes { mut extra_names } = deluxe::extract_attributes(&mut ast)?;

    // Get the default type name (the struct name)
    let ident = &ast.ident;
    let ident_str_hyphenated = hyphenate_struct_name(ident);

    // Add the hyphenated struct name to the list of extra names, so that it can be parsed as well.
    extra_names.insert(0, ident.to_string());
    let extra_names_str = extra_names.join(", ");

    // Split the generics into the parts needed for the impl.
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // generate impl
    Ok(quote::quote! {
        impl #impl_generics Reflective for #ident #type_generics #where_clause {
            type ParseError = anyhow::Error;

            fn type_name() -> &'static str {
                #ident_str_hyphenated
            }

            fn parse(s: &str) -> Result<Self, Self::ParseError> {
                match s {
                    #ident_str_hyphenated #(| #extra_names)* => Ok(Self),
                    _ => Err(anyhow::anyhow!("Invalid value: {s}. Expected '{}', or one of: {}.", #ident_str_hyphenated, #extra_names_str)),
                }
            }
        }
    })
}

/// Derive the `Reflective` trait.
///
/// By default, the type name (with hyphens instead of camel case) will be used as the string representation of the type. For example, `MyStruct` will be
/// represented as `"my-struct"`. The struct can be parsed from this string, as well as the actual struct name, i.e. `"MyStruct"`. You can specify any
/// additional string representations using the `extra_names` attribute, e.g. `#[extra_names(extra_names = ["ms", "Ms"])]`.
///
/// # Example
///
/// ```rust
/// use resource_cycles::Reflective;
///
/// #[derive(Reflective)]
/// struct Foo;
///
/// #[derive(Reflective)]
/// #[extra_names(extra_names = ["b"])]
/// struct Bar;
///
/// #[derive(Reflective)]
/// #[extra_names(extra_names = ["fb", "FB"])]
/// struct FooBar;
/// ```
///
/// In this example, the `Foo` struct can be parsed from the string `"foo"` or `"Foo"`, the `Bar` struct can be parsed from the string `"bar"` or `"b"`, and the
/// `FooBar` struct can be parsed from the string `"foo-bar"`, `"fb"`, or `"FB"`.
#[proc_macro_derive(Reflective, attributes(extra_names))]
pub fn reflective_derive(item: TokenStream) -> TokenStream {
    reflective_derive2(item.into()).unwrap().into()
}

/// Converts the name of a struct to a string literal which has '-'s instead of camel case, e.g. `MyStruct` becomes `"my-struct"`.
fn hyphenate_struct_name(ident: &syn::Ident) -> String {
    let ident_str = ident.to_string();
    let mut result = String::new();
    for (i, c) in ident_str.chars().enumerate() {
        if c.is_uppercase() && i != 0 {
            result.push('-');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}
