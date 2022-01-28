//! [wrapper-types-in-rust-choosing-your-guarantees](https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees)
//! [rust doc smart-pointers](https://doc.rust-lang.org/nightly/book/ch15-00-smart-pointers.html)

// Smart pointers, on the other hand, are data structures that not only act like a pointer
// but also have additional metadata and capabilities.

#[cfg(test)]
mod wrapper_type_test {
    use std::ops::Deref;
    use std::rc::{Rc, Weak};

    use crate::wrapper_type::wrapper_type_test::BList::{Cons, Nil};
    use crate::wrapper_type::wrapper_type_test::RcList::{RcCons, RcNil};
    use std::cell::RefCell;

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

    // ----Deref trait----
    // ----Drop trait----

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("Dropping MyBox!")
        }
    }

    #[test]
    fn deref_test() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // multiple ownership
    // We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last.

    enum RcList {
        RcCons(i32, Rc<RcList>),
        RcNil,
    }

    #[test]
    fn rc_test() {
        let a = Rc::new(RcCons(5, Rc::new(RcCons(2, Rc::new(RcNil)))));
        println!("a reference count {}", Rc::strong_count(&a));
        let _b = RcCons(3, Rc::clone(&a));
        println!("a reference count {}", Rc::strong_count(&a));
        {
            let _c = RcCons(4, Rc::clone(&a));
            println!("a reference count {}", Rc::strong_count(&a));
        }
        println!("a reference count {}", Rc::strong_count(&a));
    }

    // interior mutability pattern
    // Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data;
    // normally, this action is disallowed by the borrowing rules.
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }
    #[test]
    fn refcell_test() {
        let leaf = Rc::new(Node {
            value: 1,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });

        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
}
