#[cfg(test)]
mod basic {
    #[test]
    fn variable() {
        // variable is immutable
        let a = 1;
        println!("{}", a);
        // but we can declare a new variable, with the same name: a
        let a = 3;
        println!("{}", a);

        // Rust’s const as a “label” to a constant value.
        // During compile time they get replaced by their actual values in all the places they are used.
        const C: &str = "122";
        println!("{}", C);

        // integer
        let _ = 1_2_3; // decimal
        let _: i64 = 0xdeadbeef; // hex
        let _ = 0o444; // octal
        let _ = 0b0101_1010; // binary
        let _ = b'I'; // byte

        // float
        let _ = 2.0; // f64
        let _: f32 = 3.0; // f32

        let _ = '💯'; // char
        let _ = false; // bool
    }

    #[test]
    fn string() {
        // String is growable whereas &str is immutable and fixed size.
        let name = "fixed name"; // &str
        let name2 = String::from("growable name"); // String
        let name3 = "growable name".to_string(); // String
        let name4 = name3.as_str(); // &str
        println!("{} {} {} {}", name, name2, name3, name4);

        let parsed3 = "3".parse::<u32>().unwrap();
        println!("{}", parsed3);
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
    fn tuple() {
        let point = (10, 20.5);
        let (x, y) = point;
        println!("x: {}, y: {}", x, y);
        println!("x: {}, y: {}", point.0, point.1);

        // {...} is an expression.
        let (x, y) = {
            let x = 1;
            (x + 1, 5) // <- here it returns a tuple: (2, 5)
        };
        println!("{} {}", x, y);
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

    #[test]
    fn function() {
        fn calc(a: i8, b: i8) -> i8 {
            a + b
        }

        let calc2 = |a: i8, b: i8| a + b;

        println!("{} {}", calc(1, 2), calc2(3, 4));
    }

    #[test]
    fn control_flow() {
        fn while_flow() {
            let mut res = String::from("");
            let mut count = 0;
            while count < 10 {
                res += count.to_string().as_str();
                count += 1;
            }
            println!("{}", res);
        }
        while_flow();

        fn iter_flow() {
            let mut res = String::new();

            for each in [1, 2, 3, 4, 5].iter() {
                res += format!("{}", each).as_str();
            }
            for each in 1..5 {
                res += format!("{}", each).as_str();
            }
            println!("{}", res);
        }
        iter_flow();
    }

    #[test]
    fn stream() {
        let nums = vec![1, 2, 3];
        let double = |n: &i32| -> i32 { n * 2 };
        let less_than_10 = |n: &i32| -> bool { *n < 10 };
        let result: Vec<i32> = nums.iter().map(double).filter(less_than_10).collect();
        println!("{:?}", result);
    }

    #[test]
    fn pattern_match() {
        fn color(color: &str) -> &str {
            match color {
                "red" => "red",
                "blue" => "blue",
                _ => "unknown",
            }
        }
        println!("{} {}", color("red"), color(""));
    }

    #[test]
    fn destructuring() {
        struct Person {
            name: String,
            age: u8,
        }
        let rgb = [1, 12, 34];
        let [red, green, blue] = rgb;
        println!("{} {} {}", red, green, blue);

        let person = Person {
            name: "name".to_string(),
            age: 12,
        };
        let Person { name, age } = person;
        println!("{} {}", name, age);
    }

    #[test]
    #[allow(dead_code)]
    fn enum_match() {
        enum Direction {
            Forward,
            Backward,
            Left,
            Right,
        }

        enum Operation {
            PowerOn,
            PowerOff,
            Move(Direction),
            Rotate,
            TakePhoto { is_landscape: bool, zoom_level: i32 },
        }

        fn operate_drone(operation: Operation) {
            match operation {
                Operation::PowerOn => println!("Power On"),
                Operation::PowerOff => println!("Power Off"),
                Operation::Move(direction) => move_drone(direction),
                Operation::Rotate => println!("Rotate"),
                Operation::TakePhoto {
                    is_landscape,
                    zoom_level,
                } => println!("TakePhoto {}, {}", is_landscape, zoom_level),
            }
        }

        fn move_drone(direction: Direction) {
            match direction {
                Direction::Forward => println!("Move Forward"),
                Direction::Backward => println!("Move Backward"),
                Direction::Left => println!("Move Left"),
                Direction::Right => println!("Move Right"),
            }
        }

        operate_drone(Operation::Move(Direction::Forward));
        operate_drone(Operation::TakePhoto {
            is_landscape: true,
            zoom_level: 10,
        })
    }
}
