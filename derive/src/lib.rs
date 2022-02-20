mod token_reader;

use proc_macro::{Delimiter, Group, Punct, Spacing, TokenStream, TokenTree};
use std::str::FromStr;

use token_reader::TokenReader;

#[proc_macro_derive(SimpleArgs)]
pub fn simple_args(input: TokenStream) -> TokenStream {
    let mut rdr = TokenReader::new(input);
    rdr.take_named_ident("struct").expect("expected struct");
    let struct_name = rdr.take_ident().expect("expected struct name");
    let braces_group = rdr
        .take_delimited_group(Delimiter::Brace)
        .expect("expected braces");
    if !rdr.eof() {
        panic!("unexpected value after struct definition")
    }
    let mut struct_rdr = TokenReader::new(braces_group.stream());
    let mut arguments = vec![];
    loop {
        if struct_rdr.eof() {
            break;
        }
        let field_name = struct_rdr.take_ident().expect("expected field name");
        struct_rdr.take_given_punct(':').expect("expected colon");
        let field_type = struct_rdr.take_ident().expect("expected field type");
        let _ = struct_rdr.take_given_punct(',');
        arguments.push((field_name, field_type));
    }

    let mut output_struct = TokenStream::new();
    output_struct.extend(Some(TokenTree::Ident(struct_name.clone())));

    let mut output_struct_internal = TokenStream::new();
    for (name, _) in arguments {
        output_struct_internal.extend([
            TokenTree::Ident(name),
            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
        ]);
    }

    output_struct.extend(Some(TokenTree::Group(Group::new(
        Delimiter::Brace,
        output_struct_internal,
    ))));

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
