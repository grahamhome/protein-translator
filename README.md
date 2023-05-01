# Protein Translator
This library provides a `CodonInfo` struct which can be used to convert a DNA sequence into a list of proteins. It is my solution to the Exercism.org "Protein Translation" exercise.
## Usage
See tests for usage examples, use `cargo test` to run tests.
## Concepts reviewed
- `.collect()`-ing an iterator of type `(T, V)` into a `HashMap<T, V>`
- `.collect()`-ing an iterator of type `Option<T>` into an `Option<Vec<T>>`
- Slicing a `&str` as bytes with `.as_bytes().chunks()`