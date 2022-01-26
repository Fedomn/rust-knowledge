#[test]
fn test_custom_hash_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::Hash;
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Student<'a> {
        name: &'a str,
        age: u8,
    }

    impl<'a> Student<'a> {
        fn new(name: &'a str, age: u8) -> Student {
            Student { name, age }
        }
    }

    let mut hasher = DefaultHasher::new();
    let student = Student::new("John", 20);
    student.hash(&mut hasher);

    let mut map = HashMap::new();
    // 实现了 Hash / PartialEq / Eq 的数据结构可以作为 HashMap 的 key
    map.insert(student, "John");

    println!("{:?}", map);
}
