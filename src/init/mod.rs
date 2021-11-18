//! Provides the functionality for the init subcommand.

use std::process::Command;

use crate::error::Error;

/// Initialises the password store for the current user.
/// 
/// The first argument is the path to the pass binary.
/// Takes a GPG key id as an argument, used to initialise the
/// password store with pass.
pub fn init(pass: &str, key: &str) -> Result<(), Error> {
    log::debug!("   run: {} init {}", pass, key);

    let pass_output = Command::new(pass)
        // .args(["init", key]);
        .output()?;

    let stdout = String::from_utf8(pass_output.stdout)?;
    let stderr = String::from_utf8(pass_output.stderr)?;

    log::debug!("stdout: {}", stdout);
    log::debug!("stderr: {}", stderr);

    Ok(())

    // log::info!("Initialised password store in {} with key {}.", dir, key);
}
