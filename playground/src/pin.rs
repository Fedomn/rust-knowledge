#[cfg(test)]
mod pin_test {
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[test]
    fn self_reference_caused_issue() {
        #[derive(Debug)]
        struct SelfReference {
            name: String,
            name_ptr: *const String,
        }

        impl SelfReference {
            fn new(name: impl Into<String>) -> Self {
                Self {
                    name: name.into(),
                    name_ptr: std::ptr::null(),
                }
            }

            pub fn init(&mut self) {
                self.name_ptr = &self.name as *const String;
            }

            pub fn print_name(&self) {
                println!(
                    "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
                    self,
                    &self.name,
                    self.name_ptr,
                    self.name,
                    // 在使用 ptr 是需要 unsafe
                    // SAFETY: 这里 name_ptr 潜在不安全，会指向旧的位置
                    unsafe { &*self.name_ptr },
                );
            }
        }

        fn move_it(data: SelfReference) -> SelfReference {
            data
        }

        fn move_creates_issue() -> SelfReference {
            let mut data = SelfReference::new("Hi");
            data.init();

            // 不 move，一切正常
            data.print_name();

            println!("after move");
            let data = move_it(data);

            // move 之后，name_ref 指向的位置是已经失效的地址
            // 只不过现在 move 前的地址还没被回收挪作它用
            data.print_name();
            data
        }

        // 验证逻辑
        let data = move_creates_issue();
        println!("data: {:?}", data);
        // 如果把下面这句注释掉，程序运行会直接 segment error，这就是Rust 下，自引用类型带来的潜在危害：内存访问是不安全的。解决：使用Pin
        // data.print_name();
        print!("\\n");
    }

    #[test]
    fn self_reference_pin() {
        #[derive(Debug)]
        struct SelfReference {
            name: String,
            name_ptr: *const String,
            // PhantomPinned 占位符
            _marker: PhantomPinned,
        }

        impl SelfReference {
            fn new(name: impl Into<String>) -> Self {
                Self {
                    name: name.into(),
                    name_ptr: std::ptr::null(),
                    _marker: PhantomPinned,
                }
            }

            pub fn init(self: Pin<&mut Self>) {
                let name_ptr = &self.name as *const String;
                // SAFETY: 这里并不会把任何数据从 &mut SelfReference 中移走
                let this = unsafe { self.get_unchecked_mut() };
                this.name_ptr = name_ptr;
            }

            pub fn print_name(&self) {
                println!(
                    "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
                    self, // TODO still a little confused for self address, why not same as ref address?
                    &self.name,
                    self.name_ptr,
                    self.name,
                    // 在使用 ptr 是需要 unsafe
                    // SAFETY: 这里 name_ptr 潜在不安全，会指向旧的位置
                    unsafe { &*self.name_ptr },
                );
            }
        }

        fn move_pinned(data: Pin<&mut SelfReference>) {
            println!("{:?} (ref addr: {:p})", data, &data);
        }

        #[allow(dead_code)]
        fn move_it(data: SelfReference) {
            println!("{:?} ({:p})", data, &data);
        }

        fn move_creates_issue() {
            let mut data = SelfReference::new("Hi");
            let mut data = unsafe { Pin::new_unchecked(&mut data) };
            SelfReference::init(data.as_mut());

            // 不 move，一切正常
            println!("{:?} (ref addr: {:p})", data, &data);
            data.print_name();

            println!("after move");
            // 现在只能拿到 pinned 后的数据，所以 move 不了之前
            move_pinned(data.as_mut());
            data.print_name();

            // 你无法拿回 Pin 之前的 SelfReference 结构，所以调用不了 move_it
            // move_it(data);
        }

        // 验证逻辑
        move_creates_issue();
    }
}
