
pub mod bindings;
pub mod cmd;
pub mod cmd_item;
pub mod fallback;
pub mod ext;
pub mod ext_factory;
pub mod details;
pub mod icon;
pub mod cmd_result;
pub mod provider;
pub mod host;
pub mod notify;
pub mod settings;
pub mod content;
pub mod ctx_item;
pub mod filter;
pub mod page;
pub(crate) mod utils;

#[cfg(feature = "unstable-doc")]
pub mod _cookbook;