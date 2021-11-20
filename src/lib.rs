//! *A.K.A docker-credential-truth*
//! 
//! credential_truth is a package that compiles to a program, docker-credential-truth,
//! which can act as a Docker Credential Helper. To find out more about what
//! this is, visit the [Github repo]. Health warning: it's not particularly interesting.
//! 
//! [Github repo]: https://github.com/willburden/credential_truth

#![deny(clippy::missing_docs_in_private_items)]
#![forbid(unsafe_code)]

mod util;
mod logger;
mod error;
mod subcommands;

use util::*;
use logger::init_logger;
use error::Error;

use subcommands::{init, get, store, list};

use clap::{App, AppSettings, SubCommand, Arg, ArgMatches};
use std::env;

/// Entrypoint function, called whenever the program is run.
/// 
/// Reads the user's input, and outputs an appropriate response.
pub fn run() {
    init_logger();

    let request = match_app();
    match request.subcommand() {
        (subcommand, Some(request)) => {
            // Since we've parsed a subcommand, we are now certain we will be using
            // pass this run.
            let pass = match find_pass() {
                Some(pass) => pass,
                None => {
                    log::error!(
"pass couldn't be found on your PATH.
More info about pass here: https://www.passwordstore.org/"
                    );
                    return;
                }
            };

            exec_subcommand(&pass, subcommand, request);
        }

        (_, None) => { // This can't happen with the current App specification.
            panic!("No subcommand matched even though it's set as required!");
        }
    }
}

/// Executes a subcommand.
///
/// Redirects the control flow to the correct module for the given
/// subcommand, passing it any relevant args from the user's request.
fn exec_subcommand(pass_path: &str, subcommand: &str, request: &ArgMatches) {
    let result = match subcommand {
        "init" => {
            // Safe to unwrap as key-id is required.
            // value_of panics at invalid UTF-8 code points. I'm not sure if that's
            // something to worry about, so I'm gonna leave it as it is until it becomes
            // an issue.
            let key_id = request.value_of("key-id").unwrap();

            init(pass_path, key_id)
        }

        "store" => {
            store(pass_path)
        }

        "get" => {
            get(pass_path)
        }

        "list" => {
            list()
        }

        subcommand => {
            Err(Error::Message(format!("The '{}' subcommand is not yet implemented.", subcommand)))
        }
    };

    // if let Err(Error(message)) = result {
    //     panic!("{}", message);
    // }

    if let Err(error) = result {
        match error {
            Error::Message(message) => log::error!("{}", message),
            Error::FromError(error) => log::error!("{:?}", error)
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
    App::new("docker-credential-truth")
        .author(&authors(env!("CARGO_PKG_AUTHORS"))[..])
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::GlobalVersion)
        .after_help(
"For more information, visit https://github.com/willburden/docker-credential-truth"
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialises the credential helper for the current user")
                .arg(
                    Arg::with_name("key-id")
                        .help("The ID of the GPG key to initialise your pass store with")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("store")
                .about("Stores the credentials sent to stdin")
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Retrieves the credentials for the URL sent to stdin")
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists all stored credentials for the current user")
        )
        .subcommand(
            SubCommand::with_name("erase")
                .about("Deletes all credentials for the URL sent to stdin")
        )
        .get_matches()
}
