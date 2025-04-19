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

#[cfg(test)]
mod simple_future_test {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };

    /// A future which returns a random number when it resolves.
    #[derive(Default)]
    struct RandFuture;

    impl Future for RandFuture {
        // Every future has to specify what type of value it returns when it resolves.
        // This particular future will return a u16.
        type Output = u16;

        // random number generator

        /// The `Future` trait has only one method, named "poll".
        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(1u16)
        }
    }

    #[tokio::test]
    async fn test_simple_future() {
        let res = RandFuture.await;
        assert_eq!(res, 1);
    }
}

#[cfg(test)]
mod nested_future_test {
    use std::{
        pin::Pin,
        task::{Context, Poll},
        time::{Duration, Instant},
    };

    use futures::Future;

    /// refer:
    /// https://blog.cloudflare.com/pin-and-unpin-in-rust/
    /// https://hashrust.com/blog/moves-copies-and-clones-in-rust/
    #[pin_project::pin_project]
    pub struct TimedWrapper<Fut: Future> {
        // For each field, we need to choose whether `project` returns an
        // unpinned (&mut T) or pinned (Pin<&mut T>) reference to the field.
        // By default, it assumes unpinned:
        start: Option<Instant>,
        // Opt into pinned references with this attribute:
        #[pin]
        future: Fut,
    }

    impl<Fut: Future> TimedWrapper<Fut> {
        pub fn new(future: Fut) -> Self {
            Self {
                future,
                start: None,
            }
        }
    }

    impl<Fut: Future> Future for TimedWrapper<Fut> {
        // This future will output a pair of values:
        // 1. The value from the inner future
        // 2. How long it took for the inner future to resolve
        type Output = (Fut::Output, Duration);

        fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
            // This returns a type with all the same fields, with all the same types,
            // except that the fields defined with #[pin] will be pinned.
            let this = self.project();

            // Call the inner poll, measuring how long it took.
            let start = this.start.get_or_insert_with(Instant::now);
            let inner_poll = this.future.poll(cx);
            let elapsed = start.elapsed();

            match inner_poll {
                // The inner future needs more time, so this future needs more time too
                Poll::Pending => Poll::Pending,
                // Success!
                Poll::Ready(output) => Poll::Ready((output, elapsed)),
            }
        }
    }

    #[tokio::test]
    async fn test_nested_future() {
        let res = TimedWrapper::new(async { 111 }).await;
        assert_eq!(res.0, 111);
        println!("elapsed: {:?}", res.1);
    }
}
