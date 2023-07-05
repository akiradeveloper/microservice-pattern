use proc_macro::TokenStream;

use proc_macro2::*;
use quote::*;
use syn::*;

#[proc_macro_attribute]
pub fn def_service(args: TokenStream, item: TokenStream) -> TokenStream {
    let t = syn::parse::<ItemTrait>(item).unwrap();

    let ident = t.ident;
    let ident_client = format_ident!("{ident}Client");
    let ident_server = format_ident!("{ident}Server");

    let mut methods = vec![];
    for item in t.items {
        match item {
            TraitItem::Method(mut method) => {
                method.sig.inputs.insert(0, parse_quote!(&self));
                methods.push(TraitItem::Method(method));
            }
            _ => panic!("only methods are supported"),
        }
    }

    let code = quote! {
        #[async_trait::async_trait]
        #[cfg_attr(test, mockall::automock)]
        pub trait #ident {
            #(#methods)*
        }
        #[derive(Clone, shrinkwraprs::Shrinkwrap)]
        pub struct #ident_client(std::sync::Arc<dyn #ident>);
        pub struct #ident_server;
        impl #ident_server {
            pub fn new(svc: impl #ident + 'static) -> #ident_client {
                #ident_client(std::sync::Arc::new(svc))
            }
        }
    };
    code.into()
}

#[proc_macro_attribute]
pub fn impl_service(args: TokenStream, item: TokenStream) -> TokenStream {
    let t = syn::parse::<ItemImpl>(item).unwrap();
    let code = quote! {
        #[async_trait::async_trait]
        #t
    };
    code.into()
}
