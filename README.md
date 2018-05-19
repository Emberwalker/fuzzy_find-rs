# fuzzy_match

A port of the basic features of the [`fuzzy_match` Ruby gem](https://github.com/seamusabshere/fuzzy_match) to Rust.

## Usage

To use the default configuration (SorensenDice then Levenshtein to break ties):

```rust
use fuzzy_match::fuzzy_match;

let haystack = vec![("rust", 0), ("java", 1), ("lisp", 2)];
assert_eq!(Some(0), fuzzy_match("bust", haystack));
```