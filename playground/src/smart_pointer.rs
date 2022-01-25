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
}
