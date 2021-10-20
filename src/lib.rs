use proc_macro::TokenStream;
use proc_macro2::Span;

#[proc_macro_attribute]
pub fn my_macro(_args: TokenStream, _item: TokenStream) -> TokenStream {
    let float = syn::Lit::Float(syn::LitFloat::new("-10.0", Span::call_site()));
    panic!("float is parsed as: {:?}", float);
}