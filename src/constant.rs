#[cfg(test)]
mod constant {
    // https://doc.rust-lang.org/edition-guide/rust-next/const-fn.html
    const fn ten() -> i8 {
        10
    }

    #[test]
    fn constfn_test() {
        const TEN: i8 = ten();
        println!("{}", TEN)
    }
}
