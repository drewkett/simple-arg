pub(crate) mod token_reader;
pub(crate) mod token_tree_ext;
pub(crate) mod token_writer;

use proc_macro::{Delimiter, Ident, TokenStream};
use token_writer::TokenWriter;

use token_reader::TokenReader;

struct Arg {
    name: Ident,
    typ: Ident,
}

impl Arg {
    fn new(name: Ident, typ: Ident) -> Self {
        Self { name, typ }
    }
}

struct SimpleArgs {
    positionals: Vec<Arg>,
    optionals: Vec<Arg>,
}

impl SimpleArgs {
    fn new() -> Self {
        Self {
            positionals: Vec::new(),
            optionals: Vec::new(),
        }
    }
}

enum ArgType {
    Positional,
    Optional,
}

#[proc_macro_derive(SimpleArgs, attributes(arg))]
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
    let mut args = SimpleArgs::new();
    while !struct_rdr.eof() {
        let mut arg_type = ArgType::Positional;
        if let Some(_) = struct_rdr.take_given_punct('#') {
            let attrs = struct_rdr
                .take_delimited_group(Delimiter::Bracket)
                .expect("attribute brackets");
            let mut attrs_rdr = TokenReader::new(attrs.stream());
            while !attrs_rdr.eof() {
                let attr = attrs_rdr.take_ident().expect("expected ident");
                if let Some(parens) = attrs_rdr.take_delimited_group(Delimiter::Parenthesis) {
                    let mut parens_rdr = TokenReader::new(parens.stream());
                    let attr_arg = parens_rdr.take_ident().expect("expected attribute ident");
                    parens_rdr.take_given_punct(',');
                    if &attr.to_string() == "arg" {
                        match attr_arg.to_string().as_str() {
                            "optional" => {
                                arg_type = ArgType::Optional;
                            }
                            arg => panic!("unexpected attribute arg '{}'", arg),
                        }
                    } else {
                        panic!("unknown attribute '{}'", attr)
                    }
                } else {
                    panic!("unexpected attribute '{}'", attr)
                }
            }
        }
        let field_name = struct_rdr.take_ident().expect("expected field name");
        struct_rdr.take_given_punct(':').expect("expected colon");
        let field_type = struct_rdr.take_ident().expect("expected field type");
        let _ = struct_rdr.take_given_punct(',');
        let arg = Arg::new(field_name, field_type);
        match arg_type {
            ArgType::Positional => args.positionals.push(arg),
            ArgType::Optional => {
                if arg.typ.to_string().as_str() != "bool" {
                    panic!("optional requires a bool")
                }
                args.optionals.push(arg)
            }
        }
    }

    let mut output = TokenWriter::new(span);
    output.str("impl Parser for").unwrap();
    output.ident(struct_name.clone());
    output.group(Delimiter::Brace, |impl_wrtr| {
        impl_wrtr
            .str("fn from_iter(mut args: impl Iterator<Item=String>) -> Self")
            .unwrap();
        impl_wrtr.braces(|fn_wrtr| {
            let mut match_arms = TokenWriter::new(span);
            for Arg { name,..} in &args.optionals {
                fn_wrtr.str("let mut").unwrap();
                fn_wrtr.ident(name.clone());
                fn_wrtr.str("= false;").unwrap();
                match_arms.ident_str("Some");
                match_arms.parentheses(|p| p.string_literal(&format!("--{}",name)));
                match_arms.puncts("=>");
                match_arms.braces(|p| {
                    p.ident(name.clone());
                    p.str("= true;").unwrap();
                });
            }
            for Arg { name, typ} in &args.positionals {
                fn_wrtr.str("let").unwrap();
                fn_wrtr.ident(name.clone());
                fn_wrtr.str("= loop").unwrap();
                fn_wrtr.braces(|loop_wrtr| {
                    loop_wrtr.str("match args.next().as_ref().map(|s| s.as_str())").unwrap();
                    loop_wrtr.braces(|match_wrtr| {
                        match_wrtr.extend(match_arms.clone());
                        match_wrtr.str(&format!(r#"Some(arg) => break ::std::str::FromStr::from_str(&arg).expect("expected type {} for arg '{}'"),"#,typ,name)).unwrap();
                        match_wrtr
                            .str(&format!(
                                r#"None => panic!("expected argument for '{}'")"#,
                                name
                            ))
                            .unwrap();
                    });
                    loop_wrtr.punct(';');
                });
                fn_wrtr.str(";").unwrap();
            }
                fn_wrtr.str("while let Some(arg) = args.next()").unwrap();
                fn_wrtr.braces(|loop_wrtr| {
                    loop_wrtr.str("match Some(arg.as_str())").unwrap();
                    loop_wrtr.braces(|match_wrtr| {
                        match_wrtr.extend(match_arms.clone());
                        match_wrtr
                            .str(
                                r#"_ => panic!("unexpected trailing argument '{}'",arg)"#,
                            )
                            .unwrap();
                    });
                    loop_wrtr.punct(';');
                });
                fn_wrtr.str(";").unwrap();
            fn_wrtr.str(r#"if let Some(arg) = args.next() { panic!("unexpected trailing argument '{}'",arg) }"#).unwrap();
            fn_wrtr.ident(struct_name.clone());
            fn_wrtr.braces(|struct_wrtr| {
                for Arg { name, .. } in &args.positionals {
                    struct_wrtr.ident(name.clone());
                    struct_wrtr.punct(',');
                }
                for Arg { name, .. } in &args.optionals {
                    struct_wrtr.ident(name.clone());
                    struct_wrtr.punct(',');
                }
            });
        });
    });
    output.into_inner()
}
