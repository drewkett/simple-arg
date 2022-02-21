use crate::token_tree_ext::TokenTreeExt;
use proc_macro::{token_stream, Delimiter, Group, Ident, Punct, Span, TokenStream, TokenTree};

pub(crate) struct TokenReader {
    reader: std::iter::Peekable<token_stream::IntoIter>,
}

impl TokenReader {
    pub(crate) fn new(stream: TokenStream) -> Self {
        Self {
            reader: stream.into_iter().peekable(),
        }
    }

    pub(crate) fn eof(&mut self) -> bool {
        self.reader.peek().is_none()
    }

    pub(crate) fn next_span(&mut self) -> Option<Span> {
        self.reader.peek().map(|tt| tt.span())
    }

    pub(crate) fn take_named_ident(&mut self, ident: &str) -> Option<Ident> {
        match self.reader.peek() {
            Some(TokenTree::Ident(next_ident)) if next_ident.to_string() == ident => {
                self.reader.next().and_then(TokenTreeExt::try_into_ident)
            }
            _ => None,
        }
    }

    pub(crate) fn take_given_punct(&mut self, punct: char) -> Option<Punct> {
        match self.reader.peek() {
            Some(TokenTree::Punct(p)) if p.as_char() == punct => {
                self.reader.next().and_then(TokenTreeExt::try_into_punct)
            }
            _ => None,
        }
    }

    pub(crate) fn take_ident(&mut self) -> Option<Ident> {
        match self.reader.peek() {
            Some(TokenTree::Ident(_)) => self.reader.next().and_then(TokenTree::try_into_ident),
            _ => None,
        }
    }

    pub(crate) fn take_delimited_group(&mut self, delimiter: Delimiter) -> Option<Group> {
        match self.reader.peek() {
            Some(TokenTree::Group(g)) if g.delimiter() == delimiter => {
                self.reader.next().and_then(TokenTreeExt::try_into_group)
            }
            _ => None,
        }
    }
}

impl Iterator for TokenReader {
    type Item = TokenTree;
    fn next(&mut self) -> Option<TokenTree> {
        self.reader.next()
    }
}
