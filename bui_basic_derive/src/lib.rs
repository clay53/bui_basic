use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(SignalReciever)]
pub fn signal_reciever_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}