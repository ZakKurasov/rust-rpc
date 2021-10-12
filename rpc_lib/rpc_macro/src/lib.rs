extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate quote;
extern crate bincode;

use proc_macro::{TokenStream};
use quote::{quote, format_ident};
use syn::{ItemTrait, parse_macro_input, TraitItem, FnArg, Pat, Ident};

fn impl_client_method(service_name: &Ident, item: &TraitItem) -> proc_macro2::TokenStream {
    if let TraitItem::Method(method) = item {
        let method_name = &method.sig.ident;
        let arguments = &method.sig.inputs
            .iter()
            .collect::<Vec<_>>();
        let arguments_prints = &method.sig.inputs
            .iter()
            .map(|arg| {
                return if let FnArg::Typed(typed_arg) = arg {
                    if let Pat::Ident(name) = &*typed_arg.pat {
                        quote! {
                            bincode::serialize_into(&mut writer, &#name).expect(format!("Unable to send {} argument", stringify!(#name)).as_str());
                        }
                    } else {
                        quote!()
                    }
                } else {
                    quote!()
                }
            })
            .collect::<Vec<_>>();
        let return_type = &method.sig.output;
        return quote! {
            fn #method_name(#(#arguments), *) #return_type {
                use bincode;

                {
                    let mut writer = std::io::BufWriter::new(&mut *self.stream);
                    bincode::serialize_into(&mut writer, stringify!(#service_name)).expect("Unable to send service name");
                    bincode::serialize_into(&mut writer, stringify!(#method_name)).expect("Unable to send method name");
                    #(#arguments_prints)*
                    std::io::Write::flush(&mut writer).expect("Unable to flush writer");
                }

                let mut reader = std::io::BufReader::new(&mut *self.stream);
                return bincode::deserialize_from(&mut reader).expect("Unable to read return value");
            }
        };
    }
    quote!()
}

fn impl_service_client(service_name: &Ident, service_interface: &ItemTrait) -> proc_macro2::TokenStream {
    let client_name = format_ident!("{}Client", service_name.to_string());
    let methods = service_interface.items
        .iter()
        .map(|item| impl_client_method(service_name, item));
    quote! {
        pub struct #client_name {
            stream: Box<dyn rpc_lib::Stream>,
        }
        impl #client_name {
            pub fn new(stream: Box<dyn rpc_lib::Stream>) -> Self {
                Self {
                    stream
                }
            }
        }
        impl #service_name for #client_name {
            #(#methods)*
        }
    }
}

fn impl_wrapper_method(item: &TraitItem) -> proc_macro2::TokenStream {
    match item {
        TraitItem::Method(method) => {
            let method_name = &method.sig.ident;
            let handler_name = format_ident!("handle_{}", method_name);
            let arguments = &method.sig.inputs
                .iter()
                .collect::<Vec<_>>();
            let arguments_names = arguments
                .iter()
                .map(|argument| {
                    if let FnArg::Typed(argument) = argument {
                        if let Pat::Ident(argument_name) = &*argument.pat {
                            return Some(argument_name.ident.clone());
                        }
                    }
                    return None;
                })
                .filter(|argument_name| argument_name.is_some())
                .map(|item| item.unwrap())
                .collect::<Vec<_>>();
            let arguments_reads = &method.sig.inputs
                .iter()
                .map(|item| {
                    if let FnArg::Typed(argument) = item {
                        if let Pat::Ident(argument_name) = &*argument.pat {
                            let argument_type = &*argument.ty;
                            return Some(quote! {
                                let #argument_name: #argument_type = bincode::deserialize_from::<_, #argument_type>(&mut reader).expect(format!("Unable to read {} argument", stringify!(#argument_name)).as_str());
                            })
                        }
                    }
                    None
                })
                .filter(|item| item.is_some())
                .map(|item| item.unwrap())
                .collect::<Vec<_>>();
            quote!(
                fn #handler_name(&mut self, mut reader: &mut dyn std::io::Read, mut writer: &mut dyn std::io::Write) {
                    use bincode;

                    #(#arguments_reads)*
                    let result_value = self.handler.#method_name(#(#arguments_names), *);
                    bincode::serialize_into(&mut writer, &result_value).expect("Unable to write result value");
                    std::io::Write::flush(&mut writer).expect("Unable to flush writer");
                }
            )
        },
        _ => quote!()
    }
}

fn impl_service_wrapper(service_name: &Ident, service_interface: &ItemTrait) -> proc_macro2::TokenStream {
    let wrapper_name = format_ident!("{}Wrapper", service_name.to_string());
    let methods = service_interface.items
        .iter()
        .map(|item| impl_wrapper_method(item));
    let handle_matches = service_interface.items
        .iter()
        .map(|item| {
            if let TraitItem::Method(method) = item {
                let method_name = &method.sig.ident;
                let method_call = format_ident!("handle_{}", method_name);
                return quote! {
                    stringify!(#method_name) => self.#method_call(reader, writer)
                }
            }
            return quote!();
        })
        .collect::<Vec<_>>();
    quote! {
        pub struct #wrapper_name<T: #service_name> {
            handler: T,
        }
        impl<T: #service_name> #wrapper_name<T> {
            pub fn new(handler: T) -> Self {
                Self {
                    handler
                }
            }
            #(#methods)*
        }
        impl<T: #service_name> rpc_lib::Handler for #wrapper_name<T> {
            fn handle(&mut self, mut reader: &mut dyn std::io::Read, writer: &mut dyn std::io::Write) {
                use bincode;

                let method_name = bincode::deserialize_from::<_, String>(&mut reader).expect("Unable to read method name");
                match method_name.as_str() {
                    #(#handle_matches),*,
                    _ => eprintln!("Unknown method {}", method_name)
                };
            }
        }
    }
}

fn impl_service_macro(service_interface: &ItemTrait) -> TokenStream {
    let service_name = &service_interface.ident;
    let service_client = impl_service_client(service_name, service_interface);
    let service_wrapper = impl_service_wrapper(service_name, service_interface);
    let service_impl = quote! {
        #service_interface
        #service_client
        #service_wrapper
    };
    service_impl.into()
}

#[proc_macro_attribute]
pub fn rpc_service(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as ItemTrait);
    impl_service_macro(&item_ast)
}