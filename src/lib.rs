use quote::quote;

#[proc_macro]
pub fn dotenv(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let dotenv_path = match dotenv::dotenv() {
        Ok(v) => v,
        Err(_) => return quote! { compile_error!("Could not find .env file") }.into(),
    };

    quote! {}.into()
}
