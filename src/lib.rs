// Pushrod Widget Library
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

//! `pushrod` is a GUI library for Rust.
//!
//! # Dependencies
//! Pushrod uses the following dependency:
//! ```ignore
//! [dependencies.sdl2]
//! version = "^0.32"
//! features = ["ttf", "image"]
//! ```
//!
//! To use the crate in your project, add the following dependencies:
//! ```ignore
//! [dependencies]
//! rust-pushrod = "^0.4"
//! ```
//! This will pull in the latest version.
//!
//! # Core Components
//! `pushrod::render` is the _core_ rendering components, containing the `Widget` base class, and
//! drawing loop logic.
//! `pushrod::widgets` is the extended `Widget` component library.

#[macro_use]

/// These macros are used and shared by all components in the `Pushrod` library.
mod macros {
    #[macro_export]
    /// This macro is used by `Widget` implementations, which auto-injects getter code for the
    /// `Widget`'s properties.  Since all `Widget` implementations share these functions, and must
    /// implement them locally, this `macro` serves as a quick way to implement the same reused code
    /// automatically.
    macro_rules! default_widget_properties {
        () => {
            /// This function is a macro-created getter function that returns the `Widget`'s configuration
            /// object as a borrowed mutable reference.  This code is auto-generated using the
            /// `default_widget_properties!()` macro.
            fn get_config(&mut self) -> &mut WidgetConfig {
                &mut self.config
            }

            /// This function is a macro-created getter function that returns the `Widget`'s system
            /// properties as a borrowed mutable reference.  This code is auto-generated using the
            /// `default_widget_properties!()` macro.
            fn get_system_properties(&mut self) -> &mut HashMap<i32, String> {
                &mut self.system_properties
            }

            /// This function is a macro-created getter function that returns the `Widget`'s
            /// `CallbackRegistry` object as a borrowed mutable reference.  This code is auto-generated
            /// using the `default_widget_properties!()` macro.
            fn get_callbacks(&mut self) -> &mut CallbackRegistry {
                &mut self.callback_registry
            }
        }
    }

    /// This macro is used by `Widget` implementations, which auto-injects callback functions.  It
    /// overrides standard functions so that mouse events, tick events, keyboard events, etc. all are
    /// routed through the `get_callbacks()` property function.
    macro_rules! default_widget_callbacks {
        () => {
            /// This function is a macro-created tick callback override, created by the
            /// `default_widget_callbacks!()` macro.
            fn tick_callback(&mut self, _widgets: &[WidgetContainer]) {
                if self.get_callbacks().has_on_tick() {
                    if let Some(mut cb) = self.get_callbacks().on_tick.take() {
                        cb(self, _widgets);
                        self.get_callbacks().on_tick = Some(cb);
                    }
                }
            }

            /// This function is a macro-created mouse entered callback override, created by the
            /// `default_widget_callbacks!()` macro.
            fn mouse_entered_callback(&mut self, _widgets: &[WidgetContainer]) {
                if self.get_callbacks().has_on_mouse_entered() {
                    if let Some(mut cb) = self.get_callbacks().on_mouse_entered.take() {
                        cb(self, _widgets);
                        self.get_callbacks().on_mouse_entered = Some(cb);
                    }
                }
            }

            /// This function is a macro-created mouse exited callback override, created by the
            /// `default_widget_callbacks!()` macro.
            fn mouse_exited_callback(&mut self, _widgets: &[WidgetContainer]) {
                if self.get_callbacks().has_on_mouse_exited() {
                    if let Some(mut cb) = self.get_callbacks().on_mouse_exited.take() {
                        cb(self, _widgets);
                        self.get_callbacks().on_mouse_exited = Some(cb);
                    }
                }
            }

            /// This function is a macro-created mouse moved callback override, created by the
            /// `default_widget_callbacks!()` macro.
            fn mouse_moved_callback(&mut self, _widgets: &[WidgetContainer], _points: Points) {
                if self.get_callbacks().has_on_mouse_moved() {
                    if let Some(mut cb) = self.get_callbacks().on_mouse_moved.take() {
                        cb(self, _widgets, _points);
                        self.get_callbacks().on_mouse_moved = Some(cb);
                    }
                }
            }

            /// This function is a macro-created mouse scrolled callback override, created by the
            /// `default_widget_callbacks!()` macro.
            fn mouse_scrolled_callback(&mut self, _widgets: &[WidgetContainer], _points: Points) {
                if self.get_callbacks().has_on_mouse_scrolled() {
                    if let Some(mut cb) = self.get_callbacks().on_mouse_scrolled.take() {
                        cb(self, _widgets, _points);
                        self.get_callbacks().on_mouse_scrolled = Some(cb);
                    }
                }
            }

            /// This function is a macro-created mouse scrolled callback override, created by the
            /// `default_widget_callbacks!()` macro.
            fn button_clicked_callback(&mut self, _widgets: &[WidgetContainer], _button: u8, _clicks: u8, _state: bool) {
                if self.get_callbacks().has_on_mouse_clicked() {
                    if let Some(mut cb) = self.get_callbacks().on_mouse_clicked.take() {
                        cb(self, _widgets, _button, _clicks, _state);
                        self.get_callbacks().on_mouse_clicked = Some(cb);
                    }
                }
            }
        }
    }
}

/// `widgets` is a core rendering library used by `Pushrod`, containing the default set of `Widget`s.
pub mod widgets;

/// `render` is the core rendering/event loop portion of `Pushrod`.
pub mod render;

/// `layouts` is the core layout managers included with `Pushrod`.
pub mod layouts;
