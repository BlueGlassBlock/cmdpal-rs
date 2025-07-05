use super::Content;
use crate::notify::*;
use crate::utils::{assert_send_sync, ComBuilder};
use crate::{bindings::*, utils::map_array};
use windows::core::{Event, IInspectable, IUnknownImpl as _, Result, implement};
use windows_core::ComObject;

#[implement(ITreeContent, IContent, INotifyPropChanged, INotifyItemsChanged)]
pub struct TreeContent {
    root: NotifyLock<Content>,
    children: NotifyLock<Vec<Content>>,
    prop_event: PropChangedEventHandler,
    item_event: ItemsChangedEventHandler,
}

pub struct TreeContentBuilder {
    root: Content,
    children: Vec<Content>,
}

impl TreeContentBuilder {
    pub fn new(root: Content) -> Self {
        TreeContentBuilder {
            root,
            children: Vec::new(),
        }
    }

    pub fn children(mut self, children: Vec<Content>) -> Self {
        self.children = children;
        self
    }

    pub fn add_child(mut self, child: Content) -> Self {
        self.children.push(child);
        self
    }
}

impl ComBuilder for TreeContentBuilder {
    type Target = TreeContent;
    fn build_unmanaged(self) -> TreeContent {
        TreeContent {
            root: NotifyLock::new(self.root),
            children: NotifyLock::new(self.children),
            prop_event: Event::new(),
            item_event: Event::new(),
        }
    }
}

impl TreeContent_Impl {
    pub(crate) fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.prop_event
            .call(|handler| handler.Invoke(&sender, &arg));
    }

    pub(crate) fn emit_self_items_changed(&self, total_items: i32) {
        let sender: IInspectable = self.to_interface();
        let arg: IItemsChangedEventArgs = ItemsChangedEventArgs(total_items).into();
        self.item_event
            .call(|handler| handler.Invoke(&sender, &arg));
    }

    pub fn root(&self) -> Result<NotifyLockReadGuard<'_, Content>> {
        self.root.read()
    }

    pub fn root_mut(&self) -> Result<NotifyLockWriteGuard<'_, Content, impl Fn()>> {
        self.root
            .write(|| self.emit_self_prop_changed("RootContent"))
    }

    pub fn children(&self) -> Result<NotifyLockReadGuard<'_, Vec<Content>>> {
        self.children.read()
    }

    pub fn children_mut(&self) -> Result<NotifyLockWriteGuard<'_, Vec<Content>, impl Fn()>> {
        self.children.write(|| self.emit_self_items_changed(-1))
    }
}

impl ITreeContent_Impl for TreeContent_Impl {
    fn RootContent(&self) -> windows_core::Result<IContent> {
        self.root.read().map(|x| IContent::from(&*x))
    }

    fn GetChildren(&self) -> windows_core::Result<windows_core::Array<IContent>> {
        let children = self.children.read()?;
        Ok(map_array(&children, |x| Some(x.into())))
    }
}

impl IContent_Impl for TreeContent_Impl {}

impl INotifyPropChanged_Impl for TreeContent_Impl {
    fn PropChanged(
        &self,
        handler: windows_core::Ref<
            '_,
            windows::Foundation::TypedEventHandler<
                windows_core::IInspectable,
                IPropChangedEventArgs,
            >,
        >,
    ) -> windows_core::Result<i64> {
        self.prop_event.add(handler.ok()?)
    }

    fn RemovePropChanged(&self, token: i64) -> windows_core::Result<()> {
        self.prop_event.remove(token);
        Ok(())
    }
}

impl INotifyItemsChanged_Impl for TreeContent_Impl {
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

const _: () = assert_send_sync::<ComObject<TreeContent>>();
