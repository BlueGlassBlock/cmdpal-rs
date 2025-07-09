//! Types for displaying the details tab.
//! 
//! `Details` is displayed in a tab occupying the right side of UI,
//! displaying additional information about the selected item.

use crate::icon::IconInfo;
use crate::utils::{OkOrEmpty, assert_send_sync, map_array};
use crate::{bindings::*, utils::ComBuilder};
use windows_core::{ComObject, Result, implement};
use windows_core::{AgileReference, HSTRING};

/// Represents a tag for classification.
///
/// See: [`ITag`]
///
#[doc = include_str!("./bindings_docs/ITag.md")]
#[implement(ITag)]
pub struct Tag {
    icon: Option<ComObject<IconInfo>>,
    text: HSTRING,
    foreground: Option<Color>,
    background: Option<Color>,
    tooltip: HSTRING,
}

/// Builder for [`Tag`].
pub struct TagBuilder {
    icon: Option<ComObject<IconInfo>>,
    text: Option<HSTRING>,
    foreground: Option<Color>,
    background: Option<Color>,
    tooltip: Option<HSTRING>,
}

impl TagBuilder {
    /// Creates a builder.
    pub fn new() -> Self {
        TagBuilder {
            icon: None,
            text: None,
            foreground: None,
            background: None,
            tooltip: None,
        }
    }

    /// Sets the icon for the tag.
    ///
    #[doc = include_str!("./bindings_docs/ITag/Icon.md")]
    pub fn icon(mut self, icon: ComObject<IconInfo>) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the text for the tag.
    ///
    #[doc = include_str!("./bindings_docs/ITag/Text.md")]
    pub fn text(mut self, text: impl Into<HSTRING>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets the foreground color for the tag.
    ///
    #[doc = include_str!("./bindings_docs/ITag/Foreground.md")]
    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Sets the background color for the tag.
    ///
    #[doc = include_str!("./bindings_docs/ITag/Background.md")]
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Sets the hover tooltip for the tag.
    ///
    #[doc = include_str!("./bindings_docs/ITag/ToolTip.md")]
    pub fn tooltip(mut self, tooltip: impl Into<HSTRING>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

impl ComBuilder for TagBuilder {
    type Output = Tag;
    fn build_unmanaged(self) -> Tag {
        Tag {
            icon: self.icon,
            text: self.text.unwrap_or_else(|| HSTRING::new()),
            foreground: self.foreground,
            background: self.background,
            tooltip: self.tooltip.unwrap_or_else(|| HSTRING::new()),
        }
    }
}

impl Default for TagBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ITag_Impl for Tag_Impl {
    fn Icon(&self) -> Result<crate::bindings::IIconInfo> {
        self.icon
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or_empty()
    }

    fn Text(&self) -> Result<windows_core::HSTRING> {
        Ok(self.text.clone())
    }

    fn Foreground(&self) -> Result<OptionalColor> {
        Ok(self.foreground.into())
    }

    fn Background(&self) -> Result<OptionalColor> {
        Ok(self.background.into())
    }

    fn ToolTip(&self) -> Result<windows_core::HSTRING> {
        Ok(self.tooltip.clone())
    }
}

/// Represents a collection of tags in details.
///
/// See: [`IDetailsTags`]
///
#[doc = include_str!("./bindings_docs/IDetailsTags.md")]
#[implement(IDetailsTags, IDetailsData)]
pub struct DetailsTags {
    tags: Vec<ComObject<Tag>>,
}

/// Builder for [`DetailsTags`].
pub struct DetailsTagsBuilder {
    tags: Vec<ComObject<Tag>>,
}

impl DetailsTagsBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        DetailsTagsBuilder { tags: Vec::new() }
    }

    /// Adds a tag to the collection.
    pub fn add_tag(mut self, tag: ComObject<Tag>) -> Self {
        self.tags.push(tag);
        self
    }

    /// Sets the tags for the collection.
    pub fn tags(mut self, tags: Vec<ComObject<Tag>>) -> Self {
        self.tags = tags;
        self
    }
}

impl ComBuilder for DetailsTagsBuilder {
    type Output = DetailsTags;
    fn build_unmanaged(self) -> DetailsTags {
        DetailsTags { tags: self.tags }
    }
}

impl Default for DetailsTagsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IDetailsData_Impl for DetailsTags_Impl {}

impl IDetailsTags_Impl for DetailsTags_Impl {
    fn Tags(&self) -> Result<windows_core::Array<ITag>> {
        Ok(map_array(&self.tags, |x| x.to_interface::<ITag>().into()))
    }
}

/// Represents a hyperlink.
///
/// See: [`IDetailsLink`]
///
#[doc = include_str!("./bindings_docs/IDetailsLink.md")]
#[implement(IDetailsLink, IDetailsData)]
pub struct DetailsLink {
    text: HSTRING,
    link: windows::Foundation::Uri,
}

/// Builder for [`DetailsLink`].
pub struct DetailsLinkBuilder {
    text: Option<HSTRING>,
    link: windows::Foundation::Uri,
}

impl DetailsLinkBuilder {
    /// Creates a new builder.
    pub fn new(link: windows::Foundation::Uri) -> Self {
        DetailsLinkBuilder { text: None, link }
    }

