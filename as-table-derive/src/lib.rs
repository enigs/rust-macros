use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(table_name))]
struct TableNameStructAttributes(String);

#[derive(Clone, deluxe::ExtractAttributes)]
#[deluxe(attributes(column_type))]
struct ColumnTypeAttributes(syn::Type);

#[derive(Clone, deluxe::ExtractAttributes)]
#[deluxe(attributes(column_from))]
struct ColumnFromAttributes(syn::ExprPath);

#[derive(Clone, deluxe::ExtractAttributes)]
#[deluxe(attributes(column_to))]
struct ColumnToAttributes(syn::ExprPath);

fn extract_column_type_field_attrs(
    ast: &mut DeriveInput
) -> deluxe::Result<std::collections::HashMap<syn::Ident, ColumnTypeAttributes>> {
    let mut field_attrs: std::collections::HashMap<
        syn::Ident,
        ColumnTypeAttributes
    > = std::collections::HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            if let Ok(attrs) = deluxe::extract_attributes(field) {
                field_attrs.insert(field.ident.as_ref().unwrap().clone(), attrs);
            }
        }
    }

    Ok(field_attrs)
}

fn extract_column_from_field_attrs(
    ast: &mut DeriveInput
) -> deluxe::Result<std::collections::HashMap<syn::Ident, ColumnFromAttributes>> {
    let mut field_attrs: std::collections::HashMap<
        syn::Ident,
        ColumnFromAttributes
    > = std::collections::HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            if let Ok(attrs) = deluxe::extract_attributes(field) {
                field_attrs.insert(field.ident.as_ref().unwrap().clone(), attrs);
            }
        }
    }

    Ok(field_attrs)
}

fn extract_column_to_field_attrs(
    ast: &mut DeriveInput
) -> deluxe::Result<std::collections::HashMap<syn::Ident, ColumnToAttributes>> {
    let mut field_attrs: std::collections::HashMap<
        syn::Ident,
        ColumnToAttributes
    > = std::collections::HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            if let Ok(attrs) = deluxe::extract_attributes(field) {
                field_attrs.insert(field.ident.as_ref().unwrap().clone(), attrs);
            }
        }
    }

    Ok(field_attrs)
}

fn as_table_derive_macro2(
    item: proc_macro2::TokenStream
) -> deluxe::Result<proc_macro2::TokenStream> {
    // Parse
    let mut ast: DeriveInput = syn::parse2(item)?;

    // Extract type field attributes
    let field_attrs: std::collections::HashMap<
        syn::Ident,
        ColumnTypeAttributes
    > = extract_column_type_field_attrs(&mut ast)?;

    let (column_name, column_type): (Vec<syn::Ident>, Vec<syn::Type>) = field_attrs
        .into_iter()
        .map(|(field, attrs)|  (field, attrs.0))
        .unzip();

    // Extract conversion from field attributes
    let field_attrs: std::collections::HashMap<
        syn::Ident,
        ColumnFromAttributes
    > = extract_column_from_field_attrs(&mut ast)?;

    let (_, column_from): (Vec<syn::Ident>, Vec<syn::ExprPath>) = field_attrs
        .into_iter()
        .map(|(field, attrs)|  (field, attrs.0))
        .unzip();

    // Extract conversion to field attributes
    let field_attrs: std::collections::HashMap<
        syn::Ident,
        ColumnToAttributes
    > = extract_column_to_field_attrs(&mut ast)?;

    let (_, column_to): (Vec<syn::Ident>, Vec<syn::ExprPath>) = field_attrs
        .into_iter()
        .map(|(field, attrs)|  (field, attrs.0))
        .unzip();


    // Extract struct attributes
    let table_name: TableNameStructAttributes = deluxe::extract_attributes(&mut ast)?;
    let table_name = syn::Ident::new(&table_name.0, proc_macro2::Span::call_site());

    // Define impl variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // Generate
    let generated = quote::quote! {
        impl #impl_generics AsTable for #ident #type_generics #where_clause {}

        impl From<#ident> for #table_name {
            fn from(value: #ident) -> Self {
                let mut data = Self::default();

                #(
                    data.#column_name = #column_from(value.#column_name);
                )*

                data
            }
        }

        impl From<#table_name> for #ident {
            fn from(value: #table_name) -> Self {
                let mut data = Self::default();

                #(
                    data.#column_name = #column_to(value.#column_name);
                )*

                data
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct #table_name {
            #(
                #[serde(skip_serializing_if = "Option::is_none")]
                pub #column_name: Option<#column_type>
            ),*
        }

        impl #table_name {
            pub fn to<T: From<Self>>(&self) -> T {
                T::from(self.clone())
            }
        }
    };

    Ok(generated)
}

#[proc_macro_derive(AsTable, attributes(table_name, column_type, column_from, column_to))]
pub fn as_table_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    as_table_derive_macro2(item.into()).unwrap().into()
}