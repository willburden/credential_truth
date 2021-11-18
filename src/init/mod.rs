//! Provides the functionality for the init subcommand.

use std::process::Command;

use crate::error::Error;
use crate::util::{set_password_store_dir, for_each_line};

/// Initialises the password store for the current user.
/// 
/// The first argument is the path to the pass binary.
/// Takes a GPG key id as an argument, used to initialise the
/// password store with pass.
pub fn init(pass: &str, key: &str) -> Result<(), Error> {
    let dir = set_password_store_dir();

    std::env::var("PASSWORD_STORE_DIR").unwrap();

    log::info!("Initialising password store in {} with key {}", dir, key);

    log::debug!("   run: {} init {}", pass, key);

    let pass_output = Command::new(pass)
        .args(["init", key])
        .output()?;

    let outputs = [
        ("stdout", pass_output.stdout),
        ("stderr", pass_output.stderr)
    ];

    for output in outputs {
        for_each_line(&output.1[..],|line| {
            log::debug!("{}: {}", output.0, line)
        })?;
    }

    Ok(())
}
