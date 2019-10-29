// Pushrod Rendering Library
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

//! `pushrod-render` is a core rendering library used by `Pushrod`.
//!
//! For reference, see the [rust-pushrod project at this link](https://www.github.com/KenSuenobu/rust-pushrod/).
//!
//! # Dependencies
//! Pushrod uses the following dependencies:
//! ```ignore
//! [dependencies]
//! sdl2 = "^0.32"
//! ```
//!
//! To use the crate in your project, add the following dependencies:
//! ```ignore
//! [dependencies]
//! rust-pushrod-render = "*"
//! ```
//! This will pull in the latest version.
//!
//! # Core Components
//! `pushrod::render` is the _core_ rendering components, containing the widget config, and
//! drawing loop logic.

/// The rendering logic library.
pub mod render;
