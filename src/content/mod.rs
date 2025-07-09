//! Contents that can be displayed in [`ContentPage`][`crate::page::content::ContentPage`].

pub mod form;
pub mod markdown;
pub mod tree;

pub use form::{FormContent, FormContentBuilder};
pub use markdown::MarkdownContent;
pub use tree::{TreeContent, TreeContentBuilder};
use windows_core::ComObject;

/// Represents all kinds of content that can be displayed in a [`ContentPage`][`crate::page::content::ContentPage`].
pub enum Content {
    /// A form content that can be used to collect user input.
    Form(ComObject<FormContent>),
    /// A markdown content that can be used to display formatted text.
    Markdown(ComObject<MarkdownContent>),
    /// A tree content that can be used to display nested content.
    Tree(ComObject<TreeContent>),
}

impl From<FormContent> for Content {
    fn from(value: FormContent) -> Self {
        Content::Form(ComObject::new(value))
    }
}

impl From<MarkdownContent> for Content {
    fn from(value: MarkdownContent) -> Self {
        Content::Markdown(ComObject::new(value))
    }
}

impl From<TreeContent> for Content {
    fn from(value: TreeContent) -> Self {
        Content::Tree(ComObject::new(value))
    }
}

impl From<ComObject<FormContent>> for Content {
    fn from(value: ComObject<FormContent>) -> Self {
        Content::Form(value)
    }
}

impl From<ComObject<MarkdownContent>> for Content {
    fn from(value: ComObject<MarkdownContent>) -> Self {
        Content::Markdown(value)
    }
}

impl From<ComObject<TreeContent>> for Content {
    fn from(value: ComObject<TreeContent>) -> Self {
        Content::Tree(value)
    }
}

impl From<&Content> for crate::bindings::IContent {
    fn from(value: &Content) -> Self {
        match value {
            Content::Form(content) => content.to_interface(),
            Content::Markdown(content) => content.to_interface(),
            Content::Tree(content) => content.to_interface(),
        }
    }
}
