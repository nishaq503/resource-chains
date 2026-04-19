//! Helpers for deriving the `Reflective` trait for structs, enums, and unions.

use proc_macro2::TokenStream as TokenStream2;

/// Any extra attributes for the type on which we will derive the `Reflective` trait.
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(reflective))]
struct ReflectiveAttributes {
    /// `extra_names` is a list of additional string representations for the type, in addition to the default type name.
    #[deluxe(default = Vec::new())]
    extra_names: Vec<String>,
}

/// A helper for deriving the `Reflective` trait for a type.
pub fn derive(item: TokenStream2) -> deluxe::Result<TokenStream2> {
    let mut ast = syn::parse2(item)?;
    let (extra_names, error_message) = extract_extra_names(&mut ast)?;

    let token_stream = match &ast.data {
        syn::Data::Struct(s_data) => match &s_data.fields {
            syn::Fields::Named(fields_named) => {
                let fields = fields_named
                    .named
                    .iter()
                    .filter_map(|f| f.ident.as_ref())
                    .collect::<Vec<_>>();
                derive_struct(&ast, &extra_names, &error_message, &fields)
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                if fields_unnamed.unnamed.is_empty() {
                    derive_unit_struct(&ast, &extra_names, &error_message)
                } else {
                    let field_types = fields_unnamed
                        .unnamed
                        .iter()
                        .map(|f| &f.ty)
                        .collect::<Vec<_>>();
                    derive_tuple_struct(&ast, &extra_names, &error_message, &field_types)
                }
            }
            syn::Fields::Unit => derive_unit_struct(&ast, &extra_names, &error_message),
        },
        syn::Data::Enum(_) => unimplemented!("`Reflective` cannot yet be derived for Enums"),
        syn::Data::Union(_) => unimplemented!("`Reflective` cannot yet be derived for Unions"),
    };

    Ok(token_stream)
}

/// Extracts the extra names from the attributes of the type. These are additional string representations for the type, in addition to the default type name.
fn extract_extra_names(ast: &mut syn::DeriveInput) -> deluxe::Result<(Vec<String>, String)> {
    let ReflectiveAttributes { mut extra_names } = deluxe::extract_attributes(ast)?;

    let ident = &ast.ident;

    let error_message = format!(
        "Invalid value: {{s}}. Expected '{ident}', or one of: {}.",
        extra_names.join(", ")
    );

    extra_names.insert(0, ident.to_string());
    Ok((extra_names, error_message))
}

/// Generates a regex pattern for the type.
fn generate_regex_pattern(names: &[String], fields: Option<&[&syn::Ident]>) -> String {
    let names = format!("({})", names.join("|"));
    fields.map_or_else(
        // If there are no fields, the pattern looks like `^(foo|Foo|f|F)$`.
        || format!("^{names}$"),
        // If there are fields, the pattern looks like `^(foo|Foo|f|F)::(?P<field1>field1=.*):(?P<field2>field2=.*)$`.
        |fields| {
            let field_patterns = fields
                .iter()
                .map(|field| format!(r"(?P<{field}>{field}=.*)"))
                .collect::<Vec<_>>()
                .join(":");
            format!("^{names}::{field_patterns}$")
        },
    )
}

