//! This crate implements a few utilities for the [listenbrain-rs](https://crates.io/crates/listenbrainz) crate.

#![deny(
    missing_docs,
    missing_debug_implementations,
    unsafe_code,
    unstable_features
)]

/// This module contains readers for the APIs endpoint. Those allow for easier iterating over pages
pub mod readers;

// Re-exports
pub use listenbrainz;
pub use musicbrainz_rs as musicbrainz;
