#[cfg(test)]
mod generic {
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
}
