#[test]
fn basic() {
    // a struct type (also an enum type) represent a concept, and organize similar data under a single umbrella.
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let root = User {
        username: String::from("guest"),
        email: String::from("root@universe.com"),
        sign_in_count: 5,
        active: false,
    };

    let jack = User {
        username: String::from("jack"),
        email: String::from("jack@secret.org"),
        // cumbersome:
        // active: root.active,
        // sign_in_count: root.sign_in_count,
        //
        // struct update syntax:
        ..root
    };

    println!("{:?}", jack);
}

#[test]
fn tuple_structs() {
    struct Color(i32, i32, i32); // (Red, Green, Blue)
    struct Point(i32, i32, i32); // (X,   Y,     Z)

    let mut _black = Color(0, 0, 0);
    let mut _origin = Point(0, 0, 0);

    // Each struct you define is its own type,
    // even though the fields within the struct
    // have the same types.
    //
    // _black = _origin     // MISMATCHED TYPES:
    // _black is a Color
    // _origin is a Point

    // You can access tuple struct fields using their indexes.
    println!("R: {}, G: {}, B: {}", _black.0, _black.1, _black.2);
}

// https://doc.rust-lang.org/book/ch05-03-method-syntax.html
#[test]
fn method() {
    // Methods are used for organization and readability purposes.
    // You can declare methods with Implementations blocks like the one below.
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // first parameter is always self, which represents the instance of the struct the method is being called on
        // Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.
        // If we wanted to change the instance, we can use `&mut self`
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    let r2 = Rectangle::square(10);
    println!(
        "{:?} {:?} {} {} {}",
        r1,
        r2,
        r1.can_hold(&r2),
        r1.area(),
        r2.area()
    );
}
