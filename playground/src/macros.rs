#[cfg(test)]
#[allow(clippy::vec_init_then_push)]
mod macros_test {

    #[macro_export]
    macro_rules! my_vec {
        // 没带任何参数的 my_vec，我们创建一个空的 vec，最好带着完整的命名空间
        () => {
            std::vec::Vec::new()
        };

        // 处理 my_vec![1, 2, 3]
        // 条件捕获的参数使用 $ 开头的标识符来声明
        // $(...),* 告诉编译器可以匹配任意多个以逗号分隔的表达式，然后捕获到的每一个表达式可以用 $el 来访问。
        // $(...)*  在执行的代码块中，我们也要相应地使用 $(...)* 展开，匹配多少条 $el ，就需要展开多少条 push。
        ($($el:expr),*) => ({
            let mut v = std::vec::Vec::new();
            $(v.push($el);)*
            v
        });

        // 处理 my_vec![0; 10]
        ($el:expr; $n:expr) => {
            std::vec::from_elem($el, $n)
        };
    }

    #[test]
    fn test_declarative_macros() {
        let mut v = my_vec![];
        v.push(1);
        println!("{:?}", v);

        trace_macros!(true);
        let v2 = my_vec![1, 2, 3];
        trace_macros!(false);
        println!("{:?}", v2);

        println!("{:?}", my_vec! {1, 2, 3});
        println!("{:?}", my_vec![1; 3]);
        
    }

    use procmacrotestlib::query;

    #[test]
    fn proc_macro_test_1() {
        query!("select * from t where id > 1;");
    }
}
