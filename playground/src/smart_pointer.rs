#[cfg(test)]
mod smart_pointer {
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

    #[test]
    fn test_cow_basic() {
        use std::borrow::Cow;

        fn insert_prefix_cow<'a>(strs: &'a Vec<String>, prefix: &'a str) -> Vec<Cow<'a, String>> {
            strs.into_iter()
                .filter_map(|s| match s.starts_with(prefix) {
                    true => Some(Cow::Borrowed(s)),
                    false => Some(Cow::Owned(prefix.to_owned() + s)),
                })
                .collect::<Vec<Cow<String>>>()
        }

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
}
