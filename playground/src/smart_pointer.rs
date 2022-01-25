#[cfg(test)]
mod smart_pointer {
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::fmt::{Debug, Formatter};
    use std::ops::Deref;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use std::{fmt, thread};

    #[test]
    fn test_box_my_allocator() {
        use std::alloc::{AllocError, Allocator, Layout};
        use std::alloc::{GlobalAlloc, System};
        use std::ptr::NonNull;
        struct MyAllocator;
        unsafe impl Allocator for MyAllocator {
            fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
                let raw_ptr = unsafe { System.alloc(layout) };
                eprintln!("ALLOC: {:p}, size {}", raw_ptr, layout.size());
                let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
            }
            unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
                eprintln!("FREE: {:p}, size {}", ptr, layout.size());
                System.dealloc(ptr.as_ptr(), layout);
            }
        }

        #[allow(unused)]
        struct Matrix {
            data: [u8; 321],
        }

        impl Default for Matrix {
            fn default() -> Self {
                Self { data: [0; 321] }
            }
        }

        let data = Box::try_new_in(Matrix::default(), MyAllocator).unwrap();

        // 输出中有一个 321 大小的内存分配，是 println! 导致的
        println!(
            "!!! allocated memory: {:p}, len: {}",
            &*data,
            std::mem::size_of::<Matrix>()
        );

        // data 在这里 drop，可以在打印中看到 FREE
    }

    pub fn insert_prefix_cow<'a>(strs: &'a Vec<String>, prefix: &'a str) -> Vec<Cow<'a, String>> {
        strs.into_iter()
            .filter_map(|s| match s.starts_with(prefix) {
                true => Some(Cow::Borrowed(s)),
                false => Some(Cow::Owned(prefix.to_owned() + s)),
            })
            .collect::<Vec<Cow<String>>>()
    }

    #[test]
    fn test_cow_basic() {
        let strs = vec!["hi_rust".to_string(), "rust".to_string()];
        let p = "hi_";
        let new_strs = insert_prefix_cow(&strs, &p);
        println!("{:?}", new_strs);

        for i in 0..2 {
            println!("source addr: {:p}", &strs[i]);
            println!("cow addr: {:p}", &*new_strs[i]); // cow通过deref，获取原始数据
        }

        // 对于第一个hi_rust，它是 Cow::Borrowed，因为它是从 strs 中 borrow 的
        // 对于第二个rust，它是 Cow::Owned，因此它的地址发生了变化
        assert_eq!(strs[0].as_ptr(), new_strs[0].as_ref().as_ptr());
        assert_ne!(strs[1].as_ptr(), new_strs[1].as_ref().as_ptr());
    }

    #[test]
    fn mutex_guard_test() {
        let metrics = Arc::new(Mutex::new(HashMap::<Cow<str>, usize>::new()));
        for i in 0..30 {
            let m = metrics.clone();
            thread::spawn(move || {
                // 此时只有拿到 MutexGuard 的线程可以访问 HashMap
                let mut m = m.lock().unwrap();
                println!("I am {:?}, got lock", i);
                // Cow 实现了很多数据结构的 From trait，所以我们可以用 "hello".into() 生成 Cow
                let e = m.entry("key".into()).or_insert(0);
                *e += 1;
            });
        }

        thread::sleep(Duration::from_millis(100));

        println!("metrics {:?}", metrics.lock().unwrap());
    }

    #[test]
    fn custom_smart_pointer_string() {
        // 字符串较小时候，直接放在栈上，否则仍然使用String
        const MINI_STRING_MAX_LEN: usize = 30;

        struct MiniString {
            len: u8,
            data: [u8; MINI_STRING_MAX_LEN],
        }

        impl MiniString {
            fn new(s: impl AsRef<str>) -> Self {
                let bytes = s.as_ref().as_bytes();
                let len = bytes.len();
                let mut data = [0; MINI_STRING_MAX_LEN];
                data[..len].copy_from_slice(bytes);
                Self {
                    len: len as u8,
                    data,
                }
            }
        }

        use std::str;
        impl Deref for MiniString {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                str::from_utf8(&self.data[..self.len as usize]).unwrap()
            }
        }

        impl Debug for MiniString {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.deref())
            }
        }

        #[derive(Debug)]
        enum MyString {
            Inline(MiniString),
            Standard(String),
        }

        impl Deref for MyString {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                match *self {
                    // Using the ref keyword, the value is only borrowed, not moved
                    // & vs ref
                    // & denotes that your pattern expects a reference to an object. Hence & is a part of said pattern: &Foo matches different objects than Foo does.
                    // ref indicates that you want a reference to an unpacked value. It is not matched against: Foo(ref foo) matches the same objects as Foo(foo).
                    MyString::Inline(ref v) => v.deref(),
                    MyString::Standard(ref v) => v.deref(),
                }
            }
        }

        impl<T> From<T> for MyString
        where
            T: AsRef<str> + Into<String>,
        {
            fn from(s: T) -> Self {
                match s.as_ref().len() > MINI_STRING_MAX_LEN {
                    true => Self::Standard(s.into()),
                    false => Self::Inline(MiniString::new(s)),
                }
            }
        }

        impl fmt::Display for MyString {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.deref())
            }
        }

        // MyString 里，String 有 3 个 word，一共 24 字节，enum 的 tag + padding 最少 8 字节，整个结构占 32 字节
        // MiniString 有 30 字节 data + 1 字节 len，一共 31 字节
        let len1 = std::mem::size_of::<MyString>(); // 32
        let len2 = std::mem::size_of::<MiniString>(); // 31
        assert_eq!(len1, 32);
        assert_eq!(len2, 31);

        let s1: MyString = "hello world".into();
        let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

        println!("{:?}", s1);
        println!("{:?}", s2);
    }
}

#[cfg(test)]
mod cow_bench_test {
    extern crate test; // 声明外部crate依赖，并将其bind到当前作用于中

    use test::Bencher;

    use super::smart_pointer::*;

    fn insert_prefix_clone(strs: &Vec<String>, prefix: &str) -> Vec<String> {
        strs.into_iter()
            .filter_map(|s| match s.starts_with(prefix) {
                true => Some(s.clone()),
                false => Some(s.clone() + prefix),
            })
            .collect()
    }

    #[bench]
    #[ignore]
    fn test_cow(b: &mut Bencher) {
        let mut c = vec!["hi_rust".to_string(); 1024];
        let mut f = vec!["rust".to_string(); 1024];
        c.append(&mut f);
        let p = "hi_";
        b.iter(|| insert_prefix_cow(&c, &p));
    }

    #[bench]
    #[ignore]
    fn test_clone(b: &mut Bencher) {
        let mut c = vec!["hi_rust".to_string(); 1024];
        let mut f = vec!["rust".to_string(); 1024];
        c.append(&mut f);
        let p = "hi_";
        b.iter(|| insert_prefix_clone(&c, &p));
    }
}
