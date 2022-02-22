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
    use std::panic::catch_unwind;

    use super::*;

    #[test]
    fn test_empty() {
        #[derive(Debug, SimpleArgs)]
        struct Foo {}

        let args = [].into_iter();
        let foo = Foo::from_iter(args);
        dbg!(foo);
    }

    #[test]
    fn test_positional_bool() {
        #[derive(Debug, SimpleArgs)]
        struct Foo {
            bar: bool,
        }

        let args = ["true"].into_iter().map(ToString::to_string);
        let foo = Foo::from_iter(args);
        assert!(foo.bar);
        let args = ["1"].into_iter().map(ToString::to_string);
        assert!(catch_unwind(|| Foo::from_iter(args)).is_err());
    }

    #[test]
    fn test_flag_bool() {
        #[derive(Debug, SimpleArgs)]
        struct Foo {
            #[arg(optional)]
            bar: bool,
        }

        let args = ["--bar"].into_iter().map(ToString::to_string);
        let foo = Foo::from_iter(args);
        assert!(foo.bar);
        let args = [].into_iter();
        let foo = Foo::from_iter(args);
        assert!(!foo.bar);
        let args = ["1"].into_iter().map(ToString::to_string);
        assert!(catch_unwind(|| Foo::from_iter(args)).is_err());
    }
    #[test]
    fn test_multiple_usize() {
        #[derive(Debug, SimpleArgs)]
        struct Foo {
            bar: usize,
            baz: usize,
        }

        let args = ["1", "2"].into_iter().map(ToString::to_string);
        let foo = Foo::from_iter(args);
        assert_eq!(foo.bar, 1);
        assert_eq!(foo.baz, 2);
        let args = ["true"].into_iter().map(ToString::to_string);
        assert!(catch_unwind(|| Foo::from_iter(args)).is_err());
    }

    #[test]
    fn test_unexpected_trailing() {
        #[derive(Debug, SimpleArgs)]
        struct Foo {
            _bar: usize,
        }

        let args = ["1", "1"].into_iter().map(ToString::to_string);
        assert!(catch_unwind(|| Foo::from_iter(args)).is_err());
    }
}
