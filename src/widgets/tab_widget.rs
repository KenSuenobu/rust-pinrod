// Pushrod Widget Library
// Tab Widget
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

use crate::render::callbacks::CallbackRegistry;
use crate::render::layout_cache::LayoutContainer;
use crate::render::widget::*;
use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::{WidgetConfig, CONFIG_COLOR_BASE, CONFIG_ORIGIN, CONFIG_SIZE};
use crate::render::{Points, POINT_X, POINT_Y, SIZE_HEIGHT, SIZE_WIDTH};

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::pixels::Color;
use std::any::Any;
use std::collections::HashMap;

/// This is the storage object for the `TabWidget`.  It stores the config, properties, callback registry,
/// and the tab widget IDs.
pub struct TabWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    tabs: Vec<String>,
    selected_tab: i32,
}

/// Creates a new `ImageWidget`, which draws an image in a supported image format for SDL2 at a specific
/// location on the screen.  Requires the name of the image (the full path to the file), the position
/// within the widget (defined as `ImagePosition`), the xywh bounds, and whether or not the image is
/// scaled within the bounds of the `Widget`.
impl TabWidget {
    /// Creates a new instance of the `ImageWidget` object.  Requires an image name (full path of the file),
    /// image position (defined in `ImagePosition`), the `xywh` bounds of the `Widget`, and a scale flag.
    /// If `scaled` is set to `true`, the image will be scaled within the `Widget` bounds, and the
    /// `ImagePosition` will be ignored.  Likewise, if set to `false`, the image will be displayed for
    /// the size of the image, and will be placed in the bounds of the `Widget` based on the position
    /// specified in the `ImagePosition`.
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            tabs: vec![],
            selected_tab: 0,
        }
    }
}

/// This is the `Widget` implementation of the `TabWidget`.  Image is rendered onto a 3D texture, then
/// copied to the canvas after rendering.
impl Widget for TabWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) {
        let base_color = self.get_color(CONFIG_COLOR_BASE);
        let origin = self.get_config().get_point(CONFIG_ORIGIN);
        let size = self.get_config().get_size(CONFIG_SIZE);

        c.set_draw_color(base_color);
        c.fill_rect(self.get_drawing_area()).unwrap();

        c.set_draw_color(Color::RGB(0, 0, 0));
        c.draw_line(
            Point::new(
                origin[POINT_X],
                origin[POINT_Y] + size[SIZE_HEIGHT] as i32 - 1,
            ),
            Point::new(
                origin[POINT_X] + size[SIZE_WIDTH] as i32,
                origin[POINT_Y] + size[SIZE_HEIGHT] as i32 - 1,
            ),
        )
        .unwrap();
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        eprintln!("Mouse entered bounds of TabWidget.");
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        eprintln!("Mouse exited bounds of TabWidget.");
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
        eprintln!("Mouse moved in bounds of TabWidget: {:?}", _points);
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
        eprintln!(
            "Mouse button clicked in bounds of TabWidget: {} {}",
            _button, _state
        );
    }

    default_widget_functions!();
    default_widget_properties!();
}
