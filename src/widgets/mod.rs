// Pushrod Widgets Library
// Core Widgets
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]

/// This is a `TextWidget`, which draws text in a clipped area.
pub mod text_widget;

/// This is an `ImageWidget`, which draws an Image in a clipped area.
pub mod image_widget;

/// This is a `ProgressWidget`, which draws a progress bar.
pub mod progress_widget;

/// This is a `TimerWidget`, which times out after a specified duration, triggering a callback
/// after the timeout is exceeded.
pub mod timer_widget;

/// This is a `PushButtonWidget`, which draws text in a `Widget` that can be clicked.  It triggers an
/// `on_click` callback when the button is clicked.
pub mod push_button_widget;

/// This is a `ToggleButtonWidget`, which acts similar to a `PushButtonWidget` except that it triggers
/// `on_toggle` callbacks when the state changes.
pub mod toggle_button_widget;

/// This is an `ImageButtonWidget`, which acts like a `PushButtonWidget`, drawing an image on the left-hand
/// side of the bounds of the `Widget`, then the text next to it, justified left.
pub mod image_button_widget;

/// This is a `CheckboxWidget`, which acts similar to a `ToggleButtonWidget`, but does not fill the
/// box with a black/white color on select.  Rather, it enables/disables a checkbox to indicate a
/// selected option.
pub mod checkbox_widget;

/// This is a `SliderWidget` that displays a slider in a movable area, which changes values from min to
/// max bounds.
pub mod slider_widget;

/// This is a `GridWidget` that contains a number of `Widget`s that can be repositioned and snapped to
/// a grid coordinate.
pub mod grid_widget;

/// This is a `ListWidget` that displays a list of items in a selectable box.
pub mod list_widget;

/// This is a `TileWidget` that displays an image and some text below it, with a display change when
/// the widget is hovered over by a mouse enter event.  This is mainly used in conjunction with the
/// `ToolbarWidget`, which contains a list of `TileWidget`s
pub mod tile_widget;

/// This is a `TabBarWidget` that provides a selectable set of tabs along the top-most part of a
/// selectable area containing multiple pages of layouts.  It provides a callback to indicate which
/// page has been selected.
pub mod tab_bar_widget;

/// This is a `ViewportWidget` that provides a scrollable view within a container.  It provides
/// a callback indicating when the visible viewport area is altered.
pub mod viewport_widget;