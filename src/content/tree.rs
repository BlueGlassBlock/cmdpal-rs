//! Tree content that can be used to display nested content.

use super::Content;
use crate::notify::*;
use crate::utils::{ComBuilder, assert_send_sync};
use crate::{bindings::*, utils::map_array};
use windows_core::{Event, IInspectable, IUnknownImpl as _, Result, implement};
use windows_core::ComObject;

/// Tree content that can be used to display nested content.
/// 
/// See [`TreeContent_Impl`] for field accessors.
///
#[doc = include_str!("../bindings_docs/ITreeContent.md")]
#[implement(ITreeContent, IContent, INotifyPropChanged, INotifyItemsChanged)]
pub struct TreeContent {
    root: NotifyLock<Content>,
    children: NotifyLock<Vec<Content>>,
    prop_event: PropChangedEventHandler,
    item_event: ItemsChangedEventHandler,
}

/// Builder for [`TreeContent`].
pub struct TreeContentBuilder {
    root: Content,
    children: Vec<Content>,
}

impl TreeContentBuilder {
    /// Creates a new builder with the specified root content.
    pub fn new(root: Content) -> Self {
        TreeContentBuilder {
            root,
            children: Vec::new(),
        }
    }

    /// Sets the children of the tree node.
    pub fn children(mut self, children: Vec<Content>) -> Self {
        self.children = children;
        self
    }

    /// Adds a child to the tree node.
    pub fn add_child(mut self, child: Content) -> Self {
        self.children.push(child);
        self
    }
}

impl ComBuilder for TreeContentBuilder {
    type Output = TreeContent;
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

    /// Readonly access to [`ITreeContent::RootContent`].
    /// 
    #[doc = include_str!("../bindings_docs/ITreeContent/RootContent.md")]
    pub fn root(&self) -> Result<NotifyLockReadGuard<'_, Content>> {
        self.root.read()
    }

    /// Mutable access to [`ITreeContent::RootContent`].
    /// 
    #[doc = include_str!("../bindings_docs/ITreeContent/RootContent.md")]
    /// 
    /// Notifies the host about the property change when dropping the guard.
    pub fn root_mut(&self) -> Result<NotifyLockWriteGuard<'_, Content>> {
        self.root
            .write(|| self.emit_self_prop_changed("RootContent"))
    }

    /// Readonly access to [`ITreeContent::GetChildren`].
    ///
    #[doc = include_str!("../bindings_docs/ITreeContent/GetChildren.md")]
    pub fn children(&self) -> Result<NotifyLockReadGuard<'_, Vec<Content>>> {
        self.children.read()
    }

    /// Mutable access to [`ITreeContent::GetChildren`].
    ///
    #[doc = include_str!("../bindings_docs/ITreeContent/GetChildren.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn children_mut(&self) -> Result<NotifyLockWriteGuard<'_, Vec<Content>, usize>> {
        self.children
            .write_with_peek(|v| v.len(), |len| self.emit_self_items_changed(len as i32))
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
