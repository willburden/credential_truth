//! Implements the program's subcommands: *init*, *store*,
//! *get*, *list*, and *erase*.

use std::process::{Command, Output, Stdio};
use std::io::stdin;
use std::io::Write;
use std::fs;
use std::path::Path;

use crate::error::Error;
use crate::util::{set_password_store_dir, for_each_line};

use serde::{Serialize, Deserialize};

/// Initialises the password store for the current user.
/// 
/// The first argument is the path to the pass binary.
/// Takes a GPG key id as an argument, used to initialise the
/// password store with pass.
pub fn init(pass: &str, key: &str) -> Result<(), Error> {
    let dir = set_password_store_dir();

    std::env::var("PASSWORD_STORE_DIR").unwrap();

    log::info!("Initialising password store in '{}' with key {}", dir, key);

    log::debug!("   run: {} init {}", pass, key);

    let pass_output = Command::new(pass)
        .args(["init", key])
        .output()?;

    log_command_output(&pass_output)?;

    Ok(())
}

/// A full set of authentication details.
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct AuthDetails {
    /// The URL of the server this authenticates with.
    ServerURL: String,
    /// The user's username, or '<token>'.
    Username: String,
    /// Either a password or an identity token.
    Secret: String
}

/// Stores some auth details to the pass store.
/// 
/// The argument is the path to pass. The function reads
/// the auth details as JSON from stdin.
pub fn store(pass: &str) -> Result<(), Error> {
    set_password_store_dir();

    log::info!("Enter auth details:");

    let mut auth: AuthDetails = serde_json::from_reader(stdin())?;

    log::trace!("Received auth details:");
    log::trace!("   ServerURL = {}", auth.ServerURL);
    log::trace!("   Username  = {}", auth.Username);
    log::trace!("   Secret    = <a secret>");

    auth.ServerURL = base64_url::encode(&auth.ServerURL);

    log::trace!("Encoded URL = {}", auth.ServerURL);
    
    let pass_name = format!("{}/{}", auth.ServerURL, auth.Username);
    log::info!("Storing secret under {}", pass_name);

    log::debug!("   run: {} insert {}", pass, pass_name);
    let mut pass_process = Command::new(pass)
        .args(["insert", "-fe", &pass_name])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    
    let mut input_stream = pass_process.stdin.take()
        .expect("Failed to write to pass's stdin!");
    
    writeln!(input_stream, "{}", auth.Secret)?;

    let pass_output = pass_process.wait_with_output()?;
    log_command_output(&pass_output)?;

    Ok(())
}

/// The payload that 'get' returns.
#[derive(Serialize)]
#[allow(clippy::missing_docs_in_private_items)]
#[allow(non_snake_case)]
struct GetPayload {
    Username: String,
    Secret: String
}

/// Retrieves the stored auth details for a given server.
/// 
/// The argument is the path to pass. The function reads
/// the URL of the server from stdin.
/// It then prints the auth details to stdout.
pub fn get(pass: &str) -> Result<(), Error> {
    let dir = set_password_store_dir();

    log::info!("Enter the server url:");
    let server_url = {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        buf
    };

    let server_url = base64_url::encode(&server_url.trim_end());
    let server_path = Path::new(&dir).join(Path::new(&server_url));

    log::debug!("Looking in directory '{}'", server_path.to_string_lossy());

    let username = {
        match fs::read_dir(server_path)?.next() {
            Some(entry) => entry?.file_name(),
            None => return Err(Error("No entry for the given server found.".to_string()))
        }
    };

    let username = {
        let username_str = username.to_str().expect("Username not valid unicode!");
        if let Some((username, _)) = username_str.rsplit_once('.') {
            username
        } else {
            username_str
        }
    };

    log::debug!("Found username '{}'", username);

    let entry_path = Path::new(&server_url).join(Path::new(&username));
    let entry_path = entry_path.to_str().expect("Entry not valid unicode!");

    log::debug!("   run: {} show {}", pass, entry_path);

    let pass_output = Command::new("pass")
        .args(["show", entry_path])
        .output()?;

    log_command_output(&pass_output)?;
    
    let secret = String::from_utf8(pass_output.stdout)
        .expect("Secret isn't valid utf-8!");
        
    let secret = secret.trim_end();
    
    let payload = GetPayload {
        Username: username.to_owned(),
        Secret: secret.to_owned()
    };

    let payload = serde_json::to_string(&payload)?;

    println!("{}", payload);

    Ok(())
}

/// Lists all the stored credentials.
/// 
/// 
pub fn list() -> Result<(), Error> {
    todo!()
}

/// Pretty-logs the Output of a Command.
fn log_command_output(output: &Output) -> Result<(), std::io::Error> {
    let outputs = [
        ("stdout", &output.stdout),
        ("stderr", &output.stderr)
    ];

    for output in outputs {
        for_each_line(&output.1[..],|line| {
            log::debug!("{}: {}", output.0, line)
        })?;
    }

    Ok(())
}
