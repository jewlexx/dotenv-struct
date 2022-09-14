use proc_macro2::TokenStream;
use quote::quote;
use std::{fs::File, io::Read};

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

    let mut file = File::open(dotenv_path)?;

    let dotenv_contents = {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        buf.replace("export ", "");

        buf
    };

    let dotenv_consts = dotenv_contents
        .lines()
        .map(|line| {
            let (key, value) = {
                let mut iter = line.splitn(2, '=');

                let key = iter.next().unwrap();
                let value = iter.next().unwrap();

                (key, value)
            };

            let key = proc_macro2::Literal::string(key);
            let value = proc_macro2::Literal::string(value);

            quote! {
                const #key: &str = #value;
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    Ok(quote! {
        mod dotenv {
            #(#dotenv_consts)*
        }
    })
}
