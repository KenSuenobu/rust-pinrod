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

use crate::render::{Points, Size};
use sdl2::pixels::Color;
use std::collections::HashMap;

/// `Widget` Base `Color` key for `colors` `HashMap`.  This is the base fill color of a `Widget`
/// that is in an unselected state.  This stored as a `Config::Color`.
pub const CONFIG_COLOR_BASE: u8 = 0;

/// `Widget` Hover `Color` key for `colors` `HashMap`.  This is the base fill color of a `Widget`
/// that has a mouse hovering over the top of the `Widget`, or when a `mouse_entered` event is
/// triggered.  This is optional; the `Widget` does not need to honor this color if it does not
/// support a hover state.  This stored as a `Config::Color`.
pub const CONFIG_COLOR_HOVER: u8 = 1;

/// `Widget` Border `Color` key for `colors` `HashMap`.  This should be used for the color of the
/// border, if the `Widget` draws a border.  This stored as a `Config::Color`.
pub const CONFIG_COLOR_BORDER: u8 = 2;

/// `Widget` Text `Color` key for `colors` `HashMap`.  This should be the color for the text being
/// displayed inside the `Widget`.  This stored as a `Config::Color`.
pub const CONFIG_COLOR_TEXT: u8 = 3;

/// `Widget` Selected `Color` key for `colors` `HashMap`.  This is the color the `Widget` should
/// display when in selected state.  This stored as a `Config::Color`.
pub const CONFIG_COLOR_SELECTED: u8 = 4;

/// `Widget` Secondary `Color` key for `colors` `HashMap`.  This is the color the `Widget` should
/// display for any secondary properties, such as a fill color for a progress widget, a spinner,
/// etc.  This stored as a `Config::Color`.
pub const CONFIG_COLOR_SECONDARY: u8 = 5;

/// `Widget` configuration to store its origin on the screen.  This is a `Config::Points` object in the
/// config.
pub const CONFIG_ORIGIN: u8 = 6;

/// `Widget` configuration that stores the size of the `Widget`.  This is a `Config::Size` object
/// in the config.
pub const CONFIG_SIZE: u8 = 7;

/// `Widget` configuration that stores the display border in pixels.  This is stored as a
/// `Config::Numeric` value.
pub const CONFIG_BORDER_WIDTH: u8 = 8;

/// `Widget` text store, used to display text on the screen.  This is stored as a `Config::Text`
/// value.
pub const CONFIG_TEXT: u8 = 9;

/// `Widget` progress value store.  This is stored as a `Config::Numeric` value.
pub const CONFIG_PROGRESS: u8 = 10;

/// `Widget` image position direction, controls the position of an `Image` within the bounds of a
/// `Widget`.  This is stored as a `Config::CompassPosition` value.
pub const CONFIG_IMAGE_POSITION: u8 = 11;

/// `TextWidget` font size control.  This is stored as a `Config::Numeric` value.
pub const CONFIG_FONT_SIZE: u8 = 12;

/// This enum is used by the `ImageWidget`, which controls the positioning of the image being
/// rendered within the bounds of the `Widget`.
#[derive(Clone, Debug)]
pub enum CompassPosition {
    /// Upper left-hand corner of the bounds.
    NW,

    /// Centered top of the bounds.
    N,

    /// Upper right-hand corner of the bounds.
    NE,

    /// Centered left side of the bounds.
    W,

    /// Center of the bounds.
    Center,

    /// Centered right side of the bounds.
    E,

    /// Lower left-hand corner of the bounds.
    SW,

    /// Bottom center of the bounds.
    S,

    /// Lower right-hand corner of the bounds.
    SE,
}

/// Configuration object type - allows configurations to be set using `Piston`, `Pushrod`, or
/// native types.
#[derive(Clone, Debug)]
pub enum Config {
    /// This stores a `Points` type.
    Points(Points),

    /// This stores a `Size` type.
    Size(Size),

    /// This stores a `Color`.
    Color(Color),

    /// This stores a numeric value in the form of an `i32` value.
    Numeric(i32),

    /// This stores a `String` of text.
    Text(String),

    /// This stores a `true`/`false` boolean flag.
    Toggle(bool),

    /// This stores a `ComapssPosition`.
    CompassPosition(CompassPosition),
}