    /// Try to create a new builder from a string link.
    pub fn try_new(link: impl Into<HSTRING>) -> Result<Self> {
        let uri = windows::Foundation::Uri::CreateUri(&link.into())?;
        Ok(DetailsLinkBuilder {
            text: None,
            link: uri,
        })
    }

    /// Sets the display text for the link automatically based on the URI, if not already set.
    pub fn auto_text(mut self) -> Result<Self> {
        if self.text.is_none() {
            self.text = Some(self.link.ToString()?);
        }
        Ok(self)
    }

    /// Sets the display text for the link.
    pub fn text(mut self, text: impl Into<HSTRING>) -> Self {
        self.text = Some(text.into());
        self
    }
}

impl ComBuilder for DetailsLinkBuilder {
    type Output = DetailsLink;
    fn build_unmanaged(self) -> DetailsLink {
        DetailsLink {
            text: self.text.unwrap_or_else(|| HSTRING::new()),
            link: self.link,
        }
    }
}

impl IDetailsData_Impl for DetailsLink_Impl {}

impl IDetailsLink_Impl for DetailsLink_Impl {
    fn Text(&self) -> Result<windows_core::HSTRING> {
        Ok(self.text.clone())
    }

    fn Link(&self) -> Result<windows::Foundation::Uri> {
        Ok(self.link.clone())
    }
}

// TODO: Microsoft has changed from `IDetailsCommand` to `IDetailsCommands`, yet unreleased.

/// Represents a command that can be executed from details tab.
///
/// See: [`IDetailsCommand`]
///
#[doc = include_str!("./bindings_docs/IDetailsCommand.md")]
#[implement(IDetailsCommand, IDetailsData)]
pub struct DetailsCommand {
    command: AgileReference<ICommand>,
}

impl DetailsCommand {
    /// Creates a new unmanaged instance of `DetailsCommand` with the specified command.
    pub fn try_new_unmanaged(command: ICommand) -> Result<Self> {
        let command = AgileReference::new(&command)?;
        Ok(DetailsCommand { command })
    }

    /// Creates a new reference-counted COM object for `DetailsCommand` with the specified command.
    pub fn try_new(command: ICommand) -> Result<ComObject<Self>> {
        Self::try_new_unmanaged(command).map(Into::into)
    }

    /// Creates a new unmanaged instance of `DetailsCommand` with the specified command.
    pub fn new_unmanaged(command: AgileReference<ICommand>) -> Self {
        DetailsCommand { command }
    }

    /// Creates a new reference-counted COM object for `DetailsCommand` with the specified command.
    pub fn new(command: AgileReference<ICommand>) -> ComObject<Self> {
        Self::new_unmanaged(command).into()
    }
}

impl IDetailsData_Impl for DetailsCommand_Impl {}

impl IDetailsCommand_Impl for DetailsCommand_Impl {
    fn Command(&self) -> Result<ICommand> {
        self.command.resolve()
    }
}

/// Represents a separator in details tab.
///
/// See: [`IDetailsSeparator`]
///
/// Details contents lay out vertically,
/// this separator could be used to visually separate different sections of details.
///
#[doc = include_str!("./bindings_docs/IDetailsSeparator.md")]
#[implement(IDetailsSeparator, IDetailsData)]
pub struct DetailsSeparator;

impl DetailsSeparator {
    pub fn new() -> ComObject<Self> {
        ComObject::new(Self)
    }
}

impl IDetailsData_Impl for DetailsSeparator_Impl {}

impl IDetailsSeparator_Impl for DetailsSeparator_Impl {}

/// Represents a collection of all possible detail data types.
pub enum DetailsData {
    Tags(ComObject<DetailsTags>),
    Link(ComObject<DetailsLink>),
    Command(ComObject<DetailsCommand>),
    Separator(ComObject<DetailsSeparator>),
}

impl From<&DetailsData> for IDetailsData {
    fn from(data: &DetailsData) -> Self {
        match data {
            DetailsData::Tags(tags) => tags.to_interface(),
            DetailsData::Link(link) => link.to_interface(),
            DetailsData::Command(command) => command.to_interface(),
            DetailsData::Separator(separator) => separator.to_interface(),
        }
    }
}

/// Represents a detail metadata that can be used in [`Details`].
///
/// See: [`IDetailsElement`]
///
#[doc = include_str!("./bindings_docs/IDetailsElement.md")]
#[implement(IDetailsElement)]
pub struct DetailsElement {
    key: HSTRING,
    data: DetailsData,
}

impl DetailsElement {
    /// Creates a new unmanaged instance of `DetailsElement` with the specified key and data.
    pub fn new_unmanaged(key: impl Into<HSTRING>, data: DetailsData) -> Self {
        DetailsElement {
            key: key.into(),
            data,
        }
    }

