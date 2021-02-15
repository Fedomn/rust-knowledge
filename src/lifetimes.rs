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

#[test]
fn test1() {
    println!("{}", longest("1", "123", "ann"))
}

// One special lifetime we need to discuss is 'static, which means that this reference can live for the entire duration of the program
// But before specifying 'static as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not.
#[test]
fn static_test() {
    let s1: &'static str = "I have a static lifetime.";
    println!("{}", s1)
}
