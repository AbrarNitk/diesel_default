#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate darling;
extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::{Span};
use syn::{DeriveInput, ItemStruct, AttributeArgs};
use darling::{FromMeta, FromDeriveInput};

#[derive(Default, FromMeta, Debug)]
struct SaveArgs {
    output_type: String,
    dsl_name: String,
}

#[proc_macro_attribute]
pub fn diesel_default(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(metadata as AttributeArgs);

    let args = match SaveArgs::from_list(&attr_args){
        Ok(v) => v,
        Err(e) => { return e.write_errors().into();}
    };

    let input_clone = input.clone();
    let derive_input = parse_macro_input!(input as DeriveInput);
    let struct_item: ItemStruct = parse_macro_input!(input_clone as ItemStruct);
    let ident = struct_item.ident;
    let dsl = syn::Ident::new(&args.dsl_name, Span::call_site());
    let output = syn::Ident::new(&args.output_type , Span::call_site());

    quote!(

        #derive_input

        impl #ident {
            pub fn save(&self, conn: &PgConnection) -> Result<#output> {
                diesel::insert_into(dsl :: #dsl)
                .values(self)
                .get_result(conn)
                .map_err( |e| e.into())
            }

            pub fn save_vec(values: &[Self], conn: &PgConnection) -> Result<usize> {
                diesel::insert_into(dsl :: #dsl)
                .values(values)
                .execute(conn)
                .map_err( |e| e.into())
            }
        }
    ).into()
}


#[derive(Default, FromMeta, Debug)]
#[darling(default)]
struct Lorem {
    ipsum: bool,
    dolor: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(my_crate), forward_attrs(allow, doc, cfg))]
struct MyTraitOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    lorem: Lorem,
}

#[proc_macro_derive(MyTrait, attributes(my_crate))]
pub fn check(input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(input as DeriveInput);

    let attrs = match MyTraitOpts::from_derive_input(&attr_args) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    println!("Lorem {:?}", attrs.lorem);
    quote!().into()
}