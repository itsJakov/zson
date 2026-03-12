use proc_macro::{TokenStream};
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use quote::quote;

#[proc_macro_derive(Encodable)]
pub fn derive_encodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(f) => f.named,
            _ => panic!("Only named fields are supported"),
        }
        _ => panic!("Only structs are supported"),
    };

    let decode_stmt = fields.iter().map(|field| {
        let field = field.ident.as_ref().unwrap();
        let field_name = field.to_string();
        quote! {
            map.insert(#field_name.to_owned(), self.#field.encode());
        }
    });

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics zson::Encodable #ty_generics for #name #where_clause {
            fn encode(&self) -> zson::Value {
                let mut map: HashMap<String, zson::Value> = std::collections::HashMap::new();
                #(#decode_stmt;)*
                return zson::Value::Object(map);
            }
        }
    }.into()
}