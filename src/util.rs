//! Contains miscellanous utility functions that
//! wouldn't belong in any other module. 

use std::env;
use std::path::Path;

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
/// The value will either be $CREDENTIAL_TRUTH/$USER/
/// or /var/docker-credential-truth/$USER/, depending
/// on whether the CREDENTIAL_TRUTH var is set.
pub fn set_password_store_dir() {
    let path_str = match env::var("CREDENTIAL_TRUTH") {
        Ok(path_str) => path_str,
        Err(_) => String::from("/var/docker-credential-truth/")
    };

    let parent_path = Path::new(&path_str);

    let username = whoami::username();
    let child_path = Path::new(&username);

    let path = parent_path.join(child_path);

    env::set_var("PASSWORD_STORE_DIR", path.to_str().unwrap());
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::util::*;

    #[test]
    fn authors_conversion_is_correct() {
        assert_eq!(authors("alice:bob:charlie"), "alice, bob, charlie");
    }

    #[test]
    fn password_store_dir_from_env() {
        test_password_store_dir(Some("/home/will/.docker-auth"), "/home/will/.docker-auth/will");
    }

    #[test]
    fn default_password_store_dir() {
        test_password_store_dir(None, "/var/docker-credential-truth/will");
    }

    fn test_password_store_dir(cred_truth: Option<&str>, expected: &str) {
        env::remove_var("PASSWORD_STORE_DIR");
        
        if let Some(cred_truth) = cred_truth {
            env::set_var("CREDENTIAL_TRUTH", cred_truth);
        } else {
            env::remove_var("CREDENTIAL_TRUTH");
        }

        set_password_store_dir();

        assert_eq!(
            env::var("PASSWORD_STORE_DIR").unwrap(),
            String::from(expected)
        );
    }
}
