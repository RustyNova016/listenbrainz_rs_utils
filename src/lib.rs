//! # Listenbrainz-rs-utils
//!
//! A collection of utilities for the [listenbrain-rs](https://crates.io/crates/listenbrainz) crate. Those may or may not be included upstream later on.
//!
//! ## Readers
//! Readers allows easier handling of paginated APIs.

#![warn(
    missing_docs,
)]

#![deny(
    missing_debug_implementations,
    unsafe_code,
    unstable_features
)]

/// This module contains readers for the APIs endpoint. Those allow for easier iterating over pages
pub mod readers;

// Re-exports
pub use listenbrainz;
pub use musicbrainz_rs as musicbrainz;
