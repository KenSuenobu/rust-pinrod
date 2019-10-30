// Pushrod Rendering Library
// Widget Configuration Store
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

use sdl2::pixels::Color;
use std::collections::HashMap;

/// `Widget` Base `Color` key for `colors` `HashMap`.  This is the base fill color of a `Widget`
/// that is in an unselected state.
pub const COLOR_BASE: u8 = 0;

/// `Widget` Hover `Color` key for `colors` `HashMap`.  This is the base fill color of a `Widget`
/// that has a mouse hovering over the top of the `Widget`, or when a `mouse_entered` event is
/// triggered.  This is optional; the `Widget` does not need to honor this color if it does not
/// support a hover state.
pub const COLOR_HOVER: u8 = 1;

/// `Widget` Border `Color` key for `colors` `HashMap`.  This should be used for the color of the
/// border, if the `Widget` draws a border.
pub const COLOR_BORDER: u8 = 2;

/// `Widget` Text `Color` key for `colors` `HashMap`.  This should be the color for the text being
/// displayed inside the `Widget`.
pub const COLOR_TEXT: u8 = 3;

/// `Widget` Selected `Color` key for `colors` `HashMap`.  This is the color the `Widget` should
/// display when in selected state.
pub const COLOR_SELECTED: u8 = 4;

/// `Widget` Secondary `Color` key for `colors` `HashMap`.  This is the color the `Widget` should
/// display for any secondary properties, such as a fill color for a progress widget, a spinner,
/// etc.
pub const COLOR_SECONDARY: u8 = 5;

/// This is the store for the `WidgetConfig`, which each `Widget` object needs.  This stores
/// information about the `Widget`.  It currently contains the point of origin, size, a `HashMap` of
/// different `Color`s, a border width, and an invalidation flag.
pub struct WidgetConfig {
    /// This `Vec` contains two points of origin: the physical X and Y coordinates in the
    /// `Canvas`.
    pub origin: Vec<i32>,

    /// This `Vec` contains the width and height of the object.
    pub size: Vec<u32>,

    /// This `HashMap` contains a key/value pair containing colors for the `Widget`.
    pub colors: HashMap<u8, Color>,

    /// This is the border width in pixels.
    pub border_width: u8,

    /// `Widget`'s hidden flag - any children that refer to this object as a `parent_id` will not
    /// be drawn, and their events will not be received.
    hidden: bool,

    /// `Widget`'s enabled flag - any mouse events are ignored, but drawing is still performed.
    enabled: bool,

    /// `Widget`'s redraw flag.  Set `true` if the object needs to be redrawn, `false` otherwise.
    invalidated: bool,
}

/// This is the implementation of the `WidgetConfig`.
impl WidgetConfig {
    /// Constructor - takes the X, Y, W, and H coordinates of the `Widget`, physically in the
    /// main `Canvas`.
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            origin: vec![x, y],
            size: vec![w, h],
            colors: [(COLOR_BASE, Color::RGB(255, 255, 255))]
                .iter()
                .cloned()
                .collect(),
            border_width: 0,
            hidden: false,
            enabled: true,
            invalidated: true,
        }
    }

    /// Converts an X point to the physical X point on the `Canvas` plus the point of origin.
    /// Returns `i32` containing the modified X coordinate.  This is a convenience method for the
    /// `Widget` to draw based on a 0x0 point of origin.
    pub fn to_x(&self, x: i32) -> i32 {
        self.origin[0] + x
    }

    /// Converts a Y point to the physical Y point on the `Canvas` plus the point of origin.
    /// Returns `i32` containing the modified Y coordinate.  This is a convenience method for the
    /// `Widget` to draw based on a 0x0 point of origin.
    pub fn to_y(&self, y: i32) -> i32 {
        self.origin[1] + y
    }

    /// Sets the invalidation state of the `Widget`, telling the `Engine` that the `Widget`
    /// contents has changed, and must be redrawn.  Setting the `flag` to `true` indicates that
    /// the `Widget` needs to be redrawn on the screen, `false` indicates that it its state has
    /// not changed, and its image can be pulled from a buffer if necessary, skipping the `draw`
    /// call.
    pub fn set_invalidate(&mut self, flag: bool) {
        self.invalidated = flag;
    }

    /// Returns the `invalidation` state.  Returns a `bool` containing the state.
    pub fn invalidated(&self) -> bool {
        self.invalidated
    }

    /// Enables the `Widget` for interaction.
    pub fn enable(&mut self) {
        self.enabled = true;
        self.invalidated = true;
    }

    /// Disables the `Widget`, preventing interaction.
    pub fn disable(&mut self) {
        self.enabled = false;
        self.invalidated = true;
    }

    /// Indicates whether or not this `Widget` is enabled or disabled - `true` if enabled,
    /// `false` otherwise.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Prevents the `Widget` from being drawn on the screen, or being interacted with.  No events
    /// are received by this `Widget` when hidden.
    pub fn hide(&mut self) {
        self.hidden = true;
        self.invalidated = true;
    }

    /// Displays the `Widget` on the screen.
    pub fn show(&mut self) {
        self.hidden = false;
        self.invalidated = true;
    }

    /// Indicates whether or not this `Widget` has been hidden from view - `true` if this `Widget`
    /// is hidden, `false` otherwise.
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
}
