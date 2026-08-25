//! Rust idiomatic bindings and SDK for the
//! [Command Palette](https://learn.microsoft.com/en-us/windows/powertoys/command-palette/overview).

// LINEBENDER LINT SET - lib.rs - v4
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod bindings;

pub mod cmd;
pub mod cmd_item;
pub mod cmd_provider;
pub mod cmd_result;
pub mod content;
pub mod ctx_item;
pub mod details;
pub mod ext;
pub mod ext_api_stubs;
pub mod ext_factory;
pub mod ext_registry;
pub mod fallback;
pub mod filter;
pub mod grid;
pub mod host;
pub mod icon;
pub mod notify;
pub mod page;
pub mod settings;
pub mod utils;

pub mod prelude;

#[cfg(feature = "unstable-doc")]
pub mod _cookbook;
