#[cfg(test)]
mod async_test {
    use futures::stream::BoxStream;
    use futures::StreamExt;
    use futures_async_stream::stream;

    struct MyExecutor {
        child: BoxStream<'static, String>,
    }

    // convert a stream of String to i32
    impl MyExecutor {
        #[stream(boxed, item = i32)]
        pub async fn s2i(self) {
            #[for_await]
            for x in self.child {
                yield x.parse().unwrap();
            }
        }
    }

    #[tokio::test]
    async fn test_collect() {
        let child = futures::stream::iter(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        let executor = MyExecutor {
            child: child.boxed(),
        };
        let res = executor.s2i().collect::<Vec<_>>().await;
        assert_eq!(res, vec![1, 2, 3]);
    }
}
