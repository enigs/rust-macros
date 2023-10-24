use proc_macro::TokenStream;
use syn::DeriveInput;

fn impl_set_cipher_trait(ast: DeriveInput) -> TokenStream {
    // Get struct identifier
    let ident = ast.ident;

    // Generate impl
    quote::quote! {
        impl #ident {
            fn encrypt(&mut self) -> &mut Self {
                for string in self.get_ciphers() {
                    *string = match ciphers::encrypt(&string) {
                        Ok(d) => d,
                        _ => string.to_string()
                    }
                }

                self
            }

            fn decrypt(&mut self) -> &mut Self {
                for string in self.get_ciphers() {
                    *string = match ciphers::decrypt(&string) {
                        Ok(d) => d,
                        _ => string.to_string()
                    }
                }

                self
            }
        }
    }
    .into()
}

#[proc_macro_derive(SetCipher)]
pub fn set_cipher_derive_macro(item: TokenStream) -> TokenStream {
    // Parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate
    impl_set_cipher_trait(ast)
}