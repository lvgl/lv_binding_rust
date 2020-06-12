use lvgl_codegen::{CodeGen, Rusty};
use proc_macro2::TokenStream;
use quote::quote;

fn main() {
    let codegen = CodeGen::new().unwrap();

    let widgets_impl: Vec<TokenStream> = codegen
        .get_widgets()
        .iter()
        .flat_map(|w| w.code(&()))
        .collect();

    let code = quote! {
        #(#widgets_impl)*
    };

    println!("{}", code.to_string());
}
