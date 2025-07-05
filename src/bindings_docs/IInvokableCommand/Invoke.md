The method called when a user selects a command.

## Parameters

*sender* **Object**

Represents the context of where the command was invoked from. This can be different types depending on where the command is being used:

- [`ICommandProvider::TopLevelCommands`] (and [`ICommandProvider::FallbackCommands`]): *sender* is the [`ICommandItem`] for the top-level command that was invoked.
- [`IListPage::GetItems`]: *sender* is the [`IListItem`] for the list item selected for that command.
- [`ICommandItem::MoreCommands`] (context menus): *sender* is either the [`IListItem`] which the command was attached to for a list page or the [`ICommandItem`] of the top-level command (if this is a context item on a top-level command).
- [`IContentPage::Commands`]: *sender* is the [`IContentPage`] itself.

Using the *sender* parameter can be useful for big lists of items where the actionable information for each item is somewhat the same. One example would be a long list of links. You can implement this as a single [`IInvokableCommand`] that opens a URL based on the *sender* object passed in. Then, each list item would store the URL to open and the title of the link. This creates less overhead for the extension and host to communicate.

## Returns

[`ICommandResult`] object that represents the result of the command invocation. This object can contain information about the success or failure of the command, as well as any additional data that may be relevant to the command's execution.
