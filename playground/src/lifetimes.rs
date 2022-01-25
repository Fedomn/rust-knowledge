//! Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
//! A lifetime means "how long the variable lives".

//! Lifetime elision sugar:
//! https://doc.rust-lang.org/stable/reference/lifetime-elision.html#lifetime-elision

#[test]
fn test_hello() {
    use std::fmt::Display;
    fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    println!("{}", longest("1", "123", "ann"))
}

// One special lifetime we need to discuss is 'static, which means that this reference can live for the entire duration of the program
// But before specifying 'static as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not.
#[test]
fn static_test() {
    let s1: &'static str = "I have a static lifetime.";
    println!("{}", s1)
}

#[test]
#[warn(unused_variables)]
fn lifetime_constraints() {
    let _x: &Vec<i8>;
    {
        let y = Vec::new();
        _x = &y;
    }
    // error[E0597]: `y` does not live long enough
    // println!("{}", _x.len());
}

#[test]
fn lifetime_annotations() {
    fn print_ret<'a>(s1: &str, s2: &'a str) -> &'a str {
        println!("s1 : {}", s1);
        s2
    }

    let first = "first".to_string();
    let second = "second".to_string();
    let ret = print_ret(&first, &second);
    println!("ret : {}", ret);
}

#[test]
fn lifetime_reference() {
    fn print_ret1(s1: u8, s2: u8) -> u8 {
        println!("s1 : {}", s1);
        s2
    }

    fn print_ret2<'a>(s1: &'a u8, s2: &'a u8) -> &'a u8 {
        println!("s1 : {}", s1);
        s2
    }

    let first = 1;
    let second = 2;
    println!("ret1 : {}", print_ret1(first, second));
    println!("ret2 : {}", print_ret2(&first, &second));
}
