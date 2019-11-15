// Pushrod Rendering Library
// Extensible Layout Library
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

use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::PaddingConstraint;

/// This is a structure that describes the position of a `Widget` within its `Layout`.  `X` and
/// `Y` coordinates are not given as physical positions on the screen, rather, their position in the
/// `Layout` matrix.
pub struct LayoutPosition {
    pub x: i32,
    pub y: i32,
}

/// This is a `Layout` trait that is used by the `Engine` service, which stores a list of `Widget`s,
/// their positions (based on matrix coordinates), and an entry point to trigger the layout compute
/// action.
pub trait Layout {
    /// Adds a `Widget` by ID to the `Layout` manager, given its `LayoutPosition`, as a position
    /// marker in the manager.
    fn add_widget(&mut self, _widget_id: i32, _widget_position: LayoutPosition);

    /// Changes the `PaddingConstraint` for this `Layout`.
    fn set_padding(&mut self, padding: PaddingConstraint);

    /// Retrieves the current `PaddingConstraint`.
    fn get_padding(&self) -> PaddingConstraint;

    /// Performs a layout, applying the `WidgetContainer` list at the time, so that referenced
    /// `Widget`s can be adjusted as necessary.
    fn do_layout(&mut self, _widgets: &[WidgetContainer]);

    /// Indicates whether or not the `Layout` needs to have `do_layout` re-run.  This is generally
    /// needed when the `LayoutPosition` changes, or when `PaddingConstraint`s change.
    fn needs_layout(&self) -> bool;
}
