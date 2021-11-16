//! ### A.K.A docker-credential-truth
//! 
//! *credential_truth* is a package that compiles to a program, *docker-credential-truth*,
//! which can act as a Docker Credential Helper. To find out more about what
//! this is, visit the [Github repo]. Health warning: it's not particularly interesting.
//! 
//! [Github repo]: https://github.com/willburden/credential_truth

#![deny(clippy::missing_docs_in_private_items)]
#![forbid(unsafe_code)]

use clap::{App, AppSettings, SubCommand, Arg, ArgMatches};
use simple_logger::SimpleLogger;

mod util;
use util::authors;

/// Entrypoint function, called whenever the program is run.
/// 
/// Reads the user's input, and outputs an appropriate response.
pub fn run() {
    // Safe to unwrap as init only errors if another logger is already set.
    SimpleLogger::new().init().unwrap();

    let request = match_app();
    match request.subcommand() {
        (subcommand, Some(_)) => {
            log::error!("The '{}' subcommand is not yet implemented.", subcommand);
        },
        (_, None) => { // This can't happen with the current App specification.
            panic!("No subcommand matched even though it's set as required!");
        }
    }
}

/// Provides the command-line app interface.
/// 
/// User's input will either be parsed into something
/// that resembles a coherent request, or they will
/// be presented with usage information and the process
/// will exit.
fn match_app<'a>() -> ArgMatches<'a> {
    // Using the clap crate:
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(&authors(env!("CARGO_PKG_AUTHORS"))[..])
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_help(
"This program is intended as a substitute for docker-credential-pass.
For more information, see https://github.com/willburden/docker-credential-truth"
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialises the credential helper for the current user.")
                .arg(
                    Arg::with_name("key-id")
                        .help("The ID of the GPG key to initialise your pass store with.")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("store")
                .about("Stores the credentials sent to stdin.")
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Retrieves the credentials for the URL sent to stdin.")
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists all stored credentials for the current user.")
        )
        .subcommand(
            SubCommand::with_name("erase")
                .about("Deletes all credentials for the URL sent to stdin.")
        )
        .get_matches()
}
