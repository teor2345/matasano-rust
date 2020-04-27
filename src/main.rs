//! Matasano Cryptopals Challenges

#![deny(missing_docs)]

// Each challenge is a sub-module of the binary root module

/// Set 1, Challenge 1
mod s1c1;

/// Check the results of each challenge
fn main() {
    s1c1::check();
}
