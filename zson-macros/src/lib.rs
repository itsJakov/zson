use proc_macro::{TokenStream};
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use quote::quote;

#[proc_macro_derive(Encodable)]
pub fn derive_encodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Struct(struct_data) = input.data else {
        panic!("Only structs are supported");
    };

    let Fields::Named(fields) = struct_data.fields else {
        panic!("Only named fields are supported");
    };

    let field_decoders = fields.named
        .iter()
        .map(|field| { field.ident.as_ref().unwrap() })
        .map(|ident| {
            quote! {
                map.insert(stringify!(#ident).to_owned(), self.#ident.encode());
            }
        });

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics zson::Encodable #ty_generics for #name #where_clause {
            fn encode(&self) -> zson::Value {
                let mut map: std::collections::HashMap<String, zson::Value> = std::collections::HashMap::new();
                #(#field_decoders;)*
                return zson::Value::Object(map);
            }
        }
    }.into()
}