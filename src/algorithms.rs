//! This module provides the raw algorithms used for string similarity. These can be used directly if raw weights are
//! required between two strings, but most users should prefer the functionality in the crate root.

use sliding_windows::{IterExt, Storage};
use std::collections::HashSet;
use util::round_score_decimal;

const SPACE: char = ' ';

/// Trait defining a single similarity algorithm.
pub trait SimilarityAlgorithm {
    /// Create a new instance of this algorithm. It's recommended to keep a single instance around as long as possible,
    /// as this allows any temporary storage to be recycled without additional allocations.
    fn new() -> Self;

    /// Gets the similarity of a given pair of strings using this algorithm. Two identical strings will
    /// always produce `1.0`, and mismatching length 1 strings produce `0.0`. If either string is empty, `0.0` will be
    /// returned. Implementations within the crate round the weight to 5 decimal places, but other implementations don't
    /// need to do the same.
    fn get_similarity(&mut self, a: &str, b: &str) -> f32;
}

/// Sorensen-Dice coefficient algorithm.
pub struct SorensenDice(Storage<char>);
impl SorensenDice {
    fn str_to_char_set(&mut self, s: &str) -> HashSet<(char, char)> {
        if s.len() < 2 {
            return HashSet::new();
        }
        let windows = s.chars().sliding_windows(&mut self.0);
        windows
            .map(|x| {
                let mut iter = x.iter();
                (iter.next().map(|it| *it), iter.next().map(|it| *it))
            })
            .filter_map(|entry| {
                if let (Some(a), Some(b)) = entry {
                    if a != SPACE && b != SPACE {
                        Some((a, b))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<HashSet<(char, char)>>()
    }
}
impl SimilarityAlgorithm for SorensenDice {
    fn new() -> SorensenDice {
        SorensenDice(Storage::new(2))
    }

    fn get_similarity(&mut self, a: &str, b: &str) -> f32 {
        if a == b {
            1f32
        } else if a.len() == 1 && b.len() == 1 {
            0f32
        } else if a.len() == 0 || b.len() == 0 {
            0f32
        } else {
            let acs = self.str_to_char_set(a);
            let bcs = self.str_to_char_set(b);

            let union = acs.len() + bcs.len();
            let intersect = acs.intersection(&bcs).count();

            round_score_decimal((2f32 * intersect as f32) / union as f32)
        }
    }
}

/// Levenshtein edit distance algorithm.
// Add a hidden, unused param to prevent direct construction.
pub struct Levenshtein(bool);
impl SimilarityAlgorithm for Levenshtein {
    fn new() -> Levenshtein {
        Levenshtein(false)
    }

    fn get_similarity(&mut self, s: &str, t: &str) -> f32 {
        use std::cmp::{min, max};

        let n = s.len();
        let m = t.len();

        if s == t {
            1f32
        } else if n == 1 && m == 1 {
            0f32
        } else if n == 0 || m == 0 {
            0f32
        } else {
            // For a description of the algorithm, see
            // https://people.cs.pitt.edu/~kirk/cs1501/Pruhs/Spring2006/assignments/editdistance/Levenshtein%20Distance.htm

            // Get character vector for both strings
            let s_chars = s.chars().collect::<Vec<char>>();
            let t_chars = t.chars().collect::<Vec<char>>();

            // Build the matrix
            let mut rows: Vec<Vec<usize>> = Vec::with_capacity(m);
            for i in 0..n + 1 {
                let mut row = Vec::with_capacity(n + 1);
                for j in 0..m + 1 {
                    if i == 0 { 
                        row.push(j);
                    } else if j == 0 {
                        row.push(i);
                    } else {
                        row.push(0);
                    }
                }
                rows.push(row);
            }

            // Iterate over the strings
            for i in 1..n + 1 {
                for j in 1..m + 1 {
                    let cost = if s_chars[i - 1] == t_chars[j - 1] {
                        0
                    } else {
                        1
                    };

                    let above = 1 + rows[i-1][j];
                    let left = 1 + rows[i][j-1];
                    let diag = cost + rows[i-1][j-1];

                    rows[i][j] = min(min(above, left), diag);
                }
            }

            round_score_decimal(1f32 - (rows[n][m] as f32 / max(n, m) as f32))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Test to catch sliding_window panics
    #[test]
    fn test_sorensen_bigrams_single_char() {
        assert_eq!(0, SorensenDice::new().str_to_char_set("b").len());
    }

    #[test]
    fn test_sorensen_bigrams_correctness() {
        let mut bigrams = HashSet::new();
        bigrams.insert(('r', 'u'));
        bigrams.insert(('u', 's'));
        bigrams.insert(('s', 't'));
        assert_eq!(bigrams, SorensenDice::new().str_to_char_set("rust"));
    }

    #[test]
    fn test_sorensen_eq_strs() {
        assert_eq!(1f32, SorensenDice::new().get_similarity("string", "string"));
    }

    #[test]
    fn test_sorensen_one_char() {
        assert_eq!(0f32, SorensenDice::new().get_similarity("a", "b"));
    }

    #[test]
    fn test_sorensen_empty_str() {
        assert_eq!(0f32, SorensenDice::new().get_similarity("string", ""));
    }

    // Added due to sliding_window panic when either string is size 1
    #[test]
    fn test_sorensen_one_short() {
        assert_eq!(0f32, SorensenDice::new().get_similarity("rust", "b"))
    }

    #[test]
    fn test_sorensen_correctness() {
        let mut inst = SorensenDice::new();
        assert_eq!(0.66667f32, inst.get_similarity("rust", "bust"));
        assert_eq!(0.0f32, inst.get_similarity("rust", "ritz"));
        assert_eq!(0.72727f32, inst.get_similarity("chance", "enhance"));
    }

    #[test]
    fn test_levenshtein_eq_strs() {
        assert_eq!(1f32, Levenshtein::new().get_similarity("string", "string"));
    }

    #[test]
    fn test_levenshtein_one_char() {
        assert_eq!(0f32, Levenshtein::new().get_similarity("a", "b"));
    }

    #[test]
    fn test_levenshtein_empty_str() {
        assert_eq!(0f32, Levenshtein::new().get_similarity("string", ""));
    }

    #[test]
    fn test_levenshtein_correctness() {
        let mut inst = Levenshtein::new();
        assert_eq!(0.75f32, inst.get_similarity("rust", "bust"));
        assert_eq!(0.25f32, inst.get_similarity("rust", "ritz"));
        assert_eq!(0.71429f32, inst.get_similarity("chance", "enhance"));
    }

    #[cfg(feature = "nightly")]
    mod bench {
        use super::super::*;
        use test::Bencher;

        fn get_words_contents() -> String {
            use std::fs::File;
            use std::io::prelude::*;
            // Load the words file.
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/words_alpha.txt");
            let mut contents = String::new();
            {
                let mut f = File::open(path).expect("Unable to load words_alpha.txt");
                f.read_to_string(&mut contents)
                    .expect("Unable to read words_alpha.txt");
            }
            contents
        }

        fn get_words(contents: &str) -> Vec<&str> {
            contents
                .lines()
                .map(|it| it.trim_right_matches("\n").trim_right_matches("\r"))
                .collect()
        }

        #[bench]
        fn bench_sorensen_pairs(bench: &mut Bencher) {
            let contents = get_words_contents();
            let words = get_words(&contents);
            let words_iter = words.iter();

            bench.iter(|| {
                let mut inst = SorensenDice::new();
                words_iter
                    .clone()
                    .map(|word| inst.str_to_char_set(word).len())
                    .collect::<Vec<_>>()
            });
        }

        #[bench]
        fn bench_complete_sorensen(bench: &mut Bencher) {
            let contents = get_words_contents();
            let words = get_words(&contents);
            let words_iter = words.iter();

            bench.iter(|| {
                let mut inst = SorensenDice::new();
                words_iter
                    .clone()
                    .map(|word| inst.get_similarity("rust", word))
                    .collect::<Vec<f32>>()
            });
        }

        #[bench]
        fn bench_complete_levenshtein(bench: &mut Bencher) {
            let contents = get_words_contents();
            let words = get_words(&contents);
            let words_iter = words.iter();

            bench.iter(|| {
                let mut inst = Levenshtein::new();
                words_iter
                    .clone()
                    .map(|word| inst.get_similarity("rust", word))
                    .collect::<Vec<f32>>()
            });
        }
    }
}
