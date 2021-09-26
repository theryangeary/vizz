use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Data;
use syn::Fields;

#[proc_macro_derive(Visualize)]
pub fn visualize_derive(input: TokenStream) -> TokenStream {
    // Construct a Rust code ast we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_visualize(&ast)
}

fn impl_visualize(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let data_fn_impl = match &ast.data {
        Data::Enum(enum_decl) => {
            // turn enum variant names into strings
            let mut arms = Vec::new();

            for variant in &enum_decl.variants {
                let ident = &variant.ident;
                let output = format!("{}", ident);

                let params = match variant.fields {
                    Fields::Unit => quote! {},
                    Fields::Unnamed(..) => quote! { (..) },
                    Fields::Named(..) => quote! { {..} },
                };

                arms.push(quote! { #name::#ident #params => ::std::string::String::from(#output) });
            }

            // return stringified enum variant name as data
            quote! {
                fn data(&self) -> Option<Value> {
                    Some(Value::Owned(match self {
                        #(#arms),*
                    }))
                }
            }
        }
        _ => quote! {},
    };

    let associated_data_fn_impl = {
        let body = match &ast.data {
            Data::Struct(struct_decl) => {
                // turn member names into DataDescriptions with labels
                let mut members = Vec::new();

                for field in &struct_decl.fields {
                    let ident = &field
                        .ident
                        .as_ref()
                        .expect("named fields should have idents");

                    let label = format!("{}", ident);
                    members.push(quote! { DataDescription::from(&self.#ident).with_label(#label) });
                }

                // return vec of associated data
                quote! { Some(vec![ #(#members),* ]) }
            }
            Data::Enum(enum_decl) => {
                // turn enum fields into DataDescriptions
                let mut arms = Vec::new();

                for variant in &enum_decl.variants {
                    let ident = &variant.ident;
                    let mut members = Vec::new();

                    let params =
                        match &variant.fields {
                            Fields::Named(fields) => {
                                let mut param_names = Vec::new();
                                for field in &fields.named {
                                    let ident = &field
                                        .ident
                                        .as_ref()
                                        .expect("named fields should have idents");

                                    let label = format!("{}", ident);
                                    members.push(
                                        quote! { DataDescription::from(#ident).with_label(#label) },
                                    );
                                    param_names.push(quote! { #ident });
                                }
                                quote! { { #(#param_names),* } }
                            }
                            Fields::Unnamed(fields) => {
                                let mut param_names = Vec::new();
                                &fields.unnamed.iter().enumerate().for_each(
                                    |(field_num, field)| {
                                        let ident = &field.ident.clone().unwrap_or(format_ident!(
                                            "__VISUALIZE_PARAM_{}",
                                            field_num
                                        ));
                                        members.push(quote! { DataDescription::from(#ident) });
                                        param_names.push(quote! { #ident });
                                    },
                                );
                                quote! { ( #(#param_names),* ) }
                            }
                            Fields::Unit => quote! {},
                        };

                    let match_result = if members.is_empty() {
                        quote! { None }
                    } else {
                        quote! { Some(vec![ #(#members),* ]) }
                    };

                    arms.push(quote! { #name::#ident #params => #match_result });
                }

                quote! { match self { #(#arms),* } }
            }
            _ => panic!("Unions not supported at this time"),
        };

        quote! {
            fn associated_data(&self) -> ::std::option::Option<std::vec::Vec<DataDescription>> {
                #body
            }
        }
    };

    let gen = quote! {
        impl Visualize for #name {
            #data_fn_impl
            #associated_data_fn_impl
        }
    };

    gen.into()
}