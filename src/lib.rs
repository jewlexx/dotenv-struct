use quote::quote;

#[proc_macro]
pub fn dotenv(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote! {}.into()
}
