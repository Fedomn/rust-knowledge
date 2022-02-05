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
}
