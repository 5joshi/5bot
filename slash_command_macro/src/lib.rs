mod command;

use command::Command;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::convert::TryFrom;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, Error as SynError, Ident, Result as SynResult, Token, Visibility,
};

/// Writes the boilerplate code required for slash commands.
///
/// The following additional attributes can be specified:
/// - `#[args = "..."]` to specify the type for arguments.
/// The type must implement the function `async fn parse_options(Arc<Context>, CommandData) -> BotResult<Self>`.
/// - `#[description = "..."]` must be specified to define the command's description.
/// - `#[name = "..."]` for the command name. Defaults to the lowercase struct name.
/// - `#[options = "..."]` for the function name that returns the command options as `Vec<CommandOption>`.
/// If none is specified the defined command won't have options.
/// - `#[run = "..."` for the function name that runs the command. Defaults to the lowercase struct name.
///
/// For a given command struct `C` this macro enables:
/// - `C::NAME -> &'static str` for the command name
/// - `C::define() -> Command` as function that returns the twilight command
/// - `async C::run(Arc<Context>, ApplicationCommand) -> BotResult<()>` as function that runs the command.
/// If the `args` attribute was specified, there will be a third parameter of the type specified in the attribute.
///
/// ## Example: Ping
///
/// ```ignore
/// #[command]
/// #[description = "Let's play table tennis"]
/// pub struct Ping;
///
/// async fn ping(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
///     println!("ping");
///
///     Ok(())
/// }
/// ```
///
/// ## Example: Roll
///
/// ```ignore
/// #[command]
/// #[args = "RollArgs"]
/// #[description = "Roll a random number"]
/// #[name = "roll_command"]
/// #[options = "roll_options"]
/// #[run = "my_roll_fn"]
/// pub struct Roll;
///
/// struct RollArgs { limit: u64 }
///
/// impl RollArgs {
///     async fn parse_options(ctx: Arc<Context>, data: CommandData) -> BotResult<Self> {
///         Ok(Self { limit: 100 })
///     }
/// }
///
/// async fn my_roll_fn(ctx: Arc<Context>, command: ApplicationCommand, args: RollArgs) -> BotResult<()> {
///     println!("roll up to {}", args.limit);
///
///     Ok(())
/// }
///
/// fn roll_options() -> Vec<CommandOption> {
///     Vec::new()
/// }
/// ```
#[proc_macro_attribute]
pub fn command(attr: TokenStream, input: TokenStream) -> TokenStream {
    if attr.is_empty() {
        let base = parse_macro_input!(input as Base);

        match Command::try_from(base) {
            Ok(command) => TokenStream::from(quote! { #command }),
            Err(err) => TokenStream::from(err.to_compile_error()),
        }
    } else {
        let message = format!("expected `#[command]`, found #[command({})]", attr);
        let err = SynError::new(Span::call_site(), message);

        TokenStream::from(err.to_compile_error())
    }
}

struct Base {
    attributes: Vec<Attribute>,
    name: Ident,
}

impl Parse for Base {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let _ = input.parse::<Visibility>()?;
        let _ = input.parse::<Token![struct]>()?;
        let name = input.parse()?;
        let _ = input.parse::<Token![;]>()?;

        Ok(Self { attributes, name })
    }
}
