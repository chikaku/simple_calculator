use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(TokenWord, attributes(char))]
pub fn token_word(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    match input.data {
        Data::Enum(data) => {
            let variant_chars = data
                .variants
                .iter()
                .map(|var| {
                    (
                        var.ident.to_string(),
                        get_attr(var, "char".to_string())[2..3].to_string(),
                    )
                })
                .collect::<Vec<_>>();

            let char_to_variants = variant_chars
                .iter()
                .map(|(variant, token_char)| {
                    format!("'{}' => Self::{},", token_char, variant)
                        .parse()
                        .unwrap()
                })
                .collect::<Vec<proc_macro2::TokenStream>>();

            let variant_to_char = variant_chars
                .iter()
                .map(|(variant, token_char)| {
                    format!("Self::{} => write!(f, \"{}\"),", variant, token_char)
                        .parse()
                        .unwrap()
                })
                .collect::<Vec<proc_macro2::TokenStream>>();

            (quote! {
                impl #enum_name {
                    pub fn from_char(c: char) -> Self {
                        match c {
                            #(#char_to_variants)*
                            _ => panic!("unexpected character"),
                        }
                    }
                }

                impl Debug for #enum_name {
                    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#variant_to_char)*
                        }
                    }
                }
            })
            .into()
        }
        _ => panic!("{} is not enum type", enum_name),
    }
}

fn get_attr(var: &syn::Variant, name: String) -> String {
    var.attrs
        .iter()
        .find(|a| match a.path.get_ident() {
            Some(id) => id == &name,
            None => false,
        })
        .expect(format!("enum variant {} has no char attribute", &var.ident).as_str())
        .tokens
        .to_string()
}
