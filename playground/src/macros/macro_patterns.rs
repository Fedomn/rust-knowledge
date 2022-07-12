#[cfg(test)]
mod macro_patterns_test {
    /// https://veykril.github.io/tlborm/syntax-extensions/ast.html
    /// macro processing in Rust happens after the construction of the AST

    #[test]
    fn test_callback_pattern() {
        macro_rules! call_with_larch {
            ($callback:ident) => {
                $callback!(larch)
            };
        }

        macro_rules! recognise_tree {
            (larch) => {
                println!("#1, the Larch.")
            };
            (redwood) => {
                println!("#2, the Mighty Redwood.")
            };
            ($($other:tt)*) => {
                println!("I don't know; some kind of birch maybe?")
            };
        }

        call_with_larch!(recognise_tree);
    }

    #[test]
    fn test_internal_rules() {
        /// @as_expr as internal rules is embed in foo macro
        macro_rules! foo {
            (@as_expr $e:expr) => {$e};

            ($($tts:tt)*) => {
                foo!(@as_expr $($tts)*)
            };
        }

        dbg!(foo!(1 + 2));
    }
}
