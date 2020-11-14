#![deny(warnings)]

extern crate proc_macro;

use proc_macro::TokenStream;
use std::{fs, path::Path};

#[proc_macro_attribute]
pub fn monotonic(args: TokenStream, input: TokenStream) -> TokenStream {

    println!("Args: {:#?}", args);
    println!("Args: {:#?}", input);

    let ts = input;

    // Try to write the expanded code to disk
    if Path::new("target").exists() {
        fs::write("target/rtic-expansion.rs", ts.to_string()).ok();
    }

    ts.into()
}
