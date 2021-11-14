//! The entrypoint binary for `docker-credential-truth`.

#![deny(clippy::missing_docs_in_private_items)]
#![forbid(unsafe_code)]

use credential_truth::run;

/// The main function simply passes execution over to
/// `lib.rs`.
fn main() {
    run();
}
