use crate::icon::IconInfo;
use crate::utils::{assert_send_sync, map_array};
use crate::{bindings::*, utils::ComBuilder};
use windows::core::{ComObject, Result, implement};
use windows_core::{AgileReference, HSTRING};

#[implement(ITag)]
pub struct Tag {
    icon: Option<ComObject<IconInfo>>,
    text: HSTRING,
    foreground: Option<Color>,
    background: Option<Color>,
    tooltip: HSTRING,
}

pub struct TagBuilder {
    icon: Option<ComObject<IconInfo>>,
    text: Option<HSTRING>,
    foreground: Option<Color>,
    background: Option<Color>,
    tooltip: Option<HSTRING>,
}

impl TagBuilder {
    pub fn new() -> Self {
        TagBuilder {
            icon: None,
            text: None,
            foreground: None,
            background: None,
            tooltip: None,
        }
    }

    pub fn icon(mut self, icon: ComObject<IconInfo>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn text(mut self, text: impl Into<HSTRING>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<HSTRING>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

impl ComBuilder<Tag> for TagBuilder {
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
            .ok_or(windows::core::Error::empty())
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

#[implement(IDetailsTags, IDetailsData)]
pub struct DetailsTags {
    tags: Vec<ComObject<Tag>>,
}

pub struct DetailsTagsBuilder {
    tags: Vec<ComObject<Tag>>,
}

impl DetailsTagsBuilder {
    pub fn new() -> Self {
        DetailsTagsBuilder { tags: Vec::new() }
    }

    pub fn add_tag(mut self, tag: ComObject<Tag>) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn tags(mut self, tags: Vec<ComObject<Tag>>) -> Self {
        self.tags = tags;
        self
    }
}

impl ComBuilder<DetailsTags> for DetailsTagsBuilder {
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

#[implement(IDetailsLink, IDetailsData)]
pub struct DetailsLink {
    text: HSTRING,
    link: windows::Foundation::Uri,
}

pub struct DetailsLinkBuilder {
    text: Option<HSTRING>,
    link: windows::Foundation::Uri,
}

impl DetailsLinkBuilder {
    pub fn new(link: windows::Foundation::Uri) -> Self {
        DetailsLinkBuilder { text: None, link }
    }

    pub fn text(mut self, text: impl Into<HSTRING>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn link(mut self, link: windows::Foundation::Uri) -> Self {
        self.link = link;
        self
    }
}

impl ComBuilder<DetailsLink> for DetailsLinkBuilder {
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

#[implement(IDetailsCommand, IDetailsData)]
pub struct DetailsCommand {
    command: AgileReference<ICommand>,
}

impl DetailsCommand {
    pub fn try_new_unmanaged(command: ICommand) -> Result<Self> {
        let command = AgileReference::new(&command)?;
        Ok(DetailsCommand { command })
    }

    pub fn try_new(command: ICommand) -> Result<ComObject<Self>> {
        Self::try_new_unmanaged(command).map(Into::into)
    }

    pub fn new_unmanaged(command: AgileReference<ICommand>) -> Self {
        DetailsCommand { command }
    }

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

#[implement(IDetailsSeparator, IDetailsData)]
pub struct DetailsSeparator;

impl DetailsSeparator {
    pub fn new() -> ComObject<Self> {
        ComObject::new(Self)
    }
}

impl IDetailsData_Impl for DetailsSeparator_Impl {}

impl IDetailsSeparator_Impl for DetailsSeparator_Impl {}

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

#[implement(IDetailsElement)]
pub struct DetailsElement {
    key: HSTRING,
    data: DetailsData,
}

impl DetailsElement {
    pub fn new_unmanaged(key: impl Into<HSTRING>, data: DetailsData) -> Self {
        DetailsElement {
            key: key.into(),
            data,
        }
    }

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

#[implement(IDetails)]
pub struct Details {
    hero_image: Option<ComObject<IconInfo>>,
    title: HSTRING,
    body: HSTRING,
    metadata: Vec<ComObject<DetailsElement>>,
}

pub struct DetailsBuilder {
    hero_image: Option<ComObject<IconInfo>>,
    title: Option<HSTRING>,
    body: Option<HSTRING>,
    metadata: Vec<ComObject<DetailsElement>>,
}

impl DetailsBuilder {
    pub fn new() -> Self {
        DetailsBuilder {
            hero_image: None,
            title: None,
            body: None,
            metadata: Vec::new(),
        }
    }

    pub fn hero_image(mut self, hero_image: ComObject<IconInfo>) -> Self {
        self.hero_image = Some(hero_image);
        self
    }

    pub fn title(mut self, title: impl Into<HSTRING>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn body(mut self, body: impl Into<HSTRING>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn metadata(mut self, metadata: Vec<ComObject<DetailsElement>>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_metadata(mut self, element: ComObject<DetailsElement>) -> Self {
        self.metadata.push(element);
        self
    }
}

impl ComBuilder<Details> for DetailsBuilder {
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

impl Details {
    pub fn new(
        hero_image: Option<ComObject<IconInfo>>,
        title: impl Into<HSTRING>,
        body: impl Into<HSTRING>,
        metadata: Vec<ComObject<DetailsElement>>,
    ) -> Self {
        Details {
            hero_image,
            title: title.into(),
            body: body.into(),
            metadata,
        }
    }
}

impl IDetails_Impl for Details_Impl {
    fn HeroImage(&self) -> Result<crate::bindings::IIconInfo> {
        self.hero_image
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or(windows_core::Error::empty())
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