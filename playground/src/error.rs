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
        // https://rust-lang.github.io/rust-clippy/master/index.html#or_fun_call
        let res = env::var("FALLBACK_ERR").unwrap_or_else(|_| "false".to_string());
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
    fn catch_unwind() {
        use std::panic;
        let result = panic::catch_unwind(|| {
            panic!("Panic!");
        });
        assert!(result.is_err());
    }

    #[test]
    fn boxed_err() {
        use chrono::NaiveDate;
        use std::error;

        // dyn => dynamic and refers to the fact that trait objects perform dynamic dispatch.
        // This means that the decision of exactly which function is called will occur at program run time.
        // Contrast this to static dispatch which uses the impl Trait syntax.
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

    #[test]
    fn custom_error_thiserror_anyhow() {
        use anyhow::Error;
        use anyhow::Result;
        use std::env;
        use thiserror::Error;

        #[derive(Error, Debug)]
        pub enum CustomError {
            #[error("Var error {0}")]
            VarError(String),

            // forward the source and Display methods straight through to an underlying error without adding an additional message
            #[error(transparent)]
            ParseError(#[from] Error),
        }

        fn print_error(r: Result<String>) {
            match r {
                Ok(d) => println!("Get data: {}", d),
                Err(e) => println!("Get err: {}", e),
            }
        }

        fn gen_var_error() -> Result<String> {
            Err(Error::from(CustomError::VarError("1".to_string())))
        }

        fn gen_parse_error() -> Result<String> {
            let string = env::var("NOT_EXIST")?;
            Ok(string)
        }

        print_error(Ok("1".to_string()));
        print_error(gen_var_error());
        print_error(gen_parse_error());
    }
}
