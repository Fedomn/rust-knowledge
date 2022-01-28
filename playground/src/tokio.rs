/// https://tokio.rs/tokio/tutorial/channels
#[cfg(test)]
mod tokio_test {

    // multi-producer, single-consumer channel. Many values can be sent.
    #[tokio::test]
    async fn mpsc_channel_test() {
        use tokio::sync::mpsc;
        use tokio::time::{sleep, Duration};

        let (tx, mut rx) = mpsc::channel(1);
        let tx2 = tx.clone();

        tokio::spawn(async move {
            sleep(Duration::from_millis(200)).await;
            tx.send(1).await.unwrap();
        });

        tokio::spawn(async move {
            sleep(Duration::from_millis(500)).await;
            tx2.send(2).await.unwrap();
        });

        assert_eq!(rx.recv().await.unwrap(), 1);
        assert_eq!(rx.recv().await.unwrap(), 2);
    }

    // single-producer, single consumer channel. A single value can be sent.
    #[tokio::test]
    async fn oneshot_channel_test() {
        use tokio::sync::oneshot;
        use tokio::time::{interval, sleep, Duration};

        let (send, mut recv) = oneshot::channel();
        let mut interval = interval(Duration::from_millis(100));

        tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;
            send.send("shut down").unwrap();
        });

        loop {
            tokio::select! {
                _ = interval.tick() => println!("Another 100ms"),
                msg = &mut recv => {
                    println!("Got message: {}", msg.unwrap());
                    break;
                }
            }
        }
    }

    // multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
    #[tokio::test]
    async fn broadcast_channel_test() {
        use tokio::sync::broadcast;

        let (tx, mut rx1) = broadcast::channel(16);
        let mut rx2 = tx.subscribe();

        tokio::spawn(async move {
            assert_eq!(rx1.recv().await.unwrap(), 10);
            assert_eq!(rx1.recv().await.unwrap(), 20);
        });

        tokio::spawn(async move {
            assert_eq!(rx2.recv().await.unwrap(), 10);
            assert_eq!(rx2.recv().await.unwrap(), 20);
        });

        tx.send(10).unwrap();
        tx.send(20).unwrap();
    }
}
