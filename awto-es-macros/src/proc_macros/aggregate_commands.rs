use heck::{CamelCase, ShoutySnekCase};
use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::spanned::Spanned;

pub struct AggregateCommands {
    command_ident: syn::Ident,
    // error_ty: syn::Type,
    event_ident: syn::Ident,
    ident: syn::Ident,
    input: syn::ItemImpl,
    methods: Vec<Method>,
}

struct Method {
    args: Vec<Arg>,
    docs: Vec<Literal>,
    ident: syn::Ident,
    is_vec: bool,
}

struct Arg {
    ident: syn::Ident,
    ty: syn::Type,
}

impl AggregateCommands {
    fn expand_command_enum(&self) -> syn::Result<TokenStream> {
        let Self {
            command_ident,
            ident,
            methods,
            ..
        } = self;

        let ident = ident.to_string();

        let variants = methods.iter().map(|method| {
            let docs = &method.docs;

            let variant_ident = format_ident!("{}", method.ident.to_string().to_camel_case());
            let variant_ident_upper = method.ident.to_string().TO_SHOUTY_SNEK_CASE();

            let fields = method.args.iter().map(|arg| {
                let ident = &arg.ident;
                let ty = &arg.ty;
                quote!(#ident: #ty)
            });

            quote!(
                #(#[doc = #docs])*
                #[serde(rename = #variant_ident_upper)]
                #variant_ident {
                    #( #fields, )*
                }
            )
        });

        Ok(quote!(
            #[derive(Clone, Debug, PartialEq, ::awto_es::Command, ::awto_es::StreamTopic, ::awto_es::CommandMessage, ::serde::Deserialize, ::serde::Serialize)]
            #[aggregate = #ident]
            pub enum #command_ident {
                #( #variants, )*
            }
        ))
    }

    fn expand_impl_aggregate_command_handler(&self) -> syn::Result<TokenStream> {
        let Self {
            command_ident,
            event_ident,
            ident,
            methods,
            ..
        } = self;

        let matches = methods.iter()
            .map(|method| {
                let method_ident = &method.ident;
                let variant_ident =
                    format_ident!("{}", method.ident.to_string().to_camel_case());

                let fields: Vec<_> = method.args.iter().map(|arg| {
                    &arg.ident
                }).collect();

                if method.is_vec {
                    quote!(
                        #command_ident::#variant_ident { #( #fields, )* } => Ok(self.#method_ident(#( #fields ),*)?.into_iter().map(|event| event.into()).collect())
                    )
                } else {
                    quote!(
                        #command_ident::#variant_ident { #( #fields, )* } => self.#method_ident(#( #fields ),*).map(|event| vec![event.into()])
                    )
                }
            });

        Ok(quote!(
            impl ::awto_es::AggregateCommandHandler for #ident {
                type Command = #command_ident;
                type Event = #event_ident;

                fn execute(&self, command: Self::Command) -> Result<Vec<Self::Event>, ::awto_es::Error> {
                    match command {
                        #( #matches, )*
                    }
                }
            }
        ))
    }
}

