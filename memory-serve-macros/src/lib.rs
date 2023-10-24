use proc_macro::TokenStream;
use std::path::Path;
use utils::list_assets;

mod asset;
mod utils;

use crate::asset::Asset;

#[proc_macro]
pub fn load_assets(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim_matches('"');
    let path = Path::new(&input);

    // skip if a subscriber is already registered (for instance by rust_analyzer)
    let _ = tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .try_init();

    if !path.exists() {
        panic!("The path {:?} does not exists!", path);
    }

    let files: Vec<Asset> = list_assets(path);

    let route = files.iter().map(|a| &a.route);
    let content_type = files.iter().map(|a| &a.content_type);
    let etag = files.iter().map(|a| &a.etag);
    let bytes = files.iter().map(|a| &a.bytes);
    let brotli_bytes = files.iter().map(|a| &a.brotli_bytes);

    quote::quote! {
        &[
            #(memory_serve::Asset {
                route: #route,
                content_type: #content_type,
                etag: #etag,
                bytes: #bytes,
                brotli_bytes: #brotli_bytes,
            }),*
        ]
    }
    .into()
}