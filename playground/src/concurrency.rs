#[cfg(test)]
mod concurrency {
    use std::sync::Arc;
    use std::{sync, thread};
    use sync::Mutex;

    #[test]
    fn spawn() {
        let v = vec![1, 2, 3];

        // The move closure is often used alongside thread::spawn because it allows you to use data from one thread in another thread.
        // Move: Closure taking ownership of its captures; i.e., v transferred to closure.
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    #[test]
    fn mpsc() {
        // mpsc stands for multiple producer, single consumer
        let (tx, rx) = sync::mpsc::channel();
        let tx1 = tx.clone();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            // println!("val is {}", val); // error => value borrowed here after move
        });

        thread::spawn(move || {
            let val = String::from("you");
            tx1.send(val).unwrap();
        });

        for recv in rx {
            println!("Got: {}", recv)
        }
    }

    #[test]
    fn mutex() {
        // Mutex<T>: To access the data inside the mutex, we use the lock method to acquire the lock.
        let m = Mutex::new(1);
        {
            // lock returns a smart pointer called MutexGuard, wrapped in a LockResult
            let mut num = m.lock().unwrap();

            // MutexGuard impl Deref to point at inner data
            println!("m = {:?}, num = {:?}", m, num);
            *num = 5;

            // MutexGuard impl Drop to release lock automatically when it goes out of scope
        }
        println!("m = {:?}", m);
        let _ = m.lock().unwrap();
        let _ = m.lock().unwrap();
    }

    #[test]
    fn mutex_shared_in_multi_threads() {
        // counter ownership first moved into thread closure
        // println! will also get counter ownership
        // so, we used smart pointer: Arc<T> to share ownership across multiple threads
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = counter.clone();
            let h = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1
            });
            handles.push(h);
        }

        for h in handles {
            h.join().unwrap()
        }

        println!("Result: {}", counter.lock().unwrap())
    }
}
