extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn query(_input: TokenStream) -> TokenStream {
    // println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}
