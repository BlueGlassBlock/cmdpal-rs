use crate::bindings::*;
use crate::details::Details;
use crate::notify::*;
use crate::utils::{map_array, ComBuilder, OkOrEmpty};
use windows::Foundation::TypedEventHandler;
use windows::core::{ComObject, Event, IInspectable, IUnknownImpl as _, Result, implement};

#[implement(IContentPage, IPage, ICommand, INotifyPropChanged, INotifyItemsChanged)]
pub struct ContentPage {
    commands: NotifyLock<Vec<IContextItem>>,
    contents: NotifyLock<Vec<IContent>>,
    details: NotifyLock<Option<ComObject<Details>>>,
    pub base: ComObject<super::BasePage>,
    item_event: Event<TypedEventHandler<IInspectable, IItemsChangedEventArgs>>,
}

pub struct ContentPageBuilder {
    base: ComObject<super::BasePage>,
    commands: Vec<IContextItem>,
    contents: Vec<IContent>,
    details: Option<ComObject<Details>>,
}

impl ContentPageBuilder {
    pub fn new(base: ComObject<super::BasePage>) -> Self {
        ContentPageBuilder {
            base,
            commands: Vec::new(),
            contents: Vec::new(),
            details: None,
        }
    }

    pub fn commands(mut self, commands: Vec<IContextItem>) -> Self {
        self.commands = commands;
        self
    }

    pub fn add_command(mut self, command: IContextItem) -> Self {
        self.commands.push(command);
        self
    }

    pub fn contents(mut self, contents: Vec<IContent>) -> Self {
        self.contents = contents;
        self
    }

    pub fn add_content(mut self, content: IContent) -> Self {
        self.contents.push(content);
        self
    }

    pub fn details(mut self, details: ComObject<Details>) -> Self {
        self.details = Some(details);
        self
    }
}

impl ComBuilder<ContentPage> for ContentPageBuilder {
    fn build_unmanaged(self) -> ContentPage {
        ContentPage {
            base: self.base,
            commands: NotifyLock::new(self.commands),
            contents: NotifyLock::new(self.contents),
            details: NotifyLock::new(self.details),
            item_event: Event::new(),
        }
    }
}

impl ContentPage_Impl {
    pub fn emit_items_changed(&self, sender: IInspectable, total_items: i32) {
        let args: IItemsChangedEventArgs = ItemsChangedEventArgs(total_items).into();
        self.item_event
            .call(|handler| handler.Invoke(&sender, &args));
    }

    fn emit_self_items_changed(&self, total_items: i32) {
        self.emit_items_changed(self.to_interface(), total_items);
    }

    pub fn commands(&self) -> Result<NotifyLockReadGuard<'_, Vec<IContextItem>>> {
        self.commands.read()
    }

    pub fn commands_mut(&self) -> Result<NotifyLockWriteGuard<'_, Vec<IContextItem>, impl Fn()>> {
        self.commands.write(|| self.emit_self_items_changed(-1))
    }

    pub fn contents(&self) -> Result<NotifyLockReadGuard<'_, Vec<IContent>>> {
        self.contents.read()
    }

    pub fn contents_mut(&self) -> Result<NotifyLockWriteGuard<'_, Vec<IContent>, impl Fn()>> {
        self.contents.write(|| self.emit_self_items_changed(-1))
    }

    pub fn details(&self) -> Result<NotifyLockReadGuard<'_, Option<ComObject<Details>>>> {
        self.details.read()
    }

    pub fn details_mut(
        &self,
    ) -> Result<NotifyLockWriteGuard<'_, Option<ComObject<Details>>, impl Fn()>> {
        self.details.write(|| {
            self.base
                .command
                .emit_prop_changed(self.to_interface(), "Details")
        })
    }
}

impl IContentPage_Impl for ContentPage_Impl {
    fn Commands(&self) -> windows_core::Result<windows_core::Array<IContextItem>> {
        Ok(map_array(&self.commands.read()?, |x| x.clone().into()))
    }

    fn GetContent(&self) -> windows_core::Result<windows_core::Array<IContent>> {
        Ok(map_array(&self.contents.read()?, |x| x.clone().into()))
    }

    fn Details(&self) -> windows_core::Result<IDetails> {
        self.details
            .read()?
            .clone()
            .map(|x| x.to_interface())
            .ok_or_empty()
    }
}

impl INotifyItemsChanged_Impl for ContentPage_Impl {
    fn ItemsChanged(
        &self,
        handler: windows_core::Ref<
            '_,
            windows::Foundation::TypedEventHandler<
                windows_core::IInspectable,
                IItemsChangedEventArgs,
            >,
        >,
    ) -> windows_core::Result<i64> {
        self.item_event.add(handler.ok()?)
    }

    fn RemoveItemsChanged(&self, token: i64) -> windows_core::Result<()> {
        self.item_event.remove(token);
        Ok(())
    }
}

impl IPage_Impl for ContentPage_Impl {
    ambassador_impl_IPage_Impl!(
        body_struct(<>, ComObject<super::BasePage>, base)
    );
}

impl ICommand_Impl for ContentPage_Impl {
    ambassador_impl_ICommand_Impl!(
        body_struct(<>, ComObject<super::BasePage>, (base.command), (base.command), (base.command))
    );
}

impl INotifyPropChanged_Impl for ContentPage_Impl {
    ambassador_impl_INotifyPropChanged_Impl!(
        body_struct(<>, ComObject<super::BasePage>, base)
    );
}
