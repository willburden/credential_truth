//! Contains miscellanous utility functions that
//! wouldn't belong in any other module. 

#![deny(clippy::missing_docs_in_private_items)]

/// Converts a string slice of the format "{author1}:{author2}:..."
/// to the format "{author1}, {author2}, ...".
/// 
/// # Example
/// 
/// ```ignore
/// assert_eq!(authors("Alice:Bob"), "Alice, Bob");
/// ```
pub fn authors(raw: &str) -> String {
    raw.replace(':', ", ")
}
