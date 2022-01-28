#[cfg(test)]
mod closure_test {
    use std::ops::Mul;

    #[test]
    fn closure_as_a_struct() {
        use std::{collections::HashMap, mem::size_of_val};

        // 长度为 0
        let c1 = || println!("hello world!");

        // 和参数无关，长度也为 0
        let c2 = |i: i32| println!("hello: {}", i);

        let name = String::from("tyr");
        let name1 = name.clone();
        let mut table = HashMap::new();
        table.insert("hello", "world");
        // 如果捕获一个引用，长度为 8
        let c3 = || println!("hello: {}", name);

        // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
        let c4 = move || println!("hello: {}, {:?}", name1, table);

        let name2 = name.clone();
        // 和局部变量无关，捕获了一个 String name2，closure 长度 24
        let c5 = move || {
            let x = 1;
            let name3 = String::from("lindsey");
            println!("hello: {}, {:?}, {:?}", x, name2, name3);
        };

        println!(
            "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}",
            size_of_val(&c1),
            size_of_val(&c2),
            size_of_val(&c3),
            size_of_val(&c4),
            size_of_val(&c5),
        )
    }

    #[test]
    fn fn_once_test() {
        let name = String::from("John");
        // 这个闭包啥也不干，只是把捕获的参数返回去
        let c = move |greeting: String| (greeting, name);

        let result = c("hello".to_string());

        assert_eq!(result.0, "hello");
        assert_eq!(result.1, "John");

        // 无法再次调用: use of moved value: `c`
        // let result = c("hi".to_string());
    }

    #[test]
    fn fn_mut_test() {
        let mut name = String::from("John");
        let mut c = move |greeting: String| {
            name.push('1');
            println!("{} {}", greeting, name);
        };

        c("hello".to_string());
        c("hi".to_string());
    }

    #[test]
    fn curry_test() {
        fn mul_curry<T>(x: T) -> impl Fn(T) -> T
        where
            T: Mul<Output = T> + Copy,
        {
            move |y| x * y
        }

        let c = mul_curry(2);
        assert_eq!(c(3), 6);
    }
}
