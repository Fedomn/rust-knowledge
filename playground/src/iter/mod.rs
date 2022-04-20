#[cfg(test)]
mod iter_test {
    use std::iter::FromIterator;

    #[test]
    fn from_iter_test() {
        struct MyCollection(Vec<i32>);

        impl MyCollection {
            fn new() -> Self {
                Self(Vec::new())
            }

            fn add(&mut self, value: i32) {
                self.0.push(value);
            }
        }

        // impl FromIterator
        impl FromIterator<i32> for MyCollection {
            fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
                let mut c = MyCollection::new();
                for i in iter {
                    c.add(i);
                }
                c
            }
        }

        let i = 0..5;
        let c: MyCollection = i.collect();
        assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    #[allow(clippy::vec_init_then_push, clippy::flat_map_identity)]
    fn option_auto_unwrap() {
        // `Result` and `Option` all implements `IntoIterator`
        // see https://doc.rust-lang.org/std/option/enum.Option.html#impl-IntoIterator-2
        let mut v = Vec::<Option<i8>>::new();
        v.push(Some(1));
        v.push(Some(2));
        v.push(Some(3));
        for ele in v.iter().flatten() {
            println!("{}", ele);
        }

        v.iter().flat_map(|x| x).for_each(|x| println!("{}", x));
    }
}
