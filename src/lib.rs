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
/// return None if no match is similar. This consumes the input vector. See
/// [`fuzzy_match_with_algorithms`](fuzzy_match::fuzzy_match_with_algorithms) for additional details.
///
/// # Examples
/// ```rust
/// use fuzzy_match::fuzzy_match;
///
/// let haystack = vec![("rust", 0), ("java", 1), ("lisp", 2)];
/// assert_eq!(Some(0), fuzzy_match("bust", haystack));
/// ```
/// 
/// # Panics
/// This function will panic if the haystack is empty (length 0).
pub fn fuzzy_match<T>(needle: &str, haystack: Vec<(&str, T)>) -> Option<T> {
    fuzzy_match_with_algorithms::<T, algorithms::SorensenDice, algorithms::Levenshtein>(needle, haystack)
}

/// Version of [`fuzzy_match`](fuzzy_match::fuzzy_match) which allows overriding the first and second choice algorithms,
/// instead of using Sorensen-Dice and Levenshtein respectively. This consumes the input vector.
/// 
/// # Examples
/// ```rust
/// use fuzzy_match::fuzzy_match_with_algorithms;
/// use fuzzy_match::algorithms::{SorensenDice, Levenshtein};
///
/// let haystack = vec![("rust", 0), ("java", 1), ("lisp", 2)];
/// // Search with Levenshtein first, then Sorensen-Dice.
/// assert_eq!(Some(0), fuzzy_match_with_algorithms::<_, Levenshtein, SorensenDice>("bust", haystack));
/// ```
/// 
/// # Panics
/// This function will panic if the haystack is empty (length 0).
pub fn fuzzy_match_with_algorithms<T, FST: algorithms::SimilarityAlgorithm, SND: algorithms::SimilarityAlgorithm>(
    needle: &str,
    mut haystack: Vec<(&str, T)>,
) -> Option<T> {
    if haystack.len() == 0 {
        panic!("No haystack provided!");
    }

    let mut highest_set: Vec<(&str, T)> = Vec::new();
    let mut highest_weight = 0f32;
    let mut first_algo = FST::new();

    // Try with first-case algorithm.
    for (name, item) in haystack.drain(..) {
        let weight = first_algo.get_similarity(needle, name);
        if weight == highest_weight {
            highest_set.push((name, item))
        } else if weight > highest_weight {
            highest_weight = weight;
            highest_set.clear();
            highest_set.push((name, item));
        }
    }

    if highest_set.is_empty() {
        return None;
    } else if highest_set.len() == 1 {
        let (_, item) = highest_set.remove(0);
        return Some(item);
    }

    // Break ties with second-case algorithm
    let mut snd_highest_set: Vec<(&str, T)> = Vec::new();
    let mut snd_highest_weight = 0f32;
    let mut second_algo = SND::new();

    for (name, item) in highest_set.drain(..) {
        let weight = second_algo.get_similarity(needle, name);
        if weight == snd_highest_weight {
            snd_highest_set.push((name, item))
        } else if weight > highest_weight {
            snd_highest_weight = weight;
            snd_highest_set.clear();
            snd_highest_set.push((name, item));
        }
    }

    if snd_highest_set.is_empty() || snd_highest_set.len() > 1 {
        None
    } else {
        let (_, item) = snd_highest_set.remove(0);
        Some(item)
    }
}
