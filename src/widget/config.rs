// Configurable Implementation
// New configuration module, as described by u/JayDepp on Reddit - THANKS!!!
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

use graphics::types::Color;

use std::collections::HashMap;

use crate::core::point::Point;
use crate::core::point::Size;

/// Configuration object type - allows configurations to be set using `Piston`, `Pushrod`, or
/// native types.
#[derive(Clone, Debug)]
pub enum Config {
    Point(Point),
    Size(Size),
    Color(Color),
    Numeric(u64),
    Text(String),
    Toggle(bool),
}

/// Indicates whether or not a `Widget` needs to be redrawn, set by `Config::Toggle`.
pub const CONFIG_INVALIDATE: u8 = 2;

/// Origin of a `Widget`, set by `Config::Point`.
pub const CONFIG_ORIGIN: u8 = 3;

/// `Size` of a `Widget`, set by `Config::Size`.
pub const CONFIG_BODY_SIZE: u8 = 4;

/// Color of the body of a `Widget`, set by `Config::Color`.
pub const CONFIG_MAIN_COLOR: u8 = 5;

/// Color of the border of a `Widget`, set by `Config::Color`.
pub const CONFIG_BORDER_COLOR: u8 = 6;

/// Color of the text to be drawn in a `Widget`, set by `Config::Color`.
pub const CONFIG_TEXT_COLOR: u8 = 7;

/// Secondary color (body, image, etc.) in a `Widget`, set by `Config::Color`.
pub const CONFIG_SECONDARY_COLOR: u8 = 8;

/// Width of the border of a `Widget`, set by `Config::Numeric`.
pub const CONFIG_BORDER_WIDTH: u8 = 9;

/// Text to be displayed in a `Widget`, set by `Config::Text`.
pub const CONFIG_DISPLAY_TEXT: u8 = 10;

/// Progress indicator from 0-100, set by `Config::Numeric`.
pub const CONFIG_PROGRESS: u8 = 11;

/// Indicates whether or not a timer is enabled, set by `Config::Toggle`.
pub const CONFIG_TIMER_ENABLED: u8 = 12;

/// Indicates whether or not a timeout has occurred, set by `Config::Toggle`.
pub const CONFIG_TIMER_TIMEOUT: u8 = 13;

/// Indicates whether or not a `Widget` has been hidden, set by `Config::Toggle`.
pub const CONFIG_WIDGET_HIDDEN: u8 = 14;

/// Indicates whether or not a `Widget` is disabled, set by `Config::Toggle`.
pub const CONFIG_WIDGET_DISABLED: u8 = 15;

/// Identifies the `Widget`'s grouping ID for `RadioButtonWidget`, set by `Config::Numeric`.
pub const CONFIG_WIDGET_GROUP_ID: u8 = 16;

/// Indicates whether or not a `Widget` has been toggled/selected, set by `Config::Toggle`.
pub const CONFIG_SELECTED: u8 = 17;

/// Stores a payload attached to the `Widget`, which can be any string that identifies information
/// about the widget, set by `Config::Text`.  If you wish to store binary data, it may be best
/// to store that as Base64 or some other encoded form.
pub const CONFIG_PAYLOAD: u8 = 18;

/// Structure containing the configuration `HashMap`.
pub struct Configurable {
    configs: HashMap<u8, Config>,
}

impl Configurable {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Setter master method - use convenience methods instead.
    pub fn set(&mut self, config: u8, config_value: Config) {
        self.configs.insert(config, config_value.clone());
    }

    /// Removes a key from the configs.
    pub fn remove(&mut self, config: u8) {
        self.configs.remove(&config);
    }

    /// Indicates whether or not a key has been configured.
    pub fn contains(&self, config: u8) -> bool {
        self.configs.contains_key(&config)
    }

    /// Getter master method - use convenience methods instead.
    pub fn get(&self, config: u8) -> Option<&Config> {
        self.configs.get(&config)
    }

    /// Sets a point for a configuration key.
    pub fn set_point(&mut self, config: u8, x: i32, y: i32) {
        self.set(config, Config::Point([x, y]));
    }

    /// Sets a size for a configuration key.
    pub fn set_size(&mut self, config: u8, w: i32, h: i32) {
        self.set(config, Config::Size(Size { w, h }));
    }

    /// Sets a color for a configuration key.
    pub fn set_color(&mut self, config: u8, color: Color) {
        self.set(config, Config::Color(color));
    }

    /// Sets a numeric value for a configuration key.
    pub fn set_numeric(&mut self, config: u8, value: u64) {
        self.set(config, Config::Numeric(value));
    }

    /// Sets a text value for a configuration key.
    pub fn set_text(&mut self, config: u8, text: String) {
        self.set(config, Config::Text(text.clone()));
    }

    /// Sets a toggle for a configuration key.
    pub fn set_toggle(&mut self, config: u8, flag: bool) {
        self.set(config, Config::Toggle(flag));
    }

    /// Retrieves a `Point` for a configuration key.  Returns `Point::default` if not set.
    pub fn get_point(&self, config: u8) -> Point {
        match self.configs.get(&config) {
            Some(Config::Point(point)) => point.clone(),
            _ => Point::default(),
        }
    }

    /// Retrieves a `Size` for a configuration key.  Returns a `Size::default` if not set.
    pub fn get_size(&self, config: u8) -> crate::core::point::Size {
        match self.configs.get(&config) {
            Some(Config::Size(size)) => size.clone(),
            _ => Size::default(),
        }
    }

    /// Retrieves a `Color` for a configuration key.  Returns white if not set.
    pub fn get_color(&self, config: u8) -> Color {
        match self.configs.get(&config) {
            Some(Config::Color(color)) => *color,
            _ => [1.0; 4],
        }
    }

    /// Retrieves a numeric value for a configuration key.  Returns 0 if not set.
    pub fn get_numeric(&self, config: u8) -> u64 {
        match self.configs.get(&config) {
            Some(Config::Numeric(numeric)) => *numeric,
            _ => 0,
        }
    }

    /// Retrieves text for a configuration key.  Returns a blank string if not set.
    pub fn get_text(&self, config: u8) -> String {
        match self.configs.get(&config) {
            Some(Config::Text(text)) => text.clone(),
            _ => String::from(""),
        }
    }

    /// Retrieves a boolean toggle for a configuration key.  Returns `false` if not set.
    pub fn get_toggle(&self, config: u8) -> bool {
        match self.configs.get(&config) {
            Some(Config::Toggle(toggle)) => *toggle,
            _ => false,
        }
    }
}
