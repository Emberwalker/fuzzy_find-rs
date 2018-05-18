//! # fuzzy_match
//!
//! `fuzzy_match` provides functionality for finding the best match from a set of strings, optionally with items
//! assosciated with each candidate string. This crate is based in large part on the awesome
//! [`fuzzy_match` Ruby gem](https://github.com/seamusabshere/fuzzy_match), but this crate only implements the basic
//! functionality and skips the more advanced functionality for now.
#![feature(test)]

extern crate sliding_windows;
#[cfg(all(feature = "nightly", test))] extern crate test;

pub mod algorithms;
pub(crate) mod util;

/// Fuzzy finds a set of string-item pairs using a Sorensen Dice coefficient and Levenshtein for breaking ties. May
/// return None if no match is similar.
///
/// # Examples
/// ```rust
/// use fuzzy_match::fuzzy_match;
///
/// let haystack = vec![("rust", 0), ("java", 1), ("lisp", 2)];
/// assert_eq!(Some(0), fuzzy_match("bust", &haystack));
/// ```
pub fn fuzzy_match<T>(needle: &str, haystack: &[(&str, T)]) -> Option<T> {
    if haystack.len() == 0 {
        panic!("No haystack provided!");
    }

    // TODO
    unimplemented!();
}
