use proc_macro::TokenStream;
use syn::{DeriveInput, Ident};

fn impl_reflective_trait(ast: DeriveInput) -> TokenStream {
    // Get struct identifier
    let ident = ast.ident;
    let ident_str = ident.to_string();

    // Get field identifiers
    let field_idents: Vec<Ident> = match ast.data {
        syn::Data::Struct(data) => data.fields.into_iter().filter_map(|f| f.ident).collect(),
        syn::Data::Enum(_) => panic!("Enums are not supported by reflective."),
        syn::Data::Union(_) => panic!("Unions are not supported by reflective.")
    };

    let field_idents_str: Vec<String> = field_idents.iter().map(|f| f.to_string()).collect();

    // Generate impl
    quote::quote! {
        impl Reflective for #ident {
            fn name(&self) -> &'static str {
                #ident_str
            }

            fn fields(&self) -> Vec<&'static str> {
                vec![#(#field_idents_str),*]
            }
        }
    }
    .into()
}

#[proc_macro_derive(Reflective)]
pub fn reflective_derive_macro(item: TokenStream) -> TokenStream {
    // Parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate
    impl_reflective_trait(ast)
}