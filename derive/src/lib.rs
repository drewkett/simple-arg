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
    output.ident_str("impl");
    output.ident_str("Parser");
    output.ident_str("for");
    output.ident(struct_name.clone());
    output.group(Delimiter::Brace, |impl_wrtr| {
        impl_wrtr.ident_str("fn");
        impl_wrtr.ident_str("from_iter");
        impl_wrtr.group(Delimiter::Parenthesis, |arg_wrtr| {
            arg_wrtr.ident_str("mut");
            arg_wrtr.ident_str("args");
            arg_wrtr.punct(':');
            arg_wrtr.ident_str("impl");
            arg_wrtr.ident_str("Iterator");
            arg_wrtr.punct('<');
            arg_wrtr.ident_str("Item");
            arg_wrtr.punct('=');
            arg_wrtr.ident_str("String");
            arg_wrtr.punct('>');
        });
        impl_wrtr.puncts("->");
        impl_wrtr.ident_str("Self");
        impl_wrtr.braces(|fn_wrtr| {
            fn_wrtr.ident_str("loop");
            fn_wrtr.braces(|loop_wrtr| {
                loop_wrtr.ident_str("match");
                loop_wrtr.ident_str("args");
                loop_wrtr.puncts(".");
                loop_wrtr.ident_str("next");
                loop_wrtr.parentheses(|_| ());
                loop_wrtr.braces(|match_wrtr| {
                    match_wrtr.ident_str("Some");
                    match_wrtr.parentheses(|p_wrtr| p_wrtr.ident_str("_"));
                    match_wrtr.puncts("=>");
                    match_wrtr.ident_str("unimplemented");
                    match_wrtr.punct('!');
                    match_wrtr.parentheses(|_| ());
                    match_wrtr.punct(',');
                    match_wrtr.ident_str("None");
                    match_wrtr.puncts("=>");
                    match_wrtr.ident_str("return");
                    match_wrtr.ident(struct_name.clone());
                    match_wrtr.braces(|struct_wrtr| {
                        for (name, _) in arguments {
                            struct_wrtr.ident(name);
                            struct_wrtr.punct(',');
                        }
                    });
                })
            });
        });
    });
    output.into_inner()
}
