#[cfg(test)]
mod traits {

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
}
