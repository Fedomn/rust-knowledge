#[cfg(test)]
mod concurrency {
    use std::{sync, thread};
    use std::sync::Arc;
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

    #[test]
    fn atomics_test() {
        use std::sync::atomic::{AtomicBool, Ordering};

        use std::{cell::RefCell, fmt, sync::Arc, thread};

        struct Lock<T> {
            locked: AtomicBool,
            data: RefCell<T>,
        }

        impl<T> fmt::Debug for Lock<T>
        where
            T: fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Lock<{:?}>", self.data.borrow())
            }
        }

        // SAFETY: 我们确信 Lock<T> 很安全，可以在多个线程中共享
        unsafe impl<T> Sync for Lock<T> {}

        impl<T> Lock<T> {
            pub fn new(data: T) -> Self {
                Self {
                    locked: AtomicBool::new(false),
                    data: RefCell::new(data),
                }
            }

            /// Relaxed，这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序执行。
            ///
            /// Release，当我们写入数据（比如上面代码里的 store）的时候，如果用了 Release order，那么：
            ///     对于当前线程，任何读取或写入操作都不能被乱序排在这个 store 之后。也就是说，在上面的例子里，CPU 或者编译器不能把 **3 挪到 **4 之后执行。
            ///     对于其它线程，如果使用了 Acquire 来读取这个 atomic 的数据， 那么它们看到的是修改后的结果。上面代码我们在 compare_exchange 里使用了 Acquire 来读取，所以能保证读到最新的值。
            ///
            /// Acquire，而 Acquire 是当我们读取数据的时候，如果用了 Acquire order，那么：
            ///     对于当前线程，任何读取或者写入操作都不能被乱序排在这个读取之前。在上面的例子里，CPU 或者编译器不能把 **3 挪到 **1 之前执行。
            ///     对于其它线程，如果使用了 Release 来修改数据，那么，修改的值对当前线程可见。
            ///
            /// AcqRel，是 Acquire 和 Release 的结合，同时拥有 Acquire 和 Release 的保证。这个一般用在 fetch_xxx 上，比如你要对一个 atomic 自增 1，你希望这个操作之前和之后的读取或写入操作不会被乱序，并且操作的结果对其它线程可见。
            ///
            /// SeqCst， 是最严格的 ordering，除了 AcqRel 的保证外，它还保证所有线程看到的所有 SeqCst 操作的顺序是一致的。
            pub fn lock(&self, op: impl FnOnce(&mut T)) {
                // 如果没拿到锁，就一直 spin
                while self
                    .locked
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_err()
                {
                    // 性能优化：compare_exchange 需要独占访问，当拿不到锁时，我们
                    // 先不停检测 locked 的状态，直到其 unlocked 后，再尝试拿锁
                    while self.locked.load(Ordering::Relaxed) == true {}
                } // **1

                // 已经拿到并加锁，开始干活
                op(&mut self.data.borrow_mut()); // **3

                // 解锁
                self.locked.store(false, Ordering::Release); // **4
            }
        }

        let data = Arc::new(Lock::new(0));

        let data1 = data.clone();
        let t1 = thread::spawn(move || {
            data1.lock(|v| *v += 10);
        });

        let data2 = data.clone();
        let t2 = thread::spawn(move || {
            data2.lock(|v| *v *= 10);
        });
        t1.join().unwrap();
        t2.join().unwrap();

        println!("data: {:?}", data);
    }
}
