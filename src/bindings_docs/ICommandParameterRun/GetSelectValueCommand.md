Returns the command used to select a value for this parameter. The host window handle is provided so pickers can display UI relative to the Command Palette window.

## Parameters

*hostHwnd* **UInt64**

The handle of the host window.

## Returns

An [`ICommand`] used to select a value. If the command is an [`IInvokableCommand`], it is rendered as a button. If the command is an [`IListPage`], it is rendered as a text box that filters the list.