/// This is the store for the `WidgetConfig`, which each `Widget` object needs.  This stores
/// information about the `Widget`.  It currently contains the point of origin, size, a `HashMap` of
/// different `Color`s, a border width, and an invalidation flag.
pub struct WidgetConfig {
    /// The `HashMap` store for configuration objects.
    pub config: HashMap<u8, Config>,

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
            config: [
                (CONFIG_ORIGIN, Config::Points(vec![x, y])),
                (CONFIG_SIZE, Config::Size(vec![w, h])),
                (CONFIG_COLOR_BASE, Config::Color(Color::RGB(255, 255, 255))),
                (CONFIG_BORDER_WIDTH, Config::Numeric(0)),
            ]
            .iter()
            .cloned()
            .collect(),
            hidden: false,
            enabled: true,
            invalidated: true,
        }
    }

    /// Converts an X point to the physical X point on the `Canvas` plus the point of origin.
    /// Returns `i32` containing the modified X coordinate.  This is a convenience method for the
    /// `Widget` to draw based on a 0x0 point of origin.
    pub fn to_x(&self, x: i32) -> i32 {
        self.get_point(CONFIG_ORIGIN)[0] + x
    }

    /// Converts a Y point to the physical Y point on the `Canvas` plus the point of origin.
    /// Returns `i32` containing the modified Y coordinate.  This is a convenience method for the
    /// `Widget` to draw based on a 0x0 point of origin.
    pub fn to_y(&self, y: i32) -> i32 {
        self.get_point(CONFIG_ORIGIN)[1] + y
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

    /// Sets a point for a configuration key.
    pub fn set_point(&mut self, config: u8, x: i32, y: i32) {
        self.config.insert(config, Config::Points(vec![x, y]));
    }

    /// Sets a size for a configuration key.
    pub fn set_size(&mut self, config: u8, w: u32, h: u32) {
        self.config.insert(config, Config::Size(vec![w, h]));
    }

    /// Sets a color for a configuration key.
    pub fn set_color(&mut self, config: u8, color: Color) {
        self.config.insert(config, Config::Color(color));
    }

    /// Sets a numeric value for a configuration key.
    pub fn set_numeric(&mut self, config: u8, value: i32) {
        self.config.insert(config, Config::Numeric(value));
    }

    /// Sets a text value for a configuration key.
    pub fn set_text(&mut self, config: u8, text: String) {
        self.config.insert(config, Config::Text(text.clone()));
    }

    /// Sets a toggle for a configuration key.
    pub fn set_toggle(&mut self, config: u8, flag: bool) {
        self.config.insert(config, Config::Toggle(flag));
    }

    /// Sets a compass position for a configuration key.
    pub fn set_compass(&mut self, config: u8, value: CompassPosition) {
        self.config.insert(config, Config::CompassPosition(value));
    }

    /// Retrieves a `Points` for a configuration key.  Returns `Points::default` if not set.
    pub fn get_point(&self, k: u8) -> Points {
        match self.config.get(&k) {
            Some(Config::Points(point)) => point.clone(),
            _ => Points::default(),
        }
    }

    /// Retrieves a `Size` for a configuration key.  Returns a `Size::default` if not set.
    pub fn get_size(&self, k: u8) -> Size {
        match self.config.get(&k) {
            Some(Config::Size(size)) => size.clone(),
            _ => Size::default(),
        }
    }

    /// Retrieves a `Color` for a configuration key.  Returns white if not set.
    pub fn get_color(&self, k: u8) -> Color {
        match self.config.get(&k) {
            Some(Config::Color(color)) => *color,
            _ => Color::RGB(255, 255, 255),
        }
    }

    /// Retrieves a numeric value for a configuration key.  Returns 0 if not set.
    pub fn get_numeric(&self, k: u8) -> i32 {
        match self.config.get(&k) {
            Some(Config::Numeric(numeric)) => *numeric,
            _ => 0,
        }
    }

    /// Retrieves text for a configuration key.  Returns a blank string if not set.
    pub fn get_text(&self, k: u8) -> String {
        match self.config.get(&k) {
            Some(Config::Text(text)) => text.clone(),
            _ => String::from(""),
        }
    }

    /// Retrieves a boolean toggle for a configuration key.  Returns `false` if not set.
    pub fn get_toggle(&self, k: u8) -> bool {
        match self.config.get(&k) {
            Some(Config::Toggle(toggle)) => *toggle,
            _ => false,
        }
    }

    /// Retrieves a `CompassPosition` toggle for a configuration key.  Returns `CompassPosition::W` if not set.
    pub fn get_compass(&self, k: u8) -> CompassPosition {
        match self.config.get(&k) {
            Some(Config::CompassPosition(position)) => position.clone(),
            _ => CompassPosition::W,
        }
    }
}
