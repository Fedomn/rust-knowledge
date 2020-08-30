#[cfg(test)]
mod ownership {
    #[test]
    fn scope() {
        // SCOPE owns a VARIABLE owns a VALUE

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
}
