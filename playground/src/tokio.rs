/// https://tokio.rs/tokio/tutorial/channels
#[cfg(test)]
mod tokio_channel_test {

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

#[cfg(test)]
mod tokio_mutex_test {
    #[tokio::test]
    async fn mutex_test() {
        use anyhow::Result;
        use std::{sync::Arc, time::Duration};
        use tokio::sync::Mutex;

        struct DB;

        impl DB {
            // 假装在 commit 数据
            async fn commit(&mut self) -> Result<usize> {
                Ok(42)
            }
        }

        let db1 = Arc::new(Mutex::new(DB));
        let db2 = Arc::clone(&db1);

        tokio::spawn(async move {
            let mut db = db1.lock().await;
            // 因为拿到的 MutexGuard 要跨越 await，所以不能用 std::sync::Mutex
            // 只能用 tokio::sync::Mutex
            // 因为 tokio 实现了 work-stealing 调度，Future 有可能在不同的线程中执行，普通的 MutexGuard 编译直接就会出错，所以需要使用 tokio 的 Mutex。
            // see https://docs.rs/tokio/1.13.0/tokio/sync/struct.Mutex.html
            let affected = db.commit().await?;
            println!("db1: Total affected rows: {}", affected);
            Ok::<_, anyhow::Error>(())
        });

        tokio::spawn(async move {
            let mut db = db2.lock().await;
            let affected = db.commit().await?;
            println!("db2: Total affected rows: {}", affected);
            Ok::<_, anyhow::Error>(())
        });

        // 让两个 task 有机会执行完
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}
