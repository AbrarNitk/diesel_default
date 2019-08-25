#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate darling;
extern crate proc_macro;
extern crate proc_macro2;

use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, ItemStruct};

#[derive(Default, FromMeta, Debug)]
#[darling(default)]
struct SaveArgs {
    output_type: Option<String>,
    dsl_name: String,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(save_opts), forward_attrs(allow, doc, cfg))]
struct SaveOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    opts: SaveArgs,
}

#[proc_macro_derive(DSave, attributes(save_opts))]
pub fn diesel_save(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let derive_input = parse_macro_input!(input as DeriveInput);

    let attrs = match SaveOpts::from_derive_input(&derive_input) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let struct_item: ItemStruct = parse_macro_input!(input_clone);
    let ident = struct_item.ident;
    let output_type = syn::Ident::new(&attrs.opts.output_type.unwrap(), Span::call_site());
    let dsl = syn::Ident::new(&attrs.opts.dsl_name, Span::call_site());

    quote!(
        impl DSave for #ident {
            type Output = #output_type;
            fn save(&self, conn: &PgConnection) -> Result<Self::Output>{
                diesel::insert_into(dsl :: #dsl)
                .values(self)
                .get_result(conn)
                .map_err( |e| e.into())
            }

            fn save_vec(values: &[Self], conn: &PgConnection) -> Result<usize>
            where
                Self: std::marker::Sized {
                diesel::insert_into(dsl :: #dsl)
                .values(values)
                .execute(conn)
                .map_err( |e| e.into())
            }
        }
    )
    .into()
}


#[derive(Default, FromMeta, Debug)]
#[darling(default)]
struct EncDecArgs {
    sub_key: String,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(encdec_opts), forward_attrs(allow, doc, cfg))]
struct EncDecOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    opts: EncDecArgs,
}

#[proc_macro_derive(Encrypted, attributes(encdec_opts))]
pub fn encyption(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let attrs = match EncDecOpts::from_derive_input(&derive_input) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors().into();
        }
    };
    let sub_key = attrs.opts.sub_key;
    let ident = derive_input.ident;
    quote!(

        impl Encrypted for #ident {
            fn ekey(&self) -> Result<String> {
                encode_ekey_util(self.id, #sub_key)
            }
        }

    ).into()
}

#[proc_macro_derive(Decrypted, attributes(encdec_opts))]
pub fn decyption(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let attrs = match EncDecOpts::from_derive_input(&derive_input) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors().into();
        }
    };
    let sub_key = attrs.opts.sub_key;
    let ident = derive_input.ident;
    quote!(
        impl Decrypted for #ident {
            fn dkey(&self, ekey: &str) -> Result<u64> {
                decode_ekey_util(ekey, #sub_key)
            }
        }

    ).into()
}