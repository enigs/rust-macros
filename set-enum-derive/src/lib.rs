use proc_macro::TokenStream;
use syn::DeriveInput;

fn impl_set_enum_trait(ast: DeriveInput) -> TokenStream {
    // Get struct identifier
    let ident = ast.ident;
    let ident_str = ident.to_string();

    // Get field identifiers
    let field_idents: Vec<syn::Ident> = match ast.data {
        syn::Data::Struct(_) => panic!("Structs are not supported by SetEnum."),
        syn::Data::Enum(data) => data.variants.iter().map(|v| match v.fields {
            syn::Fields::Named(_) => panic!("Named fields are not supported by SetEnum."),
            syn::Fields::Unnamed(_) => panic!("Unnamed fields are not supported by SetEnum."),
            syn::Fields::Unit => v.ident.clone()
        }).collect(),
        syn::Data::Union(_) => panic!("Unions are not supported by SetEnum.")
    };

    // let field_idents_str: Vec<String> = field_idents.iter().map(|f| f.to_string()).collect();
    let field_idents_str_lower: Vec<String> = field_idents.iter().map(|f| f.to_string().to_lowercase()).collect();
    let field_idents_str_upper: Vec<String> = field_idents.iter().map(|f| f.to_string().to_uppercase()).collect();

    // Generate impl
    quote::quote! {
        impl SetEnum for #ident {
            fn new<T: ToString>(s: T) -> Self {
                match s.to_string().to_lowercase().as_str() {
                    #(
                        #field_idents_str_lower => Self::#field_idents,
                    )*
                    _ => Self::default()
                }
            }

            fn from_str<T: ToString>(s: T) -> Self {
                match s.to_string().to_lowercase().as_str() {
                    #(
                        #field_idents_str_lower => Self::#field_idents,
                    )*
                    _ => Self::default()
                }
            }

            fn to_str(&self) -> &'static str {
                match *self {
                    #(
                        #ident::#field_idents => #field_idents_str_upper,
                    )*
                }
            }
        }

        impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg> for #ident {
            fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
                match <String as diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg>>::from_sql(bytes)?.to_lowercase().as_str() {
                    #(
                        #field_idents_str_lower => Ok(#ident::#field_idents),
                    )*
                    _ => Err(format!("Unrecognized variant while deserializing {}", stringify!(#ident)).into()),
                }
            }
        }

        impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg> for #ident {
            fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
                let text = match *self {
                    #(
                        #ident::#field_idents => #field_idents_str_upper.to_string(),
                    )*
                };

                diesel::serialize::ToSql::<diesel::sql_types::Text, diesel::pg::Pg>::to_sql(&text, &mut out.reborrow())
            }
        }

        impl From<String> for #ident {
            fn from(module: String) -> Self {
                Self::from_str(module)
            }
        }

        impl From<&String> for #ident {
            fn from(module: &String) -> Self {
                Self::from_str(module)
            }
        }

        impl From<&str> for #ident {
            fn from(module: &str) -> Self {
                Self::from_str(module)
            }
        }

        impl From<Option<String>> for #ident {
            fn from(module: Option<String>) -> Self {
                match module {
                    Some(module) => Self::from(module),
                    None => Self::default()
                }
            }
        }

        impl From<Option<&str>> for #ident {
            fn from(module: Option<&str>) -> Self {
                match module {
                    Some(module) => Self::from(module),
                    None => Self::default()
                }
            }
        }

        impl From<std::borrow::Cow<'_, str>> for #ident {
            fn from(module: std::borrow::Cow<'_, str>) -> Self {
                Self::from_str(module.to_string())
            }
        }

        impl ToString for #ident {
            fn to_string(&self) -> String {
                match *self {
                    #(
                        #ident::#field_idents => #field_idents_str_upper.to_string(),
                    )*
                }
            }
        }

        impl From<#ident> for String {
            fn from(value: #ident) -> Self {
                match value {
                    #(
                        #ident::#field_idents => #field_idents_str_upper.to_string(),
                    )*
                }
            }
        }

        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
            {
                match *self {
                    #(
                        #ident::#field_idents => serializer.serialize_str(#field_idents_str_upper),
                    )*
                    _ => serializer.serialize_str(#ident_str)
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
            {
                match String::deserialize(deserializer)?.to_lowercase().as_str() {
                    #(
                        #field_idents_str_lower => Ok(#ident::#field_idents),
                    )*
                    _ => Ok(#ident::default())
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(SetEnum)]
pub fn set_enum_derive_macro(item: TokenStream) -> TokenStream {
    // Parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate
    impl_set_enum_trait(ast)
}