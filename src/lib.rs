//! Rust idiomatic bindings and SDK for the
//! [Command Palette](https://learn.microsoft.com/en-us/windows/powertoys/command-palette/overview).
 
pub mod bindings;
pub mod cmd;
pub mod cmd_item;
pub mod cmd_provider;
pub mod cmd_result;
pub mod content;
pub mod ctx_item;
pub mod details;
pub mod ext;
pub mod ext_factory;
pub mod ext_registry;
pub mod fallback;
pub mod filter;
pub mod host;
pub mod icon;
pub mod notify;
pub mod page;
pub mod prelude;
pub mod settings;
pub mod utils;

#[cfg(feature = "unstable-doc")]
pub mod _cookbook;
