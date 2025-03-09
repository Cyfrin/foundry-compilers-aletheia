use super::*;

// This test is mainly here for interactive development. Use this test while
// you're working on the proc-macro defined in this file.
#[test]
fn test_symbols() {
    // We textually include the symbol.rs file, which contains the list of all
    // symbols, keywords, and common words. Then we search for the
    // `symbols! { ... }` call.

    static SYMBOL_RS_FILE: &str = include_str!("../../../interface/src/symbol.rs");

    let file = syn::parse_file(SYMBOL_RS_FILE).unwrap();
    let symbols_path: syn::Path = syn::parse_quote!(symbols);
    let symbols_ident = symbols_path.get_ident().unwrap();

    let m: &syn::ItemMacro = file
        .items
        .iter()
        .find_map(|i| match i {
            syn::Item::Macro(m) if m.mac.path.get_ident() == Some(symbols_ident) => Some(m),
            _ => None,
        })
        .expect("did not find `symbols!` macro invocation.");

    let body_tokens = m.mac.tokens.clone();

    test_symbols_macro(body_tokens, &[]);
}

fn test_symbols_macro(input: TokenStream, expected_errors: &[&str]) {
    let (output, found_errors) = symbols_with_errors(input);

    // It should always parse.
    let _parsed_file = syn::parse2::<syn::File>(output).unwrap();

    assert_eq!(
        found_errors.len(),
        expected_errors.len(),
        "Macro generated a different number of errors than expected"
    );

    for (found_error, &expected_error) in found_errors.iter().zip(expected_errors) {
        assert_eq!(found_error.to_string(), expected_error);
    }
}

#[test]
fn check_dup_keywords() {
    let input = quote! {
        Keywords {
            Crate: "crate",
            Crate: "crate",
        }
        Symbols {}
    };
    test_symbols_macro(input, &["Symbol `crate` is duplicated", "location of previous definition"]);
}

#[test]
fn check_dup_symbol() {
    let input = quote! {
        Keywords {}
        Symbols {
            splat,
            splat,
        }
    };
    test_symbols_macro(input, &["Symbol `splat` is duplicated", "location of previous definition"]);
}

#[test]
fn check_dup_symbol_and_keyword() {
    let input = quote! {
        Keywords {
            Splat: "splat",
        }
        Symbols {
            splat,
        }
    };
    test_symbols_macro(input, &["Symbol `splat` is duplicated", "location of previous definition"]);
}

#[test]
fn check_symbol_order() {
    let input = quote! {
        Keywords {}
        Symbols {
            zebra,
            aardvark,
        }
    };
    test_symbols_macro(
        input,
        &["Symbol `aardvark` must precede `zebra`", "location of previous symbol `zebra`"],
    );
}
