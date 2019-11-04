// Pushrod Rendering Library
// Extensible Widget Library
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

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::render::callbacks::*;
use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::*;
use crate::render::{Points, Size};
use sdl2::pixels::Color;
use std::collections::HashMap;

pub trait ConfigConvenience {}

/// This trait is shared by all `Widget` objects that have a presence on the screen.  Functions that
/// must be implemented are documented in the trait.
///
/// ## Implementation Notes
/// If no custom `get_config` function is defined, and no custom `get_system_properties` function
/// is defined, you can omit the definition of both, and use the `default_widget_properties!()`
/// macro to auto-generate this code in your `impl` of this `trait`.  Keep in mind, however, that
/// these automatically generated implementation details could change in future releases of this
/// library, so it is best to use the default implementation if possible.
pub trait Widget {
    /// Draws the widget.  If you wish to modify the canvas object, you must declare it as `mut` in
    /// your implementation (ie `fn draw(&mut self, mut canvas: Canvas<Window>)`).  The `_canvas`
    /// is the currently active drawing canvas at the time this function is called.  This called
    /// during the draw loop of the `Engine`.
    fn draw(&mut self, _c: &mut Canvas<Window>) {}

    /// Retrieves the `WidgetConfig` object for this `Widget`.
    fn get_config(&mut self) -> &mut WidgetConfig;

    /// Retrieves a `HashMap` containing system properties used by the `Pushrod` event engine.
    fn get_system_properties(&mut self) -> &mut HashMap<i32, String>;

    /// Retrieves a `Callback` registry for this `Widget`.
    fn get_callbacks(&mut self) -> &mut CallbackRegistry;

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer]) {
        self.mouse_entered_callback(_widgets);
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer]) {
        self.mouse_exited_callback(_widgets);
    }

    /// When a mouse moves within the bounds of the `Widget`, this function is triggered.  It
    /// contains the `X` and `Y` coordinates relative to the bounds of the `Widget`.  The
    /// points start at `0x0`.  This function implementation is **optional**.
    fn mouse_moved(&mut self, _widgets: &[WidgetContainer], _points: Points) {
        self.mouse_moved_callback(_widgets, _points);
    }

    /// When a mouse scroll is triggered within the bounds of the `Widget`, this function is
    /// triggered.  Movement along the X axis indicate horizontal movement, where the Y axis
    /// indicates vertical movement.  Positive movement means to the right or down, respectively.
    /// Negative movement means to the left or up, respectively.  This function implementation
    /// is **optional**.
    fn mouse_scrolled(&mut self, _widgets: &[WidgetContainer], _points: Points) {
        self.mouse_scrolled_callback(_widgets, _points);
    }

    /// When a mouse button is clicked within (or outside of) the bounds of the `Widget`, this
    /// function is called.  If a mouse button is clicked, and the mouse leaves the bounds of the
    /// `Widget`, the mouse release event will still be triggered for the last `Widget` which
    /// received the mouse down state.  This prevents `Widget`s from becoming confused.  This
    /// behavior is tracked by the main loop, not by the `Widget` code.  Therefore, when a mouse
    /// button is released outside of the bounds of _this_ `Widget`, you must adjust your state
    /// accordingly, if you pay attention to the `button_clicked` function.  This function
    /// implementation is **optional**.
    fn button_clicked(
        &mut self,
        _widgets: &[WidgetContainer],
        _button: u8,
        _clicks: u8,
        _state: bool,
    ) {
        self.button_clicked_callback(_widgets, _button, _clicks, _state);
    }

    /// When a timer tick goes by (ie. a frame is displayed on the screen), this function is
    /// called.  This function implementation is **optional**.
    fn tick(&mut self, _widgets: &[WidgetContainer]) {
        self.tick_callback(_widgets);
    }

    /// This calls the `on_tick` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_tick` callback.
    fn tick_callback(&mut self, _widgets: &[WidgetContainer]) {}

    /// This calls the `on_mouse_entered` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_mouse_entered` callback.
    fn mouse_entered_callback(&mut self, _widgets: &[WidgetContainer]) {}

    /// This calls the `on_mouse_exited` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_mouse_exited` callback.
    fn mouse_exited_callback(&mut self, _widgets: &[WidgetContainer]) {}

    /// This calls the `on_mouse_moved` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_mouse_moved` callback.
    fn mouse_moved_callback(&mut self, _widgets: &[WidgetContainer], _points: Points) {}

    /// This calls the `on_mouse_scrolled` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_mouse_scrolled` callback.
    fn mouse_scrolled_callback(&mut self, _widgets: &[WidgetContainer], _points: Points) {}

    /// This calls the `on_button_clicked` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_button_clicked` callback.
    fn button_clicked_callback(
        &mut self,
        _widgets: &[WidgetContainer],
        _button: u8,
        _clicks: u8,
        _state: bool,
    ) {
    }

    /// Sets a point for a configuration key.
    fn set_point(&mut self, config: u8, x: i32, y: i32) {
        self.get_config().set_point(config, x, y);
    }

    /// Sets a color for a configuration key.
    fn set_color(&mut self, config: u8, color: Color) {
        self.get_config().set_color(config, color);
    }

    /// Sets a numeric value for a configuration key.
    fn set_numeric(&mut self, config: u8, value: i32) {
        self.get_config().set_numeric(config, value);
    }

    /// Sets a text value for a configuration key.
    fn set_text(&mut self, config: u8, text: String) {
        self.get_config().set_text(config, text);
    }

    /// Sets a toggle for a configuration key.
    fn set_toggle(&mut self, config: u8, flag: bool) {
        self.get_config().set_toggle(config, flag);
    }

    /// Sets a compass position for a configuration key.
    fn set_compass(&mut self, config: u8, value: CompassPosition) {
        self.get_config().set_compass(config, value);
    }

    /// Retrieves a `Points` for a configuration key.  Returns `Points::default` if not set.
    fn get_point(&mut self, k: u8) -> Points {
        self.get_config().get_point(k)
    }

    /// Retrieves a `Size` for a configuration key.  Returns a `Size::default` if not set.
    fn get_size(&mut self, k: u8) -> Size {
        self.get_config().get_size(k)
    }

    /// Retrieves a `Color` for a configuration key.  Returns white if not set.
    fn get_color(&mut self, k: u8) -> Color {
        self.get_config().get_color(k)
    }

    /// Retrieves a numeric value for a configuration key.  Returns 0 if not set.
    fn get_numeric(&mut self, k: u8) -> i32 {
        self.get_config().get_numeric(k)
    }

    /// Retrieves text for a configuration key.  Returns a blank string if not set.
    fn get_text(&mut self, k: u8) -> String {
        self.get_config().get_text(k)
    }

    /// Retrieves a boolean toggle for a configuration key.  Returns `false` if not set.
    fn get_toggle(&mut self, k: u8) -> bool {
        self.get_config().get_toggle(k)
    }

    /// Retrieves a `CompassPosition` toggle for a configuration key.  Returns `CompassPosition::W` if not set.
    fn get_compass(&mut self, k: u8) -> CompassPosition {
        self.get_config().get_compass(k)
    }

    /// Sets the origin of the `Widget`, adjusting the X and Y coordinates.  Automatically sets the
    /// `invalidate` flag to `true` when adjusted, but only if the new origin is not the same as
    /// the previous origin.
    fn set_origin(&mut self, _origin: Points) {
        let old_origin = self.get_config().get_point(CONFIG_ORIGIN);

        if _origin[0] != old_origin[0] || _origin[1] != old_origin[1] {
            self.get_config()
                .set_point(CONFIG_ORIGIN, _origin[0], _origin[1]);
            self.get_config().set_invalidate(true);
        }
    }

    /// Sets the size of the `Widget`, adjusting the width and height.  Automatically
    /// sets the `invalidate` flag to `true` when adjusted, but only if the new size is not the
    /// same as the previous size.
    fn set_size(&mut self, _size: Vec<u32>) {
        let old_size = self.get_config().get_size(CONFIG_SIZE);

        if _size[0] != old_size[0] || _size[1] != old_size[1] {
            self.get_config().set_size(CONFIG_SIZE, _size[0], _size[1]);
            self.get_config().set_invalidate(true);
        }
    }

    /// Returns a `Rect` object containing the drawing bounds of this `Widget`.
    fn get_drawing_area(&mut self) -> Rect {
        Rect::new(
            self.get_config().to_x(0),
            self.get_config().to_y(0),
            self.get_config().get_size(CONFIG_SIZE)[0],
            self.get_config().get_size(CONFIG_SIZE)[1],
        )
    }
}

