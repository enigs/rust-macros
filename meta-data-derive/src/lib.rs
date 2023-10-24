use std::collections::HashMap;
use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataStructAttributes {
    author: String,
    #[deluxe(default = 0)]
    serial_version: usize,
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataFieldAttributes {
    author: String,
}

fn extract_meta_data_field_attrs(
    ast: &mut DeriveInput
) -> deluxe::Result<HashMap<String, MetaDataFieldAttributes>> {
    let mut field_attrs: HashMap<String, MetaDataFieldAttributes> = HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let attrs: MetaDataFieldAttributes = deluxe::extract_attributes(field)?;
            field_attrs.insert(field_name, attrs);
        }
    }

    Ok(field_attrs)
}

fn meta_data_derive_macro2(
    item: proc_macro2::TokenStream
) -> deluxe::Result<proc_macro2::TokenStream> {
    // Parse
    let mut ast: DeriveInput = syn::parse2(item)?;

    // Extract struct attributes
    let MetaDataStructAttributes {
        author,
        serial_version
    } = deluxe::extract_attributes(&mut ast)?;

    // Extract field attributes
    let field_attrs: HashMap<String, MetaDataFieldAttributes> = extract_meta_data_field_attrs(&mut ast)?;

    let (field_name, field_authors): (Vec<String>, Vec<String>) = field_attrs
        .into_iter()
        .map(|(field, attrs)|  (field, attrs.author))
        .unzip();

    // Define impl variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // Generate
    Ok(quote::quote! {
        impl #impl_generics MetaData for #ident #type_generics #where_clause {
            fn author(&self) -> &'static str {
                #author
            }

            fn serial_version(&self) -> usize {
                #serial_version
            }

            fn field_authors(&self) -> std::collections::HashMap<&'static str, &'static str> {
                let fields = [#(#field_name),*];
                let authors = [#(#field_authors),*];

                let map: std::collections::HashMap<&'static str, &'static str> = fields
                    .iter()
                    .zip(authors.iter())
                    .map(|(&field, &author)| (field, author))
                    .collect();

                map
            }
        }
    })
}

#[proc_macro_derive(MetaData, attributes(metadata))]
pub fn meta_data_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    meta_data_derive_macro2(item.into()).unwrap().into()
}