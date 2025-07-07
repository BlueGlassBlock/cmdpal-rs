use crate::bindings::*;
use crate::notify::*;
use crate::utils::assert_send_sync;
use windows::core::{Event, HSTRING, IInspectable, IUnknownImpl as _, implement};
use windows_core::ComObject;

#[implement(IMarkdownContent, IContent, INotifyPropChanged)]
pub struct MarkdownContent {
    body: NotifyLock<HSTRING>,
    event: PropChangedEventHandler,
}

impl MarkdownContent {
    pub fn new_unmanaged(body: HSTRING) -> Self {
        let body = NotifyLock::new(body);
        let event = Event::new();
        MarkdownContent { body, event }
    }

    pub fn new(body: HSTRING) -> ComObject<Self> {
        let content = Self::new_unmanaged(body);
        ComObject::new(content)
    }
}

impl MarkdownContent_Impl {
    fn emit_self_prop_changed(&self, prop: &str) {
        let sender: IInspectable = self.to_interface();
        let arg: IPropChangedEventArgs = PropChangedEventArgs(prop.into()).into();
        self.event.call(|handler| handler.Invoke(&sender, &arg));
    }

    pub fn body(&self) -> windows_core::Result<NotifyLockReadGuard<'_, HSTRING>> {
        self.body.read()
    }

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
