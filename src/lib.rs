use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;

use thiserror::Error as AsError;

#[derive(Debug, AsError)]
enum DotenvError {
    #[error("failed to read .env file")]
    IoError(#[from] std::io::Error),
    #[error("failed to parse .env file")]
    ParseError(#[from] dotenv::Error),
}

#[proc_macro]
pub fn dotenv(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv_inner(item.into()).into()
}

fn dotenv_inner(_item: TokenStream) -> Result<TokenStream, DotenvError> {
    let dotenv_path = match dotenv::dotenv() {
        Ok(v) => v,
        Err(_) => return quote! { compile_error!("Could not find .env file") },
    };

    let file = File::open(dotenv_path);

    quote! {}
}
