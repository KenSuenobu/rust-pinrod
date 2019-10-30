# Pushrod Render Change List

This documents the list of changes for each release.  Newest releases are listed
first.

## 0.1.10

- Modified canvas so that it pulls the canvas from `software()`

## 0.1.9

- Stored `widget_id` in the `WidgetContainer` object.
- Added `draw` function to walk the `WidgetContainer` tree and draw objects in order of `parent_id`.
- Added `get_children_of` to get a list of IDs that are a child of a given parent ID.
- Further improvements to draw loop code, smaller and much easier to use.
- Added `enable`, `disable`, and `is_enabled` flags to enable or disable `Widget`s.
- Added `hide`, `show`, and `is_hidden` flags to hide or show `Widget`s.
- Modified `find_widget` to skip over objects that are hidden.
- Moved to `rust-pushrod` project.

## 0.1.8

- Used clippy to find lintable items, corrected using slices rather than Vec so that objects aren't copied
- Used ".." operator in event engine as suggested by lint
- Added `#[derive(Default)]` to classes where possible
- Converted complex callback types into `pub type` definitions

## 0.1.7

- Corrected callbacks to call mouse scrolled on scroll callbacks instead of mouse moved
- Enabled mouse down/up callbacks in event loop
- Fixed callback macro in `Widget` so that it calls the correct callbacks (was only calling move and enter/exit.)

## 0.1.6

- Added missing callback for `mouse_scrolled`, changed signature to `i32` instead of `u8`.
- Added callbacks for most actions in `Engine` through the `WidgetCache` so that the cache can be sent from there.
- Added `setup` call to set up the base widget in the `Engine` (this will be fixed)
- Added `BaseWidget` to the demo with callback demonstrations (via debugging)
- Set up `BaseWidget` to demonstrate enter/exit callbacks.
- Added documentation to `Engine` object.

## 0.1.5

- Wrapped `Widget` in `WidgetContainer` in a `RefCell` so that `Widget` callbacks can modify other `Widget` objects.
- Callback passes reference to the `WidgetContainer` store to on_tick.
- Renamed `default_widget_property_impl` to `default_widget_properties`, which makes more sense.
- Added `default_widget_callbacks` macro to implement defaults for callbacks.
- Updated `Widget` so that it can control calling callbacks when appropriate.
- Added `tick_callback` to macro and `Widget`.
- Added callbacks for mouse move, exit, enter, scroll, and button click.

## 0.1.4

- Added `tick` function to `WidgetCache` so that it calls the `tick` callback in each `Widget`.
- Added `CallbackRegistry` in order for `Widget` actions to perform callbacks.
- Added `get_container_by_id` to get a `WidgetContinaer` object by its ID.
- Added `get_container_by_name` to get a `WidgetContainer` object by its name.

## 0.1.3

- Added `get_drawing_area` method to `Widget`.
- Added use of `get_drawing_area` method to `BaseWidget`.
- Added `set_origin` and `set_size` to allow for a `Widget` to be positioned and resized.
- Added hover color.
- Added `draw_loop` to the Engine.
- Removed "origin" requirement from `Widget` `new()` function.
- Added top level base widget in Engine to the cache.
- Removed color wheel cycling in demo.
- Added `invalidation` use in `draw_loop`

