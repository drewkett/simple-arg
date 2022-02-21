use std::str::FromStr;

use proc_macro::{Delimiter, Group, Ident, LexError, Punct, Spacing, Span, TokenStream, TokenTree};

pub(crate) struct TokenWriter {
    writer: TokenStream,
    span: Span,
}

#[allow(dead_code)]
impl TokenWriter {
    pub(crate) fn new(span: Span) -> TokenWriter {
        TokenWriter {
            writer: TokenStream::new(),
            span,
        }
    }

    pub(crate) fn into_inner(self) -> TokenStream {
        self.writer
    }

    pub(crate) fn ident(&mut self, ident: Ident) {
        self.writer.extend(Some(TokenTree::Ident(ident)));
    }

    pub(crate) fn ident_str(&mut self, ident: &str) {
        self.writer
            .extend(Some(TokenTree::Ident(Ident::new(ident, self.span))));
    }

    #[must_use]
    pub(crate) fn str(&mut self, s: &str) -> Result<(), LexError> {
        self.writer.extend(TokenStream::from_str(s)?);
        Ok(())
    }

    pub(crate) fn punct(&mut self, punct: char) {
        self.writer
            .extend(Some(TokenTree::Punct(Punct::new(punct, Spacing::Alone))));
    }

    pub(crate) fn puncts(&mut self, puncts: &str) {
        let mut it = puncts.chars().peekable();
        while let Some(c) = it.next() {
            let spacing = match it.peek() {
                Some(_) => Spacing::Joint,
                None => Spacing::Alone,
            };
            self.writer
                .extend(Some(TokenTree::Punct(Punct::new(c, spacing))));
        }
    }

    pub(crate) fn group(&mut self, delimiter: Delimiter, closure: impl FnOnce(&mut TokenWriter)) {
        let mut closure_writer = TokenWriter::new(self.span);
        closure(&mut closure_writer);
        let closure_stream = closure_writer.into_inner();
        self.writer.extend(Some(TokenTree::Group(Group::new(
            delimiter,
            closure_stream,
        ))));
    }

    pub(crate) fn braces(&mut self, closure: impl FnOnce(&mut TokenWriter)) {
        self.group(Delimiter::Brace, closure)
    }

    pub(crate) fn parentheses(&mut self, closure: impl FnOnce(&mut TokenWriter)) {
        self.group(Delimiter::Parenthesis, closure)
    }

    pub(crate) fn brackets(&mut self, closure: impl FnOnce(&mut TokenWriter)) {
        self.group(Delimiter::Bracket, closure)
    }
}
