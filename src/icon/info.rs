use windows::core::{implement, ComObject};
use crate::bindings::*;
use super::data::IconData;



#[implement(IIconInfo)]
pub struct IconInfo {
    pub light: ComObject<IconData>,
    pub dark: ComObject<IconData>,
}

impl IIconInfo_Impl for IconInfo_Impl {
    fn Dark(&self) -> windows_core::Result<IIconData> {
        Ok(self.dark.to_interface())
    }

    fn Light(&self) -> windows_core::Result<IIconData> {
        Ok(self.light.to_interface())
    }
}

impl From<IconData> for IconInfo {
    fn from(value: IconData) -> Self {
        let obj = ComObject::new(value);
        IconInfo {
            light: obj.clone(),
            dark: obj,
        }
    }
}