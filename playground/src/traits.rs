#[cfg(test)]
mod traits_test {

    #[test]
    fn test_trait_associated_type() {
        use regex::Regex;
        use std::str::FromStr;
        trait Parse {
            type Error;
            fn parse(s: &str) -> Result<Self, Self::Error>
            where
                Self: Sized;
        }

        impl<T> Parse for T
        where
            T: FromStr + Default,
        {
            type Error = String;

            fn parse(s: &str) -> Result<Self, Self::Error> {
                let re = Regex::new(r"^[0-9]+").unwrap();

                if let Some(c) = re.captures(s) {
                    c.get(0).map_or(Err("failed to capture".to_string()), |m| {
                        m.as_str()
                            .parse()
                            .map_err(|_err| "failed to parse captured string".to_string())
                    })
                } else {
                    Err("failed to parse string".to_string())
                }
            }
        }

        assert_eq!(u8::parse("123abcd"), Ok(123));
        assert_eq!(u8::parse("abcd"), Err("failed to parse string".into()));
    }

    #[test]
    fn test_trait_object() {
        trait Formatter {
            fn format(&self, input: &mut String) -> bool;
        }

        struct HtmlFormatter;
        impl Formatter for HtmlFormatter {
            fn format(&self, input: &mut String) -> bool {
                input.push_str("\nformatted by html formatter");
                true
            }
        }

        struct RustFormatter;
        impl Formatter for RustFormatter {
            fn format(&self, input: &mut String) -> bool {
                input.push_str("\nformatted by rust formatter");
                true
            }
        }

        let mut text = "Hello World".to_string();
        let html: &dyn Formatter = &HtmlFormatter;
        let rust: &dyn Formatter = &RustFormatter;
        let formatters = vec![html, rust];

        for formatter in formatters {
            formatter.format(&mut text);
        }

        println!("{}", text);
    }

    /// [trait_alias and type_alias_impl_trait explanation](https://stackoverflow.com/questions/57937436/how-to-alias-an-impl-trait)
    #[test]
    fn trait_alias_test() {
        trait X = Fn(u32) -> u32;
        fn f1() -> impl X {
            |x: u32| x
        }
        fn f2() -> impl X {
            |x: u32| x
        }
        f1();
        f2();
    }

    #[test]
    fn type_alias_impl_trait_test() {
        type X1 = impl Fn(u32) -> u32;
        type X2 = impl Fn(u32) -> u32;
        fn f1() -> X1 {
            |x: u32| x
        }

        // error: concrete type differs from previous defining opaque type use
        // fn f2() -> X1 {
        //     |x: u32| x
        // }
        fn f2() -> X2 {
            |x: u32| x
        }
        let _ = f1();
        let _ = f2();
    }
}