/// Derives the `Reflective` trait for a unit struct.
fn derive_unit_struct(
    ast: &syn::DeriveInput,
    names: &[String],
    error_message: &str,
) -> TokenStream2 {
    let ident = &ast.ident;
    let ident_str = ident.to_string();
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let regex_pattern = generate_regex_pattern(names, None);

    quote::quote! {
        impl #impl_generics Reflective for #ident #type_generics #where_clause {
            type ParseError = ::resource_chains::anyhow::Error;

            fn type_name() -> &'static str {
                #ident_str
            }

            fn regex<'a>() -> &'a ::resource_chains::lazy_regex::Regex {
                ::resource_chains::lazy_regex::regex!(#regex_pattern)
            }

            fn to_string(&self) -> String {
                #ident.to_string()
            }

            fn parse(s: &str) -> Result<Self, Self::ParseError> {
                Self::regex().captures(s).map_or_else(
                    || Err(::resource_chains::anyhow::anyhow!(#error_message, s = s)),
                    |_| Ok(Self),
                )
            }
        }
    }
}

/// Derives the `Reflective` trait for a struct with unnamed fields (tuple struct).
fn derive_tuple_struct(
    ast: &syn::DeriveInput,
    names: &[String],
    error_message: &str,
    field_types: &[&syn::Type],
) -> TokenStream2 {
    let ident = &ast.ident;
    let ident_str = ident.to_string();
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let _names_pattern = format!("({})", names.join("|"));
    let field_indices = (0..field_types.len())
        .map(syn::Index::from)
        .collect::<Vec<_>>();

    quote::quote! {
        impl #impl_generics Reflective for #ident #type_generics #where_clause {
            type ParseError = ::resource_chains::anyhow::Error;

            fn type_name() -> &'static str {
                #ident_str
            }

            fn regex<'a>() -> &'a ::resource_chains::lazy_regex::Regex {
                unimplemented!("Regex pattern generation for tuple structs is not yet implemented")
                // let sub_patterns = vec![
                //     #names_pattern,
                //     r":",
                //     #(
                //         r":",
                //         #field_types::regex().as_str(),
                //     )*
                // ];

                // let regex_pattern = concat!(
                //     #names_pattern,
                //     r":",
                // );

                // let regex_pattern = concat!(
                //     #names_pattern,
                //     r":",
                //     #(
                //         r":",
                //         #field_types::regex().as_str(),
                //     )*
                // );
                // ::resource_chains::lazy_regex::regex!(regex_pattern)
            }

            fn to_string(&self) -> String {
                // Call `Reflective::to_string` on each field and join them with `:`.
                let field_strings = vec![
                    #(::resource_chains::Reflective::to_string(&self.#field_indices)),*
                ].join(":");
                format!("{}::{}", #ident_str, field_strings)
            }

            fn parse(s: &str) -> Result<Self, Self::ParseError> {
                Self::regex().captures(s).map_or_else(
                    || Err(::resource_chains::anyhow::anyhow!(#error_message, s = s)),
                    |captures| Ok(Self(
                        #(
                            ::resource_chains::Reflective::parse(
                                captures.get(#field_indices + 2).unwrap().as_str()
                            )?,
                        )*
                    )),
                )
            }
        }
    }
}

/// Derives the `Reflective` trait for a struct with named fields.
fn derive_struct(
    ast: &syn::DeriveInput,
    names: &[String],
    error_message: &str,
    fields: &[&syn::Ident],
) -> TokenStream2 {
    let ident = &ast.ident;
    let ident_str = ident.to_string();
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let regex_pattern = generate_regex_pattern(names, Some(fields));

    quote::quote! {
        impl #impl_generics Reflective for #ident #type_generics #where_clause {
            type ParseError = ::resource_chains::anyhow::Error;

            fn type_name() -> &'static str {
                #ident_str
            }

            fn regex<'a>() -> &'a ::resource_chains::lazy_regex::Regex {
                ::resource_chains::lazy_regex::regex!(#regex_pattern)
            }

            fn to_string(&self) -> String {
                format!(
                    concat!(#ident_str, ":", #(":", stringify!(#fields), "={}",)*),
                    #(::resource_chains::Reflective::to_string(&self.#fields)),*
                )
            }

            fn parse(s: &str) -> Result<Self, Self::ParseError> {
                Self::regex().captures(s).map_or_else(
                    || Err(::resource_chains::anyhow::anyhow!(#error_message, s = s)),
                    |captures| {
                        Ok(Self {
                            #(
                                #fields: ::resource_chains::Reflective::parse(
                                    captures
                                        .name(stringify!(#fields))
                                        .unwrap()
                                        .as_str()
                                        .trim_start_matches(concat!(stringify!(#fields), "="))
                                )?,
                            )*
                        })
                    }
                )
            }
        }
    }
}
