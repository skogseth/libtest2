#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

use std::iter::FromIterator;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

#[allow(missing_docs)]
#[proc_macro_attribute]
pub fn test(attribute: TokenStream, item: TokenStream) -> TokenStream {
    generate("test", "libtest2", attribute, item)
}

#[allow(missing_docs)]
#[proc_macro_attribute]
pub fn main(attribute: TokenStream, item: TokenStream) -> TokenStream {
    generate("main", "libtest2", attribute, item)
}

/// Generates the equivalent of this Rust code as a `TokenStream`:
///
/// ```nocompile
/// ::libtest2::__support::test_parse!(#[ctor] fn foo() { ... });
/// ```
fn generate(
    macro_type: &str,
    macro_crate: &str,
    attribute: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let mut inner = TokenStream::new();

    // Search for crate_path in attributes
    let mut crate_path = None;
    let mut tokens = attribute.clone().into_iter().peekable();

    while let Some(token) = tokens.next() {
        if let TokenTree::Ident(ident) = &token {
            if ident.to_string() == "crate_path" {
                // Look for =
                if let Some(TokenTree::Punct(punct)) = tokens.next() {
                    if punct.as_char() == '=' {
                        // Collect tokens until comma or end
                        let mut path = TokenStream::new();
                        while let Some(token) = tokens.peek() {
                            match token {
                                TokenTree::Punct(p) if p.as_char() == ',' => {
                                    tokens.next();
                                    break;
                                }
                                _ => {
                                    path.extend(std::iter::once(tokens.next().unwrap()));
                                }
                            }
                        }
                        crate_path = Some(path);
                        break;
                    }
                }
            }
        }
    }

    if attribute.is_empty() {
        // #[test]
        inner.extend([
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
                TokenStream::from_iter([TokenTree::Ident(Ident::new(
                    macro_type,
                    Span::call_site(),
                ))]),
            )),
        ]);
    } else {
        inner.extend([
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
                TokenStream::from_iter([
                    TokenTree::Ident(Ident::new(macro_type, Span::call_site())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, attribute)),
                ]),
            )),
        ]);
    }

    inner.extend(item);

    let mut invoke = crate_path.unwrap_or_else(|| {
        TokenStream::from_iter([
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            TokenTree::Ident(Ident::new(macro_crate, Span::call_site())),
        ])
    });

    invoke.extend([
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("_private", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(
            &format!("{macro_type}_parse"),
            Span::call_site(),
        )),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);

    invoke
}
