pub(crate) mod token_reader;
pub(crate) mod token_tree_ext;
pub(crate) mod token_writer;

use proc_macro::{Delimiter, TokenStream};
use token_writer::TokenWriter;

use token_reader::TokenReader;

#[proc_macro_derive(SimpleArgs)]
pub fn simple_args(input: TokenStream) -> TokenStream {
    let mut rdr = TokenReader::new(input);
    let span = rdr.next_span().expect("expected struct");
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

    let mut output = TokenWriter::new(span);
    output.str("impl Parser for").unwrap();
    output.ident(struct_name.clone());
    output.group(Delimiter::Brace, |impl_wrtr| {
        impl_wrtr
            .str("fn from_iter(mut args: impl Iterator<Item=String>) -> Self")
            .unwrap();
        impl_wrtr.braces(|fn_wrtr| {
            for (name, typ) in &arguments {
                fn_wrtr.str("let").unwrap();
                fn_wrtr.ident(name.clone());
                fn_wrtr.str("= match args.next()").unwrap();
                fn_wrtr.braces(|match_wrtr| {
                    match_wrtr.str(&format!(r#"Some(arg) => ::std::str::FromStr::from_str(&arg).expect("expected type {} for arg '{}'"),"#,typ,name)).unwrap();
                    match_wrtr
                        .str(&format!(
                            r#"None => panic!("expected argument for '{}'")"#,
                            name
                        ))
                        .unwrap();
                });
                fn_wrtr.punct(';');
            }
            fn_wrtr.str(r#"if let Some(arg) = args.next() { panic!("unexpected trailing argument '{}'",arg) }"#).unwrap();
            fn_wrtr.ident(struct_name.clone());
            fn_wrtr.braces(|struct_wrtr| {
                for (name, _) in &arguments {
                    struct_wrtr.ident(name.clone());
                    struct_wrtr.punct(',');
                }
            });
        });
    });
    output.into_inner()
}
