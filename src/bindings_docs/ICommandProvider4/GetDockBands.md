The **GetDockBands** method returns the dock bands provided by this command provider. Dock bands are strips of items that appear on the Dock toolbar. Each [`ICommandItem`] returned is treated as one atomic band. If the command on an item is an [`IListPage`], then all items on that page are rendered as one band.

## Returns

An [`ICommandItem`] array that contains the dock bands provided by this command provider.
