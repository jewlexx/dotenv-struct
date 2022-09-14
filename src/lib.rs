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
    match dotenv_inner(item.into()) {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("{}", e);
            let msg = proc_macro2::Literal::string(&msg);

            return quote! {
                compile_error!(#msg);
            }
            .into();
        }
    }
    .into()
}

fn dotenv_inner(_item: TokenStream) -> Result<TokenStream, DotenvError> {
    let dotenv_path = dotenv::dotenv()?;

    let file = File::open(dotenv_path);

    quote! {}
}
