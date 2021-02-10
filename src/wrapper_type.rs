//! https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees
//! https://doc.rust-lang.org/nightly/book/ch15-00-smart-pointers.html

// Smart pointers, on the other hand, are data structures that not only act like a pointer
// but also have additional metadata and capabilities.

#[cfg(test)]
mod wrapper_type {
    use crate::wrapper_type::wrapper_type::BList::{Cons, Nil};

    // Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.
    // Boxes mostly used in these situations:
    // 1. When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // 2. When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // 3. When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

    #[derive(Debug)]
    enum BList {
        // Cons(i32, List), -> recursive type `List` has infinite size
        Cons(i32, Box<BList>),
        // after using box, Cons has exact size: i32 + the size of a box's pointer data
        Nil,
    }

    #[test]
    fn box_recursive_type() {
        let b_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{:?}", b_list);

        // error: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
        // let Cons(a1, b1) = b_list;

        if let Cons(a, b) = b_list {
            println!("{:?} {:?}", a, b)
        }
    }
}
