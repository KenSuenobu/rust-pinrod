// Pushrod Rendering Library
// Core Rendering Module
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

/// This is a type that defines two points: X and Y coordinates.
pub type Points = Vec<i32>;

/// This is a type that defines two size: width and height.
pub type Size = Vec<u32>;

/// This is the `Engine` that is used to dispatch events from the screen to a corresponding list
/// of `Widget`s in a `Window`.  This is the main event loop.
pub mod engine;

/// This is the `Callbacks` mechanism for each `Widget`, providing a way to perform a function when
/// an action is intercepted (ie. mouse enter, exit, move, etc.)
pub mod callbacks;

/// This is the `Widget` and `BaseWidget` definitions for `Widget` objects to be defined by the
/// `pushrod` project, and other crates that may define or create their own `Widget`s.
pub mod widget;

/// This is a configuration object that stores information about `Widget`s.
pub mod widget_config;

/// This is the caching object that stores a list of `Widget`s that the Pushrod engine manages.
pub mod widget_cache;
