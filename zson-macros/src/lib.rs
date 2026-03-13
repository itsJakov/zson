use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
                map.insert(stringify!(#ident).into(), self.#ident.encode());
            }
        });

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics zson::Encodable #ty_generics for #name #where_clause {
            fn encode(&self) -> zson::Value {
                let mut map = zson::ObjectMap::new();
                #(#field_decoders;)*
                return zson::Value::Object(map);
            }
        }
    }.into()
}

#[proc_macro_derive(Decodable)]
pub fn derive_decodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Struct(struct_data) = input.data else {
        panic!("Only structs are supported");
    };

    let Fields::Named(fields) = struct_data.fields else {
        panic!("Only named fields are supported");
    };

    let fields = fields.named
        .iter()
        .map(|field| { field.ident.as_ref().unwrap() })
        .map(|ident| {
            quote! {
                #ident: zson::Decodable::decode(map.remove(stringify!(#ident))?)?
            }
        });
    
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics zson::Decodable #ty_generics for #name #where_clause {
            fn decode(value: zson::Value) -> Option<Self> {
                let zson::Value::Object(mut map) = value else {
                    return None;
                };
                Some(Self {
                    #(#fields,)*
                })
            }
        }
    }.into()
}