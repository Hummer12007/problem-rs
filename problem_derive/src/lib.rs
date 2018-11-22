#![feature(extern_crate_item_prelude)]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;
extern crate problem;

extern crate darling;

use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{Attribute, DataEnum, DeriveInput, Meta, Variant};

#[derive(Debug, FromMeta)]
struct ProblemMeta {
    title: String,
    #[darling(default)]
    status: Option<u16>,
    #[darling(default)]
    type_instance: Option<String>,
    #[darling(default)]
    detail: Option<String>,
    #[darling(default)]
    instance: Option<String>,
}

fn attr_name(attr: &Attribute) -> Option<Ident> {
    attr.interpret_meta().map(|v| v.name())
}

fn opt_literal<T: ToTokens>(name:TokenStream, val: Option<T>, postfix: TokenStream) -> Option<TokenStream> {
    val.map(|v| quote!{.#name(Some(#v#postfix))})
}

fn implement(self_name: &Ident, variant: &Variant) -> TokenStream {
    let variant_name = variant.clone().ident;

    let attr = variant
        .attrs
        .iter()
        .filter(|v| attr_name(v).expect("attribute") == "problem")
        .last();

    let q = variant_name.to_string();

    match attr.and_then(Attribute::interpret_meta) {
        Some(Meta::Word(_)) | None => quote! {
            (#self_name::#variant_name) =>
                Problem::new(#q),
        },
        Some(Meta::List(ml)) => {
            let items = ml.nested.iter().map(|x| x.clone()).collect::<Vec<_>>();
            let m = ProblemMeta::from_list(items.as_slice()).unwrap();
            let ProblemMeta {title, status, type_instance, detail, instance} = m;
            let status_token = opt_literal(quote!{status}, status, quote!{});
            let type_instance_token = opt_literal(quote!{type_url}, type_instance, quote!{.to_string()});
            let detail_token = opt_literal(quote!{detail}, detail, quote!{.to_string()});
            let instance_token = opt_literal(quote!{instance}, instance, quote!{.to_string()});
            quote!{
                (#self_name::#variant_name) =>
                    ProblemBuilder::default()
                        .title(#title.to_string())
                        #status_token
                        #type_instance_token
                        #detail_token
                        #instance_token
                        .build()
                        .unwrap(),
            }
        }
        _ => panic!("Unexpected attribute parameters."),
    }
}

fn produce(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();


    if let syn::Data::Enum(DataEnum { ref variants, .. }) = ast.data {
        let generated = variants.iter().map(|v| implement(name, v)).collect::<Vec<_>>();

        quote! {
            impl #impl_generics ToProblem for #name #ty_generics #where_clause {
                fn to_problem(&self) -> Problem {
                    match self {
                        #(#generated)*
                    }
                }
            }
        }
    } else {
        panic!("#[derive(ToProblem)] is only defined for enums, not for structs!");
    }
}

#[proc_macro_derive(ToProblem, attributes(problem))]
pub fn getters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).expect("Parsing failed");
    let gen = produce(&ast);
    gen.into()
}

