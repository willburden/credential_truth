//! Contains miscellanous utility functions that
//! wouldn't belong in any other module. 

use std::env;
use std::path::PathBuf;
use std::io::{BufRead, BufReader, Read};

use home::home_dir;

/// Converts a string slice of the format "{author1}:{author2}:..."
/// to the format "{author1}, {author2}, ...".
pub fn authors(raw: &str) -> String {
    raw.replace(':', ", ")
}

/// Attempts to locate pass on the user's system.
/// 
/// Returns Some(path) where path is the path to the pass binary
/// if it was found, or None if it wasn't.
pub fn find_pass() -> Option<String> {
    match which::which("pass") {
        Ok(path) => Some(path.to_str().unwrap().to_string()),
        Err(_) => None
    }
}

/// Guarantees that the PASSWORD_STORE_DIR env var is set.
/// 
/// The value will either be $CREDENTIAL_TRUTH
/// or ~/.docker-credential-truth/, depending
/// on whether the CREDENTIAL_TRUTH var is set.
pub fn set_password_store_dir() -> String {
    let default_path = {
        let mut buf = home_dir().expect("Home directory unknown!");
        buf.push(".docker-credential-truth");
        buf
    };

    let default_path = default_path.to_str().expect("Home directory isn't unicode!");

    let path_str = env::var("CREDENTIAL_TRUTH")
        .unwrap_or_else(|_| String::from(default_path));

    let path_str = PathBuf::from(path_str).to_string_lossy()
        .into_owned();
    env::set_var("PASSWORD_STORE_DIR", &path_str);

    #[cfg(debug_assertions)]
    log::trace!("$PASSWORD_STORE_DIR = {}", &path_str);
    
    path_str
}

/// Performs some closure for each line in a string.
/// 
/// Can be used to log a multiline string such that
/// it shows the log level on each line.
pub fn for_each_line<R, F, A>(content: R, mut f: F) -> Result<(), std::io::Error>
where
    R: Read,
    F: FnMut(String) -> A
{
    let reader = BufReader::new(content);

    for line in reader.lines() {
        let line = line?;
        f(line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;
    use std::fmt::Write;
    use std::error::Error;

    use crate::util::*;

    #[test]
    fn authors_conversion_is_correct() {
        assert_eq!(authors("alice:bob:charlie"), "alice, bob, charlie");
    }

    #[test]
    fn write_for_each_line() -> Result<(), Box<dyn Error>> {
        let mut buf = String::new();
        
        let input = 
        "Hello world
        Line 2
        This is the third line.";

        for_each_line(input.as_bytes(), |line| writeln!(buf, "{}", line))?;

        assert_eq!(buf.as_str(), format!("{}\n", input));

        Ok(())
    }

    #[test]
    fn password_store_dir_from_env() {
        test_password_store_dir(Some("/.docker-auth/"), "/.docker-auth/");
    }

    #[test]
    fn default_password_store_dir() {
        test_password_store_dir(None, "/home/will/.docker-credential-truth");
    }

    fn test_password_store_dir(cred_truth: Option<&str>, expected: &str) {
        env::remove_var("PASSWORD_STORE_DIR");
        
        if let Some(cred_truth) = cred_truth {
            env::set_var("CREDENTIAL_TRUTH", cred_truth);
        } else {
            env::remove_var("CREDENTIAL_TRUTH");
        }

        let expected = String::from(expected);

        assert_eq!(
            &set_password_store_dir(),
            &expected
        );

        assert_eq!(
            env::var("PASSWORD_STORE_DIR"),
            Ok(expected)
        );
    }
}
