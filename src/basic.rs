#[cfg(test)]
mod basic {
    #[test]
    fn variable() {
        // default variable is immutable
        let a = 1;
        let b = 2;
        let x = false;
        // Rust’s const as a “label” to a constant value. During compile time they get replaced by their actual values in all the places they are used.
        const C: &str = "122";
        println!("{} {} {} {}", a, b, x, C);
    }

    #[test]
    fn string() {
        // String is growable whereas &str is immutable and fixed size.
        let name = "fixed name"; // &str
        let name2 = String::from("growable name"); // String
        let name3 = "growable name".to_string(); // String
        let name4 = name3.as_str(); // &str
        println!("{} {} {} {}", name, name2, name3, name4);
    }

    #[test]
    fn option() {
        // Rust doesn't’t even have a dedicated null data type. Instead it has something call Option
        fn read(path: &str) -> Option<&str> {
            if path != "" {
                return Some(path);
            }
            return None;
        }
        match read("path") {
            Some(val) => println!("read {}", val),
            None => println!("read none"),
        }
    }

    #[test]
    fn array() {
        // Array: fixed size
        let list = [1, 2, 3];
        println!("{:?}", list);
        // Vectors: grow/shrink in size
        let mut list = vec![1, 2, 3];
        list.push(4);
        println!("{:?}", list);
    }

    #[test]
    fn object() {
        // The `derive` attribute automatically creates the implementation required to make this `struct` printable with `fmt::Debug`.
        #[derive(Debug)]
        struct People {
            name: String,
            age: i16,
        }
        let people = People {
            name: "Jh".to_string(),
            age: 12,
        };
        // All std library types automatically are printable with {:?} too:
        println!("{:?}", people);
    }

    #[test]
    fn map() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("1", "1");
        map.insert("2", "2");
        println!("{:?}", map);
    }
}