/// This is an example top-level `Widget` object that is used to draw a background and a border
/// of specified colors.  `COLOR_BASE` determines the background fill color, and the `COLOR_BORDER`
/// determines the color of the border.  The width of the border is controlled by the
/// `get_config().border_width` property.
pub struct BaseWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
}

/// Base top-level implementation of the `BaseWidget`, which other classes can extend.
impl BaseWidget {
    /// Constructs a new base widget, given the points of origin and size.
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
        }
    }
}

/// Implementation for drawing a `BaseWidget`, with the `Widget` trait objects applied.
impl Widget for BaseWidget {
    fn draw(&mut self, mut _canvas: &mut Canvas<Window>) {
        let base_color = self.get_config().get_color(CONFIG_COLOR_BASE);
        let border_color = self.get_config().get_color(CONFIG_COLOR_BORDER);

        _canvas.set_draw_color(base_color);

        _canvas.fill_rect(self.get_drawing_area()).unwrap();

        if self.get_config().get_numeric(CONFIG_BORDER_WIDTH) > 0 && base_color != border_color {
            _canvas.set_draw_color(border_color);

            for border in 0..self.get_config().get_numeric(CONFIG_BORDER_WIDTH) {
                _canvas
                    .draw_rect(Rect::new(
                        self.config.to_x(border),
                        self.config.to_y(border),
                        self.get_config().get_size(CONFIG_SIZE)[0] - (border as u32 * 2),
                        self.get_config().get_size(CONFIG_SIZE)[1] - (border as u32 * 2),
                    ))
                    .unwrap();
            }
        }
    }

    default_widget_properties!();
    default_widget_callbacks!();
}
