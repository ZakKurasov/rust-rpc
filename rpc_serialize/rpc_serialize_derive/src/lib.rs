extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::{TokenStream};
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, Fields, FieldsNamed, DeriveInput};

#[proc_macro_derive(AsyncSerialize)]
pub fn derive_async_serialize(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item);
    let output = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let fields = named.iter()
                    .map(|field| {
                        let ident: Ident = field.ident.clone().expect("Ident");
                        quote! { state.serialize_field(stringify!(#ident), &self.#ident).await?; }
                    })
                    .collect::<Vec<_>>();
                let fields_count = fields.len();
                quote! {
                    #[async_trait::async_trait]
                    impl crate::ser::AsyncSerialize for #ident {
                        async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::ser::AsyncSerializer {
                            use crate::ser::AsyncSerializeStruct;

                            let mut state = serializer.serialize_struct(stringify!(#ident), #fields_count).await?;
                            #(#fields)*
                            state.end().await
                        }
                    }
                }
            },
            _ => panic!("only structs supported")
        },
        _ => panic!("only structs supported")
    };
    output.into()
}