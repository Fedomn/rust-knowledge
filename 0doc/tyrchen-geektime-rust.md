## tyrchen geektime-rust learning notes

[Go vs Rust](https://trio.dev/blog/golang-vs-rust)

Go for the code that has to ship tomorrow, Rust for the code that has to keep running for the next five years.

### Rust学习资料

文档：
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust 高级特性](https://doc.rust-lang.org/nomicon/)
- [Rust文档系统](https://docs.rs/)
- [Rust标准库文档](https://doc.rust-lang.org/stable/std/)


博客：
- [This Week in Rust](https://github.com/rust-lang/this-week-in-rust)
- [Rust 语言开源杂志](https://github.com/RustMagazine?type=source)

视频：
- [Beginner’s Series to: Rust](https://www.youtube.com/playlist?list=PLlrxD0HtieHjbTjrchBwOVks_sr8EVW1x)
- [Rust中高级视频](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ)

### Ownership

符合 Copy 语义的类型，在你赋值或者传参时，值会自动按位拷贝。
换句话说，当你要移动一个值，如果值的类型实现了 Copy trait，就会自动使用 Copy 语义进行拷贝，否则使用 Move 语义进行移动。

- 原生类型，包括函数、不可变引用和裸指针实现了 Copy；
- 数组和元组，如果其内部的数据结构实现了 Copy，那么它们也实现了 Copy；
- 可变引用没有实现 Copy；
- 非固定大小的数据结构，没有实现 Copy。

- 所有权：一个值只能被一个变量所拥有，且同一时刻只能有一个所有者，当所有者离开作用域，其拥有的值被丢弃，内存得到释放。
- Move 语义：赋值或者传参会导致值 Move，所有权被转移，一旦所有权转移，之前的变量就不能访问。
- Copy 语义：如果值实现了 Copy trait，那么赋值或传参会使用 Copy 语义，相应的值会被按位拷贝（浅拷贝），产生新的值。

```rust
fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    // all iXX and uXX, usize/isize, fXX implement Copy trait
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    // function (actually a pointer) is Copy
    is_copy::<fn()>();

    // raw pointer is Copy
    is_copy::<*const String>();
    is_copy::<*mut String>();

    // immutable reference is Copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    // array/tuple with values which is Copy is Copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    // unsized or dynamic sized type is not Copy
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<u8>>();
    is_copy::<String>();

    // mutable reference is not Copy
    is_copy::<&mut String>();

    // array / tuple with values that not Copy is not Copy
    is_copy::<[Vec<u8>; 4]>();
    is_copy::<(String, u32)>();
}
```

---

Borrow 语义允许一个值的所有权，在不发生转移的情况下，被其它上下文使用。就好像住酒店或者租房那样，旅客 / 租客只有房间的临时使用权，但没有它的所有权。另外，Borrow 语义通过引用语法（& 或者 &mut）来实现。

在 Rust 中，“借用”和“引用”是一个概念。所有的引用都只是借用了“临时使用权”，它并不破坏值的单一所有权约束。

#### 只读借用 / 引用

Rust 没有传引用的概念，Rust 所有的参数传递都是传值，不管是 Copy 还是 Move。所以在 Rust 中，你必须显式地把某个数据的引用，传给另一个函数。

Rust 的引用实现了 Copy trait，所以按照 Copy 语义，这个引用会被复制一份交给要调用的函数。对这个函数来说，它并不拥有数据本身，数据只是临时借给它使用，所有权还在原来的拥有者那里。

```rust
fn test() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    // addr of value: 0x70000e365810(0x70000e365810), addr of data 0x70000e3658d0, data1: 0x70000e365828
}
```

#### 可变借用 / 引用
- 多个可变引用共存：它破坏了循环的不变性（loop invariant），容易导致死循环甚至系统崩

```rust
fn test() {
    let mut data = vec![1, 2, 3];

    for item in data.iter_mut() {
        data.push(*item + 1); // cannot borrow `data` as mutable more than once at a time
    }
}
```

- 同时有一个可变引用和若干个只读引用：堆上的数据预留的空间不够了，就会重新分配一片足够大的内存，把之前的值拷过来，然后释放旧的内存。这样就会让 data1 中保存的 &data[0] 引用失效，导致内存安全问题。

```rust
fn test() {
    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];
    println!("data[0]: {:p}", &data[0]);

    for i in 0..100 {
        data.push(i); // cannot borrow `data` as mutable because it is also borrowed as immutable
    }

    println!("data[0]: {:p}", &data[0]);
    println!("boxed: {:p}", &data1);
}
```

- 为了保证内存安全，Rust 对可变引用的使用也做了严格的约束：
  - 在一个作用域内，仅允许一个活跃的可变引用。所谓活跃，就是真正被使用来修改数据的可变引用，如果只是定义了，却没有使用或者当作只读引用使用，不算活跃。
  - 在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。

