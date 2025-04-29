use crate::bindings::*;
use crate::icon::IconInfo;
use windows::core::{ComObject, Result, implement};
use windows_core::HSTRING;
use crate::utils::map_array;

#[implement(ITag)]
pub struct Tag {
    icon: Option<ComObject<IconInfo>>,
    text: HSTRING,
    foreground: Option<Color>,
    background: Option<Color>,
    tooltip: HSTRING,
}

impl ITag_Impl for Tag_Impl {
    fn Icon(&self) -> Result<crate::bindings::IIconInfo> {
        self.icon
            .as_ref()
            .map(|icon| icon.to_interface())
            .ok_or(windows_core::Error::empty())
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
    command: ICommand,
}

impl IDetailsData_Impl for DetailsCommand_Impl {}

impl IDetailsCommand_Impl for DetailsCommand_Impl {
    fn Command(&self) -> Result<ICommand> {
        Ok(self.command.clone())
    }
}

#[implement(IDetailsSeparator, IDetailsData)]
pub struct DetailsSeparator;

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
        Ok(map_array(&self.metadata, |x| x.to_interface::<IDetailsElement>().into()))
    }
}
