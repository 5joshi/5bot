use super::Base;

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use std::convert::TryFrom;
use syn::{parse_quote, Error as SynError, Ident, Lit, LitStr, Meta, Result as SynResult};

pub struct Command {
    name: Ident,
    cmd_name: Lit,
    cmd_description: Lit,
    run_name: Ident,
    args_name: Option<Ident>,
    options_name: Option<Ident>,
}

impl TryFrom<Base> for Command {
    type Error = SynError;

    fn try_from(base: Base) -> SynResult<Self> {
        let mut cmd_name = None;
        let mut cmd_description = None;
        let mut run_name = None;
        let mut args_name = None;
        let mut options_name = None;

        for attr in base.attributes {
            match attr.parse_meta()? {
                Meta::Path(m) => {
                    let message = r#"attribute must be of the form `#[... = "..."]`"#;
                    let span = m.get_ident().map_or_else(Span::call_site, Ident::span);

                    return Err(SynError::new(span, message));
                }
                Meta::List(m) => {
                    let message = r#"attribute must be of the form `#[... = "..."]`"#;
                    let span = m.path.get_ident().map_or_else(Span::call_site, Ident::span);

                    return Err(SynError::new(span, message));
                }
                Meta::NameValue(m) => {
                    if m.path == parse_quote!(name) {
                        cmd_name = Some(m.lit);
                    } else if m.path == parse_quote!(run) {
                        run_name = Some(m.lit);
                    } else if m.path == parse_quote!(args) {
                        match m.lit {
                            Lit::Str(lit) => args_name = Some(Ident::new(&lit.value(), lit.span())),
                            other => {
                                let message =
                                    r#"`args` attribute must be of the form `#[args = "..."]`"#;

                                return Err(SynError::new(other.span(), message));
                            }
                        }
                    } else if m.path == parse_quote!(description) {
                        cmd_description = Some(m.lit);
                    } else if m.path == parse_quote!(options) {
                        match m.lit {
                            Lit::Str(lit) => {
                                options_name = Some(Ident::new(&lit.value(), lit.span()))
                            }
                            other => {
                                let message = r#"`options` attribute must be of the form `#[options = "..."]`"#;

                                return Err(SynError::new(other.span(), message));
                            }
                        }
                    } else {
                        let message = "invalid attribute key, expected either `args`, `description`, `name`, `options`, or `run`";
                        let span = m.path.get_ident().map_or_else(Span::call_site, Ident::span);

                        return Err(SynError::new(span, message));
                    }
                }
            }
        }

        let cmd_description = cmd_description.ok_or_else(|| {
            let message = r#"attribute `#[description = "..."` must be specified"#;

            SynError::new(Span::call_site(), message)
        })?;

        let cmd_name = match cmd_name {
            Some(name) => name,
            None => {
                let name = base.name.to_string().to_ascii_lowercase();

                Lit::Str(LitStr::new(&name, base.name.span()))
            }
        };

        let run_name: Ident = match run_name {
            Some(Lit::Str(lit)) => Ident::new(&lit.value(), lit.span()),
            Some(other) => return Err(SynError::new(other.span(), "expected string literal")),
            None => {
                let name = base.name.to_string().to_ascii_lowercase();

                Ident::new(&name, base.name.span())
            }
        };

        let command = Self {
            name: base.name,
            cmd_name,
            cmd_description,
            run_name,
            args_name,
            options_name,
        };

        Ok(command)
    }
}

impl ToTokens for Command {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            name,
            cmd_name,
            cmd_description,
            run_name,
            args_name,
            options_name,
        } = self;

        let fut_name = format_ident!("{}Future", name);

        let base_stream = quote! {
            pub struct #name;

            pub struct #fut_name<'f> {
                fut: ::futures::future::BoxFuture<'f, crate::BotResult<()>>,
            }

            impl<'f> ::std::future::Future for #fut_name<'f> {
                type Output = crate::BotResult<()>;

                fn poll(mut self: ::std::pin::Pin<&mut Self>, cx: &mut ::std::task::Context<'_>) -> ::std::task::Poll<Self::Output> {
                    ::std::pin::Pin::new(&mut self.fut).poll(cx)
                }
            }

            impl #name {
                pub const NAME: &'static str = #cmd_name;
            }
        };

        tokens.extend(base_stream);

        let define_stream = if let Some(options_name) = options_name {
            quote! {
                impl #name {
                    pub fn define() -> ::twilight_model::application::command::Command {
                        ::twilight_model::application::command::Command {
                            application_id: None,
                            guild_id: None,
                            name: #cmd_name.to_owned(),
                            default_permission: None,
                            description: #cmd_description.to_owned(),
                            id: None,
                            kind: ::twilight_model::application::command::CommandType::ChatInput,
                            options: #options_name(),
                            // version: ::twilight_model::id::CommandVersionId::new(1).unwrap()
                        }
                    }
                }
            }
        } else {
            quote! {
                impl #name {
                    pub fn define() -> ::twilight_model::application::command::Command {
                        ::twilight_model::application::command::Command {
                            application_id: None,
                            guild_id: None,
                            name: #cmd_name.to_owned(),
                            default_permission: None,
                            description: #cmd_description.to_owned(),
                            id: None,
                            kind: ::twilight_model::application::command::CommandType::ChatInput,
                            options: Vec::new(),
                            // version: ::twilight_model::id::CommandVersionId::new(1).unwrap()
                        }
                    }
                }
            }
        };

        tokens.extend(define_stream);

        let run_stream = if let Some(args_name) = args_name {
            quote! {
                impl #name {
                    pub fn run(ctx: ::std::sync::Arc<crate::Context>, mut command: ::twilight_model::application::interaction::ApplicationCommand) -> #fut_name<'static> {
                        use futures::TryFutureExt;

                        let data = ::std::mem::take(&mut command.data);

                        let fut = #args_name::parse_options(Arc::clone(&ctx), data)
                            .and_then(|args| #run_name(ctx, command, args))
                            .map_err(Box::new)
                            .map_err(|src| crate::Error::Command {
                                name: #cmd_name,
                                src,
                            });

                        #fut_name { fut: Box::pin(fut) }
                    }
                }
            }
        } else {
            quote! {
                impl #name {
                    pub fn run(ctx: ::std::sync::Arc<crate::Context>, command: ::twilight_model::application::interaction::ApplicationCommand) -> #fut_name<'static> {
                        use futures::TryFutureExt;

                        let fut = #run_name(ctx, command)
                            .map_err(Box::new)
                            .map_err(|src| crate::Error::Command {
                                name: #cmd_name,
                                src,
                            });

                        #fut_name { fut: Box::pin(fut) }
                    }
                }
            }
        };

        tokens.extend(run_stream);
    }
}