impl AggregateCommands {
    pub fn new(input: syn::ItemImpl) -> syn::Result<Self> {
        let ident = match &*input.self_ty {
            syn::Type::Path(type_path) => type_path.path.get_ident().unwrap().clone(),
            _ => {
                return Err(syn::Error::new(
                    input.impl_token.span,
                    "impl must be on a struct",
                ))
            }
        };

        let event_ident = format_ident!("{}Event", ident);
        let command_ident = format_ident!("{}Command", ident);

        let methods = input
            .items
            .clone()
            .into_iter()
            .map(|item| match item {
                syn::ImplItem::Method(method) => Result::<_, syn::Error>::Ok(method),
                _ => Err(syn::Error::new(
                    item.span(),
                    "unexpected item: only methods are allowed in aggregate_events",
                )),
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|method| {
                let docs: Vec<_> = method
                    .attrs
                    .into_iter()
                    .filter_map(|attr| {
                        if !matches!(attr.style, syn::AttrStyle::Outer) {
                            return None;
                        }

                        if attr.path.segments.first()?.ident != "doc" {
                            return None;
                        }

                        attr.tokens.into_iter().nth(1).and_then(|doc| match doc {
                            TokenTree::Literal(lit) => Some(lit),
                            _ => None,
                        })
                    })
                    .collect();

                let mut inputs = method.sig.inputs.into_iter();
                let mut is_vec = false;

                let self_input = inputs.next().ok_or_else(|| {
                    syn::Error::new(method.sig.ident.span(), "method must take &self")
                })?;
                match self_input {
                    syn::FnArg::Receiver(receiver) => {
                        if let Some(mutability) = receiver.mutability {
                            return Err(syn::Error::new(
                                mutability.span(),
                                "self cannot be mut for aggregate commands",
                            ));
                        }
                    }
                    _ => {
                        return Err(syn::Error::new(
                            method.sig.ident.span(),
                            "method must take &mut self",
                        ));
                    }
                }

                let args: Vec<_> = inputs
                    .map(|arg| {
                        let arg = match arg {
                            syn::FnArg::Typed(arg) => arg,
                            _ => unreachable!("methods cannot take self more than once"),
                        };
                        let ident = match &*arg.pat {
                            syn::Pat::Ident(ident_pat) => format_ident!(
                                "{}",
                                ident_pat.ident.to_string().trim_start_matches('_')
                            ),
                            _ => {
                                return Err(syn::Error::new(
                                    arg.span(),
                                    "unsupported argument type",
                                ))
                            }
                        };
                        let ty = *arg.ty;

                        Ok(Arg { ident, ty })
                    })
                    .collect::<Result<_, _>>()?;

                let err_ty = match &method.sig.output {
                    syn::ReturnType::Type(_, ty) => match &**ty {
                        syn::Type::Path(ty_path) => {
                            let mut segments = ty_path.path.segments.iter();
                            segments.next().and_then(|segment| {
                                if segments.next().is_some() {
                                    return None;
                                }
                                if segment.ident != "Result" {
                                    return None;
                                }
                                let arguments = match &segment.arguments {
                                    syn::PathArguments::AngleBracketed(arguments) => arguments,
                                    _ => return None,
                                };
                                let mut args = arguments.args.iter();
                                let first_argument = args.next()?;
                                is_vec = quote!(#first_argument).to_string().starts_with("Vec <");
                                match args.next()? {
                                    syn::GenericArgument::Type(ty) => Some(ty),
                                    _ => None,
                                }
                            })
                        }
                        _ => None,
                    },
                    _ => None,
                };
                match err_ty {
                    Some(err_ty) => {
                        let err_ty_string = quote!(#err_ty).to_string().replace(' ', "");
                        if err_ty_string != "Error"
                            && err_ty_string != "awto_es::Error"
                            && err_ty_string != "::awto_es::Error"
                        {
                            return Err(syn::Error::new(
                                err_ty.span(),
                                format!(
                                    "method must return `Result<Vec<{}>, awto_es::Error>`",
                                    event_ident.to_string(),
                                ),
                            ));
                        }
                    }
                    None => {
                        return Err(syn::Error::new(
                            method.sig.output.span(),
                            format!(
                                "method must return `Result<Vec<{}>, awto_es::Error>`",
                                event_ident.to_string(),
                            ),
                        ))
                    }
                }

                Ok(Method {
                    args,
                    docs,
                    ident: method.sig.ident,
                    is_vec,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(AggregateCommands {
            command_ident,
            // error_ty: error_ty.unwrap_or_else(|| {
            //     syn::Type::Tuple(syn::TypeTuple {
            //         elems: syn::punctuated::Punctuated::new(),
            //         paren_token: syn::token::Paren::default(),
            //     })
            // }),
            event_ident,
            ident,
            input,
            methods,
        })
    }

    pub fn expand(self) -> syn::Result<TokenStream> {
        let input = &self.input;

        let expanded_input = quote!(#input);
        let expanded_events_enum = self.expand_command_enum()?;
        let expanded_impl_aggregate_command = self.expand_impl_aggregate_command_handler()?;

        Ok(TokenStream::from_iter([
            expanded_input,
            expanded_events_enum,
            expanded_impl_aggregate_command,
        ]))
    }
}