    /// Creates a new reference-counted COM object for `DetailsElement` with the specified key and data.
    pub fn new(key: impl Into<HSTRING>, data: DetailsData) -> ComObject<Self> {
        Self::new_unmanaged(key, data).into()
    }
}

impl IDetailsElement_Impl for DetailsElement_Impl {
    fn Key(&self) -> Result<windows_core::HSTRING> {
        Ok(self.key.clone())
    }

    fn Data(&self) -> Result<IDetailsData> {
        Ok((&self.data).into())
    }
}

/// Represents the details tab that can be displayed in a page.
///
/// See: [`IDetails`]
///
#[doc = include_str!("./bindings_docs/IDetails.md")]
#[implement(IDetails)]
pub struct Details {
    hero_image: Option<ComObject<IconInfo>>,
    title: HSTRING,
    body: HSTRING,
    metadata: Vec<ComObject<DetailsElement>>,
}

/// Builder for [`Details`].
pub struct DetailsBuilder {
    hero_image: Option<ComObject<IconInfo>>,
    title: Option<HSTRING>,
    body: Option<HSTRING>,
    metadata: Vec<ComObject<DetailsElement>>,
}

impl DetailsBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        DetailsBuilder {
            hero_image: None,
            title: None,
            body: None,
            metadata: Vec::new(),
        }
    }

    /// Sets the hero image for the details.
    pub fn hero_image(mut self, hero_image: ComObject<IconInfo>) -> Self {
        self.hero_image = Some(hero_image);
        self
    }

    /// Sets the title for the details.
    pub fn title(mut self, title: impl Into<HSTRING>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the body text for the details.
    pub fn body(mut self, body: impl Into<HSTRING>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets the metadata (elements) for the details.
    pub fn metadata(mut self, metadata: Vec<ComObject<DetailsElement>>) -> Self {
        self.metadata = metadata;
        self
    }

    /// Adds a metadata (element) to the details.
    pub fn add_metadata(mut self, element: ComObject<DetailsElement>) -> Self {
        self.metadata.push(element);
        self
    }

    /// Adds a metadata (element) without a key to the details.
    pub fn add_unnamed_metadata(mut self, data: DetailsData) -> Self {
        let element = DetailsElement::new_unmanaged(HSTRING::new(), data);
        self.metadata.push(ComObject::new(element));
        self
    }
}

impl ComBuilder for DetailsBuilder {
    type Output = Details;
    fn build_unmanaged(self) -> Details {
        Details {
            hero_image: self.hero_image,
            title: self.title.unwrap_or_else(|| HSTRING::new()),
            body: self.body.unwrap_or_else(|| HSTRING::new()),
            metadata: self.metadata,
        }
    }
}

impl Default for DetailsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IDetails_Impl for Details_Impl {
    fn HeroImage(&self) -> Result<crate::bindings::IIconInfo> {
        self.hero_image
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or_empty()
    }

    fn Title(&self) -> Result<windows_core::HSTRING> {
        Ok(self.title.clone())
    }

    fn Body(&self) -> Result<windows_core::HSTRING> {
        Ok(self.body.clone())
    }

    fn Metadata(&self) -> Result<windows_core::Array<IDetailsElement>> {
        Ok(map_array(&self.metadata, |x| {
            x.to_interface::<IDetailsElement>().into()
        }))
    }
}

const _: () = assert_send_sync::<DetailsData>();
const _: () = assert_send_sync::<ComObject<Details>>();
