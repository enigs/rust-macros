use proc_macro::TokenStream;
use syn::DeriveInput;

fn impl_as_jsonb_trait(ast: DeriveInput) -> TokenStream {
    // Get struct identifier
    let ident = ast.ident;

    if let syn::Data::Struct(_) = ast.data {
        // Generate impl
        return quote::quote! {
            impl diesel::deserialize::FromSql<diesel::sql_types::Jsonb, diesel::pg::Pg> for #ident  {
                fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
                    Ok(serde_json::from_value(<serde_json::Value as diesel::deserialize::FromSql<diesel::sql_types::Jsonb, diesel::pg::Pg>>::from_sql(bytes)?)?)
                }
            }

            impl diesel::serialize::ToSql<diesel::sql_types::Jsonb, diesel::pg::Pg> for #ident  {
                fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
                    diesel::serialize::ToSql::<diesel::sql_types::Jsonb, diesel::pg::Pg>::to_sql(&serde_json::to_value(self)?, &mut out.reborrow())
                }
            }
        }
        .into();
    }

    panic!("Only structs are supported by AsJsonb.");
}

#[proc_macro_derive(AsJsonb)]
pub fn as_jsonb_derive_macro(item: TokenStream) -> TokenStream {
    // Parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate
    impl_as_jsonb_trait(ast)
}