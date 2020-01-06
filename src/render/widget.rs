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
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::render::callbacks::*;
use crate::render::canvas_helper::CanvasHelper;
use crate::render::layout_cache::LayoutContainer;
use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::*;
use crate::render::{Points, Size};
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::any::Any;
use std::collections::HashMap;
use crate::render::texture_store::TextureStore;

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
    /// Retrieves this `Widget` as an `Any` object so that it can be downcast using `downcast_ref`
    /// to a `struct` that implements the `Widget` trait.
    fn as_any(&mut self) -> &mut dyn Any;

    /// Draws the widget.  If you wish to modify the canvas object, you must declare it as `mut` in
    /// your implementation (ie `fn draw(&mut self, mut canvas: Canvas<Window>)`).  The `_canvas`
    /// is the currently active drawing canvas at the time this function is called.  This called
    /// during the draw loop of the `Engine`.  This returns a reference to the stored `Texture` object
    /// within the `Widget`.  It is then copied to the canvas, and displayed in the display loop.
    /// In this function, you can just return a reference to the `Texture` if no invalidation state
    /// was set, otherwise, the draw can be re-performed, and the `Texture` returned.  If the drawing
    /// function returns no texture, return a `None`, and it will not be rendered during the display
    /// loop, but it will still be called.
    fn draw(&mut self, _c: &mut Canvas<Window>) -> Option<&Texture> {
        None
    }

    /// Retrieves the `WidgetConfig` object for this `Widget`.
    fn get_config(&mut self) -> &mut WidgetConfig;

    /// Retrieves a `HashMap` containing system properties used by the `Pushrod` event engine.
    fn get_system_properties(&mut self) -> &mut HashMap<i32, String>;

    /// Retrieves a `Callback` registry for this `Widget`.
    fn get_callbacks(&mut self) -> &mut CallbackRegistry;

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.mouse_entered_callback(_widgets, _layouts);
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.mouse_exited_callback(_widgets, _layouts);
    }

    /// When a mouse moves within the bounds of the `Widget`, this function is triggered.  It
    /// contains the `X` and `Y` coordinates relative to the bounds of the `Widget`.  The
    /// points start at `0x0`.  This function implementation is **optional**.
    fn mouse_moved(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _points: Points,
    ) {
        self.mouse_moved_callback(_widgets, _layouts, _points);
    }

    /// When a mouse scroll is triggered within the bounds of the `Widget`, this function is
    /// triggered.  Movement along the X axis indicate horizontal movement, where the Y axis
    /// indicates vertical movement.  Positive movement means to the right or down, respectively.
    /// Negative movement means to the left or up, respectively.  This function implementation
    /// is **optional**.
    fn mouse_scrolled(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _points: Points,
    ) {
        self.mouse_scrolled_callback(_widgets, _layouts, _points);
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
        _layouts: &[LayoutContainer],
        _button: u8,
        _clicks: u8,
        _state: bool,
    ) {
        self.button_clicked_callback(_widgets, _layouts, _button, _clicks, _state);
    }

    /// When a timer tick goes by (ie. a frame is displayed on the screen), this function is
    /// called.  This function implementation is **optional**.
    fn tick(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.tick_callback(_widgets, _layouts);
    }

    /// When an `Event` is sent to the application that is not handled by the `Engine::run` loop, this
    /// method is called, sending the unhandled `Event` to the currently active `Widget`.  **This behavior
    /// is subject to change** as the `Engine::run` loop is modified to handle more `Event`s.
    fn other_event(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _event: Event,
    ) {
        eprintln!("Other event: {:?}", _event);
    }

    /// This calls the `on_tick` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_tick` callback.
    fn tick_callback(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {}

    /// This calls the `on_mouse_entered` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_mouse_entered` callback.
    fn mouse_entered_callback(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
    ) {
    }

    /// This calls the `on_mouse_exited` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_mouse_exited` callback.
    fn mouse_exited_callback(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
    ) {
    }

    /// This calls the `on_mouse_moved` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_mouse_moved` callback.
    fn mouse_moved_callback(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _points: Points,
    ) {
    }

    /// This calls the `on_mouse_scrolled` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_mouse_scrolled` callback.
    fn mouse_scrolled_callback(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _points: Points,
    ) {
    }

    /// This calls the `on_button_clicked` callback.  This is implemented by the `default_widget_callbacks!` macro,
    /// so you do not need to implement it.  However, you need to call this function if you wish
    /// to honor an `on_button_clicked` callback.
    fn button_clicked_callback(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _button: u8,
        _clicks: u8,
        _state: bool,
    ) {
    }

    /// This callback is called when a setter is used to configure a value.  It is _not_ called when a
    /// call to `get_config()` using the setter is called, so it is best to use the top-level setters
    /// and getters for the configuration values - at least, until the `get_config()` call can be made
    /// private.
    fn on_config_changed(&mut self, _k: u8, _v: Config) {}

    /// Sets a point for a configuration key.
    fn set_point(&mut self, config: u8, x: i32, y: i32) {
        self.get_config().set_point(config, x, y);
        self.on_config_changed(config, Config::Points(vec![x, y]));
    }

    /// Sets a color for a configuration key.
    fn set_color(&mut self, config: u8, color: Color) {
        self.get_config().set_color(config, color);
        self.on_config_changed(config, Config::Color(color));
    }

    /// Sets a numeric value for a configuration key.
    fn set_numeric(&mut self, config: u8, value: i32) {
        self.get_config().set_numeric(config, value);
        self.on_config_changed(config, Config::Numeric(value));
    }

    /// Sets a text value for a configuration key.
    fn set_text(&mut self, config: u8, text: String) {
        self.get_config().set_text(config, text.clone());
        self.on_config_changed(config, Config::Text(text));
    }

    /// Sets a toggle for a configuration key.
    fn set_toggle(&mut self, config: u8, flag: bool) {
        self.get_config().set_toggle(config, flag);
        self.on_config_changed(config, Config::Toggle(flag));
    }

    /// Sets a compass position for a configuration key.
    fn set_compass(&mut self, config: u8, value: CompassPosition) {
        self.get_config().set_compass(config, value);
        self.on_config_changed(config, Config::CompassPosition(value));
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
            self.get_config().set_invalidated(true);
        }
    }

    /// Sets the size of the `Widget`, adjusting the width and height.  Automatically
    /// sets the `invalidate` flag to `true` when adjusted, but only if the new size is not the
    /// same as the previous size.
    fn set_size(&mut self, _size: Vec<u32>) {
        let old_size = self.get_config().get_size(CONFIG_SIZE);

        if _size[0] != old_size[0] || _size[1] != old_size[1] {
            self.get_config().set_size(CONFIG_SIZE, _size[0], _size[1]);
            self.get_config().set_invalidated(true);
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

    /// Returns whether or not a `Widget` is invalidated state.
    fn is_invalidated(&mut self) -> bool {
        self.get_config().invalidated()
    }

    /// Sets invalidation state for the current `Widget`.
    fn set_invalidated(&mut self, flag: bool) {
        self.get_config().set_invalidated(flag);
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
    texture_store: TextureStore,
}

/// Base top-level implementation of the `BaseWidget`, which other classes can extend.
impl BaseWidget {
    /// Constructs a new base widget, given the points of origin and size.
    pub fn new(points: Points, size: Size) -> Self {
        Self {
            config: WidgetConfig::new(points, size),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            texture_store: TextureStore::new(),
        }
    }
}

impl CanvasHelper for BaseWidget {}

/// Implementation for drawing a `BaseWidget`, with the `Widget` trait objects applied.
impl Widget for BaseWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) -> Option<&Texture> {
        let bounds = self.get_config().get_size(CONFIG_SIZE);

        self.texture_store.create_or_resize_texture(c, bounds[0] as u32, bounds[1] as u32);

        // You _can_ remove this `if` statement here, and just let the code run each time.  It will
        // eventually make your application less efficient if this is constantly called.
        if self.get_config().invalidated() {
            let base_color = self.get_config().get_color(CONFIG_COLOR_BASE);
            let border_color = self.get_config().get_color(CONFIG_COLOR_BORDER);

            c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
                texture.set_draw_color(base_color);
                texture.clear();

                texture.set_draw_color(border_color);
                texture
                    .draw_rect(Rect::new(0, 0, bounds[0], bounds[1]))
                    .unwrap();
            })
                .unwrap();
        }

        self.texture_store.get_optional_ref()
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
