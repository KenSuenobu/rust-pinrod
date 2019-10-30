// Widget Module
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

/// Base component and UI Components (Widget) library.  These components are used for on-screen interactions
/// between the user and the application.
pub mod widget;

/// Box component: draws a box on the screen with adjustable border color and width.
pub mod box_widget;

/// Timer component: triggers a callback after a certain amount of time.
pub mod timer_widget;

/// Text component: draws text on the screen with an adjustable text, font size, color, and font name.
pub mod text_widget;

/// Image component: draws an image on the screen in `png`, `jpg` or `gif` formats.
pub mod image_widget;

/// Push Button component: draws a clickable box on the screen, triggering an `on_clicked` callback
/// when appropriate.
pub mod push_button_widget;

/// Toggle Button component: draws a clickable box on the screen, triggering an `on_selected` callback
/// where appropriate.
pub mod toggle_button_widget;

/// Progress component: draws a progress meter widget.
pub mod progress_widget;

/// Checkbox component: draws a selectable checkbox with text.
pub mod checkbox_widget;

/// Radio button component: only allows a single item to be selected in a group.
pub mod radio_button_widget;

/// Image button component: draws an image inside a push button widget.
pub mod image_button_widget;

/// `Configurable` definition, used by `Widget` objects to store configuration settings.
pub mod config;
