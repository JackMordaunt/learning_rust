# minigrep

This is a toy implementation of basic `grep` functionality written in rust; to learn rust.

The simplest use case of `grep` searches a specified file for a specified string.

`minigrep <search-string> <file-path>`

## Command Line Tech Concepts

- File IO.
- Regex.
- Command line arguments.
- Environment variables.
- `stdin`, `stdout`, `stderr`.

## Rust Concepts
- Structuring code in rust.
- Vectors and String types.
- Error handling.
- Traits and lifetimes.
- Testing.
- Closures.
- Iterators.
- Trait objects.

## Notes

Since we are using `std::env::args` the app will panic if any arguments contain invalid Unicode. To accept invalid Unicode you would use `std::env::args_os`.

