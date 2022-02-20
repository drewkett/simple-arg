use proc_macro::{token_stream, Delimiter, Group, Ident, Punct, TokenStream, TokenTree, Span, Spacing};
use std::{iter::Peekable, str::FromStr};

type PeekableStream = Peekable<token_stream::IntoIter>;

fn extract_ident(tree: TokenTree) -> Ident {
    match tree {
        TokenTree::Ident(i) => i,
        _ => panic!("expected ident"),
    }
}

fn extract_group(tree: TokenTree) -> Group {
    match tree {
        TokenTree::Group(g) => g,
        _ => panic!("expected group"),
    }
}

fn extract_punct(tree: TokenTree) -> Punct {
    match tree {
        TokenTree::Punct(p) => p,
        _ => panic!("expected punct"),
    }
}

fn take_named_ident(input: &mut PeekableStream, ident: &str) -> Option<Ident> {
    match input.peek() {
        Some(TokenTree::Ident(next_ident)) if next_ident.to_string() == ident => {
            input.next().map(extract_ident)
        }
        _ => None,
    }
}

fn take_given_punct(input: &mut PeekableStream, punct: char) -> Option<Punct> {
    match input.peek() {
        Some(TokenTree::Punct(p)) if p.as_char() == punct => input.next().map(extract_punct),
        _ => None,
    }
}

fn take_ident(input: &mut PeekableStream) -> Option<Ident> {
    match input.peek() {
        Some(TokenTree::Ident(_)) => input.next().map(extract_ident),
        _ => None,
    }
}

fn take_delimited_group(input: &mut PeekableStream, delimiter: Delimiter) -> Option<Group> {
    match input.peek() {
        Some(TokenTree::Group(g)) if g.delimiter() == delimiter => input.next().map(extract_group),
        _ => None,
    }
}

fn start_span(input: &mut PeekableStream) -> Option<Span> {
    input.peek().map(|tree| tree.span())
}

//fn ident(ident: &str, span: Span) -> impl IntoIterator<Item = TokenTree> {
//    Some(TokenTree::Ident(Ident::new(ident, span)))
//}

#[proc_macro_derive(SimpleArgs)]
pub fn simple_args(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter().peekable();
    let _span = start_span(&mut input).expect("expected struct");
    take_named_ident(&mut input, "struct").expect("expected struct");
    let struct_name = take_ident(&mut input).expect("expected struct name");
    let braces_group = take_delimited_group(&mut input, Delimiter::Brace).expect("expected braces");
    let mut struct_def = braces_group.stream().into_iter().peekable();
    let mut arguments = vec![];
    loop {
        if struct_def.peek().is_none() {
            break;
        }
        let field_name = take_ident(&mut struct_def).expect("expected field name");
        take_given_punct(&mut struct_def, ':').expect("expected colon");
        let field_type = take_ident(&mut struct_def).expect("expected field type");
        let _ = take_given_punct(&mut struct_def, ',');
        arguments.push((field_name, field_type));
    }

    let mut output_struct = TokenStream::new();
    output_struct.extend(Some(TokenTree::Ident(struct_name.clone())));

    let mut output_struct_internal = TokenStream::new();
    for (name, _) in arguments {
        output_struct_internal.extend([
          TokenTree::Ident(name),
          TokenTree::Punct(Punct::new(',', Spacing::Alone))
        ]);
    }

    output_struct.extend(Some(TokenTree::Group(Group::new(Delimiter::Brace, output_struct_internal))));

  TokenStream::from_str(&format!(
    "impl Parser for {} {{
        fn from_iter(mut args: impl Iterator<Item = String>) -> {} {{
            loop {{
                match args.next() {{
                    Some(arg) => {{
                        unimplemented!()
                    }}
                    None => {{
                        return {}
                    }}
                }}
            }}
        }}
    }}",
    struct_name, struct_name, output_struct
))
.expect("internal error")
}
