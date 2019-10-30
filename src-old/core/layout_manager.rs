// Layout Manager Trait
// Describes how a Layout Manager behaves
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

use crate::core::point::{Point, Size};

/// This class defines the spacing rules and the padding for the layout manager.  The layout
/// manager must take the rules and honor it during the layout process.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutManagerPadding {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
    pub spacing: i32,
}

/// This structure is sent to the `LayoutManager` at the time `do_layout` is called.  This
/// structure contains the layout of all of the `Widget`s by origin, the sizes of each of the
/// `Widget`s, and their positions relative to the layout manager.  The `widget_positions` do not
/// need to be actual positions in points - they can be used to denote the positions within the
/// layout they are to occur - for instance, 0x0, 0x1, 1x0 and 1x1 in a layout, if the layout were
/// a grid layout.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutManagerCoordinates {
    pub widget_origins: Vec<Point>,
    pub widget_sizes: Vec<Size>,
    pub widget_positions: Vec<Point>,
}

/// Describes the behavior of a `LayoutManager`.  Layout Managers do not actually manage any
/// `Widget` objects by themselves, they only apply the rules of the `LayoutManagerPadding`
/// coordinates, and the size of the `LayoutManager` that is available.
pub trait LayoutManager {
    /// This method is called when a new `Widget` is added to the list, or the layout needs to
    /// be re-computed (due to a resize).  The `origin` passed is the origin of the layout
    /// manager's top-level `Widget`, as each `LayoutManager` requires a `Widget` against which
    /// objects can be added.  `size` indicates the `Size` of the `Widget` container, which is
    /// the total allowed bounds of the objects within its layout area.  `coordinates` are the
    /// coordinates of the `Widget`s to be resized.  Once the compute of all of the coordinates
    /// of all of the objects is completed, a new `LayoutManagerCoordinates` object must be
    /// returned, containing the _new_ coordinates of all of the objects within its bounds.
    /// See also `HorizontalLayoutManager` and `VerticalLayoutManager` for more info.
    fn do_layout(
        &mut self,
        origin: Point,
        size: Size,
        coordinates: LayoutManagerCoordinates,
    ) -> LayoutManagerCoordinates;

    /// This method is used to reset the `LayoutManagerCoordinates`.  Once the layout coordinates
    /// are adjusted, the system will automatically call `do_layout` so that the layout is
    /// re-calculated, and the objects are redrawn inside the bounds of the layout box.
    fn adjust_layout(&mut self, coordinates: LayoutManagerPadding);

    /// This function must be overridden to return the ID of the `Widget` that is stored within
    /// the `LayoutManager`, since the system doesn't have direct access to it.
    fn get_widget_id(&self) -> i32;
}
