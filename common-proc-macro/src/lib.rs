use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TokenLiteral)]
pub fn process_token_literal_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_token_literal_derive(&ast)
}

fn impl_token_literal_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    match data {
        syn::Data::Enum(data_enum) => {
            let variant_patterns = data_enum.variants.iter().map(|v|{
                let ident = &v.ident;
                quote! {
                    #name::#ident
                }
            });
            let variant_names = data_enum.variants.iter().map(|v| {
                v.ident.to_string()
            });
            let gen = quote! {
                impl TokenLiteral for #name {
                    fn literal(&self) -> &'static str {
                        match self {
                            #(#variant_patterns => #variant_names,)*
                        }
                    }
                }
            };
            gen.into()
        }
        _ => {
            let gen = quote! {
                impl TokenLiteral for #name {
                    fn literal(&self) -> &'static str {
                        stringify!(#name)
                    }
                }
            };
            gen.into()
        }
    }
}
