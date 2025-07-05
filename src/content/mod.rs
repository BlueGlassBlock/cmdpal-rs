pub mod form;
pub mod markdown;
pub mod tree;

pub use form::{FormContent, FormContentBuilder};
pub use markdown::MarkdownContent;
pub use tree::{TreeContent, TreeContentBuilder};
use windows::core::ComObject;

pub enum Content {
    Form(ComObject<FormContent>),
    Markdown(ComObject<MarkdownContent>),
    Tree(ComObject<TreeContent>),
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
