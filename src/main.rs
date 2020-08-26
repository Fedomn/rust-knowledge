use std::io;

fn main() {
    println!("Hello, world!");
    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("You guess : {}", guess);
        }
        Err(err) => println!("Err : {}", err),
    }
}
