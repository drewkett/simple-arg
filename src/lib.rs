pub use simple_args_derive::SimpleArgs;

pub trait Parser
where
    Self: Sized,
{
    fn from_iter(args: impl Iterator<Item = String>) -> Self;

    fn parse() -> Self {
        Parser::from_iter(std::env::args().skip(1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        #[derive(Debug, SimpleArgs)]
        struct Foo {}

        let args = [].into_iter();
        let foo = Foo::from_iter(args.into_iter());
        dbg!(foo);
    }
}
