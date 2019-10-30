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
//! features = ["ttf"]
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

/// `widgets` is a core rendering library used by `Pushrod`, containing the default set of `Widget`s.
pub mod widgets;

/// `render` is the core rendering/event loop portion of `Pushrod`.
pub mod render;
