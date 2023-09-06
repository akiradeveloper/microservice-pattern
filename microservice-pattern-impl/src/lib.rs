use proc_macro::TokenStream;

use quote::*;
use syn::*;

#[proc_macro_attribute]
pub fn service(_: TokenStream, item: TokenStream) -> TokenStream {
    let t = syn::parse::<ItemTrait>(item).unwrap();

    let ident = t.ident;
    let ident_svc = format_ident!("{ident}Svc");

    let mut funcs = vec![];
    for item in t.items {
        match item {
            TraitItem::Fn(mut func) => {
                // prepend &self to the argument list.
                func.sig.inputs.insert(0, parse_quote!(&self));
                funcs.push(TraitItem::Fn(func));
            }
            _ => {}
        }
    }

    let code = quote! {
        // automock should precedes async_trait.
        // https://docs.rs/mockall/latest/mockall/#async-traits
        #[mockall::automock]
        #[async_trait::async_trait]
        pub trait #ident: Send + Sync + 'static {
            #(async #funcs)*
        }
        #[derive(Clone, shrinkwraprs::Shrinkwrap)]
        pub struct #ident_svc(std::sync::Arc<dyn #ident>);
        impl #ident_svc {
            pub fn new(svc: impl #ident) -> #ident_svc {
                #ident_svc(std::sync::Arc::new(svc))
            }
        }
    };
    code.into()
}

#[proc_macro_attribute]
pub fn service_impl(_: TokenStream, item: TokenStream) -> TokenStream {
    let t = syn::parse::<ItemImpl>(item).unwrap();
    let code = quote! {
        #[async_trait::async_trait]
        #t
    };
    code.into()
}
