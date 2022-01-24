//! Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.
//! Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.

#[cfg(test)]
mod ownership {
    #[test]
    fn scope() {
        // SCOPE owns a VARIABLE owns a VALUE
        // SCOPE surrounded by `{}`
        // A value lives on memory, it can move between variables.
        // A variable can be an owner of a value.
        // A variable comes in and goes out of a scope.

        // RULES:
        // Each value has a single owner.
        // Ownership can move between variables.
        // When the owner goes out of scope, the value will be dropped.
        fn s() {
            let s = "v"; // s comes into scope here
            println!("{}", s);
        } // s goes out of scope here

        s()
    }

    #[test]
    fn string() {
        // A string literal.
        // Its value is known at compile-time and hardcode into the final executable.
        let a = "a";
        println!("{}", a);

        // A String.
        // Its value is only known in runtime can dynamically change in runtime.
        // Rust allocates it on the HEAP memory.
        let mut b = String::from("b");
        b.push_str("!");
        println!("{}", b);

        // b goes out of scope. Rust calls the drop fn and returns the memory used by b back to OS
    }

    #[test]
    fn scope_move() {
        // Variables have two type of semantics:
        //
        // 1. Copy Semantics:
        // Rust copies the variable's value, and uses the new value for the new variable.
        // Used for scalar values like integers.
        // This kind of values usually live on stack.
        //
        // 2. Move Semantics:
        // Variable's value moves to another variable without copying.
        // Used for heap-allocated values like a String.
        //

        //
        //
        // 1. Copy Semantics:
        let x = 5; // x is scalar value. x is allocated on the stack memory.
        let y = x; // Rust copies x's value 5 and binds the new value to the y variable.
        println!("x {} y {}", x, y); // x and y have different memory location.

        //
        //
        // 2. Move Semantics:
        let s1 = String::from("hi"); // s1 is String value. its ptr points to the location on the heap memory.

        // 2.1 compile error:
        //
        // let s2 = s1; // value moved. s2 is new String value. its ptr points to the same location on the heap memory.
        // println!("{} {}", s1, s2); // here will compile error: value borrowed here after move
        // reason:
        // -> Rust moves s1's value to s2.
        // -> s2 is the new OWNER of s1's value.
        // -> s1 is no longer valid.
        //    -> goes out of scope.
        //    -> rust claims its memory.

        // 2.2 can work:
        //
        let s2 = s1.clone();
        println!("{} {}", s1, s2);
        // reason:
        // -> s2 has a deep-copy of s1's value.
        // -> there are one more "hi" on the heap now.
        // -> and its owner is s2.

        //
        //
        // 3. dive into:
        // why Simple values like an integer doesn't need to be cloned.
        // because they can be copied by Rust automatically. It has a Copy trait.
        // see: https://doc.rust-lang.org/std/marker/trait.Copy.html

        //
        //
        // 4. derive copy and clone
        #[derive(Debug, Copy, Clone)]
        struct Foo;
        let x = Foo;
        let _ = x;
        println!("{:?}", x);
    }

    #[test]
    fn scope_func() {
        // Passing a value to a func is similar to assigning it to a variable.
        // It will either MOVE or COPY the value.

        // change s owner to this func, after this func, rust will reclaim its memory
        fn change_owner(s: String) {
            println!("{}", s);
        }

        // func receives a copy of i
        fn copy(i: i8) {
            println!("{}", i);
        }

        // give the owner of s to calling func
        fn give_owner() -> String {
            let s = String::from("s");
            s
        }

        // give back the owner of s to calling func
        fn change_backed_owner(s: String) -> String {
            s
        }

        let s = String::from("start");
        change_owner(s);
        // println!("{}", s); // will error, because the owner of s has changed.

        let i = 1;
        copy(i);
        println!("{}", i); // copy i, nothing move

        let g = give_owner();
        println!("{}", g);

        let h = String::from("hello");
        let new_h = change_backed_owner(h);
        // println!("{}", h); // will error, because h owner is `change_backed_owner` func
        // and the outer func `scope_func` loses the ownership of s.

        println!("{}", new_h);
    }

    #[test]
    fn scope_reference() {
        // reference: It's a value that refers to another value without taking its ownership.
        // represented with a leading ampersand &.

        let s = String::from("hi");
        let len = strlen(&s);
        println!("len({}) = {}", s, len);

        let mut cs = s;
        change(&mut cs);
        println!("{}", cs);

        // s is a reference to a String
        // s goes out of scope but nothing happens. because strlen isn't the owner of s
        // borrows a String and can look at it
        fn strlen(s: &String) -> usize {
            s.len()
        }

        // borrows a String and can change it
        fn change(s: &mut String) {
            s.push_str(" reference !");
        }

        fn adds_hungary(mut country: String) {
            country.push_str("-Hungary");
            println!("{}", country);
        }

        let country = String::from("Austria"); // print country will error: move occurs because `country` has type `String`, which does not implement the `Copy` trait
        adds_hungary(country); // print country will error: value moved here
        println!("space")
        // println!("{}", country); // will error: value borrowed here after move
    }
}

#[cfg(test)]
mod reference_counter {
    #[test]
    fn refcell() {
        use std::cell::RefCell;
        let data = RefCell::new(1);
        {
            // 同一个作用域下，我们不能同时有活跃的可变借用和不可变借用
            let mut v = data.borrow_mut();
            *v += 1;
        }
        println!("data: {:?}", data.borrow());
    }
}
