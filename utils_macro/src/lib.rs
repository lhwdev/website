use darling::FromMeta;
use quote::quote;
use proc_macro2::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Type, ExprPath, PathArguments, Path};

// #[proc_macro_derive(ThinWrapper)]
// pub fn thin_wrapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let item = if let Data::Struct(inner) = &input.data {
//         inner
//     } else {
//         unreachable!()
//     };

//     let ident = &input.ident;
//     let inner_type = &item.fields.iter().next().expect("No field").ty.to_token_stream();

//     let after = quote! {
//         impl Deref for #ident {
//             type Target = #inner_type;

//             fn deref(&self) -> &Self::Target {
//                 &self.0
//             }
//         }

//         impl DerefMut for #ident {
//             fn deref_mut(&mut self) -> &mut Self::Target {
//                 &mut self.0
//             }
//         }
//     };

//     after.into()
// }

#[derive(Default, FromMeta)]
#[darling(default)]
struct ThinWrapper {
    constructor: bool
}


fn is_ident(path: &Path, text: &str) -> bool {
    match path.get_ident() {
        Some(ident) => ident.to_string() == text,
        None => false
    }
}


#[proc_macro_derive(ThinWrapperSerde, attributes(thin_wrapper))]
pub fn thin_wrapper_serde(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    transform_thin_wrapper_serde(input).into()
}

fn transform_thin_wrapper_serde(input: DeriveInput) -> TokenStream {
    let item = if let Data::Struct(inner) = &input.data {
        inner
    } else {
        unreachable!()
    };

    let ident = &input.ident;
    let inner_type = &item.fields.iter().next().expect("No field").ty;

    let raw_attr = input.attrs.iter().find(|attr| is_ident(&attr.path, "thin_wrapper"));
    let attr: Option<ThinWrapper> = if let Some(raw_attr) = raw_attr {
        let meta = raw_attr.parse_meta().unwrap();
        Some(ThinWrapper::from_meta(&meta).expect("Wrong metadata content of thin_wrapper"))
    } else {
        None
    };

    // Type is inferred here / Type<Argument>::deserialize is wrong grammer
    let inner_type_mapped = if let Type::Path(path) = inner_type {
        let mut path = path.clone();
        let segments = &mut path.path.segments;
        for segment in segments {
            if let PathArguments::AngleBracketed(_) = &segment.arguments {
                segment.arguments = PathArguments::None;
            }
        }
        
        ExprPath { attrs: vec![], qself: path.qself, path: path.path }
    } else {
        unreachable!();
    };

    let mut after = quote! {
        impl std::ops::Deref for #ident {
            type Target = #inner_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for #ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer {
                self.0.serialize(serializer)
            }
        }
                    
        impl <'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de> {
                Ok(#ident(#inner_type_mapped::deserialize(deserializer)?))
            }
        }
    };

    if let Some(attr) = attr {
        if attr.constructor {
            after.extend(quote! {
                impl #ident {
                    pub fn new(inner: #inner_type) -> Self {
                        Self(inner)
                    }
                }
            });
        }
    }

    after
}
#[cfg(test)]
fn transform_thin_wrapper_serde_test(input: TokenStream) -> TokenStream {
    transform_thin_wrapper_serde(syn::parse2(input).unwrap())
}

#[cfg(test)]
mod test {
    use quote::quote;

    #[test]
    fn test() {
        let code = quote! {
            #[derive(ThinWrapperSerde, PartialEq)]
            pub struct Privileges(Vec<Privilege>);
        };

        let result = crate::transform_thin_wrapper_serde_test(code);
        println!("{}", result);
    }
}
