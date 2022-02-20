use proc_macro::{token_stream, TokenStream, TokenTree};
use std::iter::Peekable;

type PeekableStream = Peekable<token_stream::IntoIter>;

fn take_ident(input: &mut PeekableStream, ident: &str) -> Option<TokenTree> {
    match input.peek() {
        Some(TokenTree::Ident(next_ident)) if next_ident.to_string() == ident => input.next(),
        _ => None,
    }
}

#[proc_macro_derive(SimpleArgs)]
pub fn args(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter().peekable();
    take_ident(&mut input, "struct").expect("expected struct");
    TokenStream::new()
}
