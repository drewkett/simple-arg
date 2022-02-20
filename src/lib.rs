pub use simple_args_derive::SimpleArgs;

pub trait Parser
where
    Self: Sized,
{
    fn from_iter(args: impl Iterator<Item = String>) -> Self;

    fn parse() -> Self {
        Parser::from_iter(std::env::args())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_iter() {
        #[derive(Debug, SimpleArgs)]
        struct Foo {
            _abc: bool,
        }

        let args = ["exe", "true"].into_iter().map(|s| s.to_string());
        let foo = Foo::from_iter(args.into_iter());
        dbg!(foo);
    }
}
