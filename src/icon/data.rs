use crate::bindings::*;
use windows::Storage::Streams::{IBuffer, IRandomAccessStream, InMemoryRandomAccessStream, RandomAccessStreamReference};
use windows::core::HSTRING;
use windows::core::implement;
use crate::utils::FrozenBuffer;

#[implement(IIconData)]
#[derive(Debug, Clone)]
pub struct IconData {
    icon: HSTRING,
    data: Option<IRandomAccessStream>,
}

impl IIconData_Impl for IconData_Impl {
    fn Icon(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.icon.clone())
    }

    fn Data(
        &self,
    ) -> windows_core::Result<windows::Storage::Streams::IRandomAccessStreamReference> {
        let stream = self.data.as_ref().ok_or(windows_core::Error::empty())?;
        RandomAccessStreamReference::CreateFromStream(stream).map(Into::into)
    }
}

impl From<HSTRING> for IconData {
    fn from(value: HSTRING) -> Self {
        IconData {
            icon: value,
            data: None,
        }
    }
}

impl From<&HSTRING> for IconData {
    fn from(value: &HSTRING) -> Self {
        IconData {
            icon: value.clone(),
            data: None,
        }
    }
}

impl From<&str> for IconData {
    fn from(value: &str) -> Self {
        IconData {
            icon: HSTRING::from(value),
            data: None,
        }
    }
}

impl From<String> for IconData {
    fn from(value: String) -> Self {
        IconData {
            icon: HSTRING::from(value),
            data: None,
        }
    }
}

impl TryFrom<Vec<u8>> for IconData
{
    type Error = windows::core::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let buf: IBuffer = FrozenBuffer::from(value).into();
        let stream = InMemoryRandomAccessStream::new()?;
        let op = stream.WriteAsync(&buf)?;
        op.get()?;
        Ok(IconData {
            icon: HSTRING::from(""),
            data: Some(stream.into()),
        })
    }
}
