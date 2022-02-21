use proc_macro::{Group, Ident, Punct, TokenTree};

pub(crate) trait TokenTreeExt {
    fn try_into_ident(tree: Self) -> Option<Ident>;
    fn try_into_group(tree: Self) -> Option<Group>;
    fn try_into_punct(tree: Self) -> Option<Punct>;
}

impl TokenTreeExt for TokenTree {
    fn try_into_ident(tree: Self) -> Option<Ident> {
        match tree {
            TokenTree::Ident(i) => Some(i),
            _ => panic!("expected ident"),
        }
    }

    fn try_into_group(tree: Self) -> Option<Group> {
        match tree {
            TokenTree::Group(g) => Some(g),
            _ => panic!("expected group"),
        }
    }

    fn try_into_punct(tree: Self) -> Option<Punct> {
        match tree {
            TokenTree::Punct(p) => Some(p),
            _ => panic!("expected punct"),
        }
    }
}
