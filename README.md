# fuzzy_match [![Crates.io][cratebadge]][cratelink] [![Docs.rs][docsbadge]][docslink] [![Build Status][buildbadge]][buildstatus]

A port of the basic features of the [`fuzzy_match` Ruby gem](https://github.com/seamusabshere/fuzzy_match) to Rust.

## Usage

To use the default configuration (SorensenDice then Levenshtein to break ties):

```rust
use fuzzy_match::fuzzy_match;

let haystack = vec![("rust", 0), ("java", 1), ("lisp", 2)];
assert_eq!(Some(0), fuzzy_match("bust", haystack));
```

[cratebadge]: https://img.shields.io/crates/v/fuzzy_match.svg
[cratelink]: https://crates.io/crates/fuzzy_match
[docsbadge]: https://docs.rs/fuzzy_match/badge.svg
[docslink]: https://docs.rs/fuzzy_match
[buildbadge]: https://travis-ci.org/Emberwalker/fuzzy_find-rs.svg?branch=master
[buildstatus]: https://travis-ci.org/Emberwalker/fuzzy_find-rs