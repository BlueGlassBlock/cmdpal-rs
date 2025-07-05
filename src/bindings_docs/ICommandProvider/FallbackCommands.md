The **FallbackCommands** are special top-level items which allow extensions to have dynamic top-level items which respond to the text the user types on the main list page. These are not shown in the top-level list of commands, but are shown when the user types text in the Command Palette. This allows extensions to provide dynamic commands that are not shown in the top-level list.

## Returns

An array of [`IFallbackCommandItem`] that contains the commands that should be shown in the Command Palette. The commands will be displayed in the order that they are returned by this method.