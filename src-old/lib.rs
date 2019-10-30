// Pushrod
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

//! Pushrod is a Cross Platform UI Widget Library for Rust.
//!
//! It is intended to be lightweight, easy to use, and easy to understand.  Pushrod draws
//! inspiration from 16-bit GUI-based systems and other GUI libraries over the years.
//!
//! # Dependencies
//! Pushrod uses the following dependencies:
//! ```ignore
//! [dependencies]
//! piston = "^0.42"
//! pistoncore-glfw_window = "^0.49"
//! piston2d-opengl_graphics = "^0.59"
//! piston2d-graphics = "^0.30"
//! gl = "^0.11"
//! find_folder = "^0.3"
//! ```
//!
//! To use the crate in your project, add the following dependencies:
//! ```ignore
//! [dependencies]
//! rust-pushrod = "^0.3"
//! ```
//! This will pull in the latest version in the 0.3.x branch.
//!
//! # Core Components
//! `pushrod::core` is the _core_ library components, representing the main run loop, the callback
//! store mechanism, and the widget store.
//!
//! `pushrod::widget` is the core `Widget` library.
//!
//! # Widgets
//! `Widget` objects are the interactive objects in the GUI.  Several `Widget` classes are
//! provided as a convenience, and this library is growing in size.

/// Main module containing the run loop for the UI components, containers for windows and
/// `Widget` trait objects, and so on.  Contains the core elements required to build
/// a UI.
pub mod core;

/// Widget library used for on-screen UI interaction.  This is a core set of `Widget`
/// objects that are used to allow users to interact with an application.  Contains a core set
/// of widgets that can be used and extended.
pub mod widget;
