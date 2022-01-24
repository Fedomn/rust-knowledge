#[cfg(test)]
mod traits {

    use std::str::FromStr;

    #[test]
    fn test_basic() {
        use regex::Regex;
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
}
