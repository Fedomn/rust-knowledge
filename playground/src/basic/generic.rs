#[cfg(test)]
mod generic_test {
    #[test]
    fn phantom_data_test() {
        use std::marker::PhantomData;

        #[derive(Debug, Default, PartialEq, Eq)]
        pub struct Identifier<T> {
            inner: u64,
            // PhantomData<T> is a marker type that can be used to tell the compiler
            _tag: PhantomData<T>,
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        pub struct User {
            id: Identifier<Self>,
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        pub struct Product {
            id: Identifier<Self>,
        }

        let user = User::default();
        let product = Product::default();

        // 两个 id 不能比较，因为他们属于不同的类型
        // assert_ne!(user.id, product.id);

        assert_eq!(user.id.inner, product.id.inner);
    }

    #[test]
    fn generic_as_types_test() {
        // https://rust-unofficial.github.io/patterns/functional/generics-type-classes.html

        #[derive(Debug, Default)]
        pub struct Identifier<T> {
            inner: T,
        }

        impl<T> Identifier<T> {
            pub fn new(inner: T) -> Self {
                Self { inner }
            }
        }

        impl<T> Identifier<T>
        where
            T: Copy,
        {
            // only for type that impl copy trait
            pub fn id(&self) -> T {
                self.inner
            }
        }

        pub type UserId<'a> = Identifier<&'a str>;
        pub type ProductId = Identifier<String>;
        impl<T> Identifier<T>
        where
            T: Clone,
        {
            pub fn clone_id(&self) -> T {
                self.inner.clone()
            }
        }

        let num = Identifier::<i32>::new(12);
        assert!(num.id() == 12);
        let userid = UserId::new("userid");
        assert!(userid.id() == "userid");
        let productid = ProductId::new("productid".to_string());
        assert!(productid.clone_id() == "productid");
    }
}
