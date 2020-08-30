// Rust use Result<T, E> for Error Handling
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

#[cfg(test)]
mod err {
    use std::env;
    use std::env::VarError;

    use chrono::ParseError;

    #[test]
    fn ignore_err() {
        env::set_var("IGNORE_ERR", "true");
        let res = env::var("IGNORE_ERR").unwrap();
        println!("{}", res);
    }

    #[test]
    fn terminate_err() {
        env::set_var("TERMINATE_ERR", "true");
        let res = env::var("TERMINATE_ERR").expect("Can't read env");
        println!("{}", res);
    }

    #[test]
    fn fallback_err() {
        let res = env::var("FALLBACK_ERR").unwrap_or("false".to_string());
        println!("{}", res);
    }

    #[test]
    fn propagate_err() {
        // The ? operator is similar to unwrap but instead of panicking, it propagates the error to the calling function.
        // One thing to keep in mind is that we can use the ? operator only for functions that return a Result or Option type.
        fn get_env() -> Result<String, env::VarError> {
            let var = env::var("PROPAGATE_ERR")?;
            Ok(var)
        }

        match get_env() {
            Ok(var) => println!("Get env: {}", var),
            Err(e) => eprintln!("Get err: {}", e),
        }
    }

    #[test]
    fn boxed_err() {
        use chrono::NaiveDate;
        use std::error;

        fn get_date() -> Result<NaiveDate, Box<dyn error::Error>> {
            env::set_var("BOXED_DATE", "2020-08-30");
            let date = env::var("BOXED_DATE")?;
            let naive_date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d")?;
            Ok(naive_date)
        }

        match get_date() {
            Ok(date) => println!("Get date: {}", date),
            Err(e) => {
                println!("Get err: ");
                if let Some(e) = e.downcast_ref::<env::VarError>() {
                    println!("It's VarError {}", e);
                } else if let Some(e) = e.downcast_ref::<chrono::format::ParseError>() {
                    println!("It's ParseError {}", e);
                };
            }
        }
    }

    #[test]
    fn custom_err() {
        use chrono::NaiveDate;
        use std::error;
        use std::fmt;

        #[derive(Debug)]
        pub enum CustomError {
            VarError,
            ParseError,
        }

        impl error::Error for CustomError {}

        impl fmt::Display for CustomError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    CustomError::VarError => write!(f, "Env var error"),
                    CustomError::ParseError => write!(f, "Parse date error"),
                }
            }
        }

        impl From<env::VarError> for CustomError {
            fn from(_: VarError) -> Self {
                CustomError::VarError
            }
        }

        impl From<chrono::ParseError> for CustomError {
            fn from(_: ParseError) -> Self {
                CustomError::ParseError
            }
        }

        fn get_date() -> Result<NaiveDate, CustomError> {
            env::set_var("BOXED_DATE", "2020-08-30");
            let date = env::var("BOXED_DATE")?;
            let naive_date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d")?;
            Ok(naive_date)
        }

        match get_date() {
            Ok(date) => println!("Get date: {}", date),
            Err(e) => {
                println!("Get err: ");
                match e {
                    CustomError::VarError => println!("It's VarError {}", e),
                    CustomError::ParseError => println!("It's ParseError {}", e),
                }
            }
        }
    }
}
