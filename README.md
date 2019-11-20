# A simple lab using Rust to manipulate strings

This is a very simple lab where we'll use Rust to
implement the `disemvowel` function that we covered
[in a previous C lab](https://github.com/UMM-CSci-Systems/C-strings-and-memory-management#disemvowel).

The initial code has a `main.rs` that has a `main` function
and a set of tests for a (missing) `disemvowel` function.
You should write the missing `disemvowel` function so that the
tests all pass.

Use `cargo test` to run the tests.

The solution is just a few lines. There are no doubt a _lot_
of different ways to do this, and I don't claim to know Rust
well enough to know the "right" or "best" way. That said,
I found the following things useful:

- Creating a vector of vowels
- Iterating over the characters in the input string using `.chars()`
- Pushing non-vowel characters onto an initially empty string

I found [this StackExchange answer](https://codereview.stackexchange.com/a/172910) useful, although I preferred the
use of `contains` in the code from the original poster. Your mileage
may vary.
