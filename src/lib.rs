//! A tool to help manage config files on multiple devices.
#![warn(
    rust_2018_idioms,
    rust_2021_compatibility,
    future_incompatible,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]
// TODO
#![allow(dead_code)]

pub mod cli;
pub(crate) mod core;
// Ignore the error module in code coverage.
#[cfg(not(tarpaulin_include))]
pub mod error;
