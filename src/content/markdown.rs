//! Markdown content that can be used to display formatted text.

use crate::bindings::*;
use crate::notify::*;
use crate::utils::assert_send_sync;
use windows_core::{Event, HSTRING, IInspectable, IUnknownImpl as _, implement};
use windows_core::ComObject;

/// Markdown content that can be used to display formatted text.
/// 
/// See [`MarkdownContent_Impl`] for field accessors.
///
#[doc = include_str!("../bindings_docs/IMarkdownContent.md")]
#[implement(IMarkdownContent, IContent, INotifyPropChanged)]
pub struct MarkdownContent {
    body: NotifyLock<HSTRING>,
    event: PropChangedEventHandler,
}

impl MarkdownContent {
    /// Creates an unmanaged instance of `MarkdownContent` with the specified body.
    pub fn new_unmanaged(body: impl Into<HSTRING>) -> Self {
        let body = NotifyLock::new(body.into());
        let event = Event::new();
        MarkdownContent { body, event }
    }

    /// Creates a reference-counted COM object for `MarkdownContent` with the specified body.
    pub fn new(body: impl Into<HSTRING>) -> ComObject<Self> {
        let content = Self::new_unmanaged(body.into());
        ComObject::new(content)
    }
}

impl MarkdownContent_Impl {
    fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    /// Readonly access to [`IMarkdownContent::Body`].
    ///
    #[doc = include_str!("../bindings_docs/IMarkdownContent/Body.md")]
    pub fn body(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.body.read()
    }

    /// Mutable access to [`IMarkdownContent::Body`].
    ///
    #[doc = include_str!("../bindings_docs/IMarkdownContent/Body.md")]
    ///
    /// Notifies the host about the property change when dropping the guard.
    pub fn body_mut(&self) -> windows_core::Result<NotifyLockWriteGuard<'_, HSTRING>> {
        self.body.write(|| self.emit_self_prop_changed("Body"))
    }
}

impl IMarkdownContent_Impl for MarkdownContent_Impl {
    fn Body(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.body.read()?.clone())
    }
}

impl IContent_Impl for MarkdownContent_Impl {}
impl INotifyPropChanged_Impl for MarkdownContent_Impl {
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
        self.event.add(handler.ok()?)
    }

    fn RemovePropChanged(&self, token: i64) -> windows_core::Result<()> {
        self.event.remove(token);
        Ok(())
    }
}

const _: () = assert_send_sync::<ComObject<MarkdownContent>>();
