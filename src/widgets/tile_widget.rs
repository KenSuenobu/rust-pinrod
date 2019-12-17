// Pushrod Widget Library
// Tile Widget
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
use crate::render::widget::*;
use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::*;
use crate::render::{Points, Size};

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::render::layout_cache::LayoutContainer;
use std::any::Any;
use std::collections::HashMap;

/// This is the callback type that is used when an `on_click` callback is triggered from this
/// `Widget`.  Returns a flag indicating the selected state - toggled on or off.
pub type OnSelectedCallbackType =
    Option<Box<dyn FnMut(&mut TileWidget, &[WidgetContainer], &[LayoutContainer], bool)>>;

/// This is the storage object for the `TileWidget`.  It stores the config, properties, callback registry.
pub struct TileWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    on_selected: OnSelectedCallbackType,
    image_filename: String,
    tile_text: String,
}

/// This is the implementation of the `TileWidget`, which displays an image next to some text.
impl TileWidget {
    /// Creates a new `TileWidget`, given the `x, y, w, h` coordinates, a block of `text`, the
    /// `font_size` to use, and the `image_name` to load and display.
    pub fn new(points: Points, size: Size, image_filename: String, tile_text: String) -> Self {
        Self {
            config: WidgetConfig::new(points.clone(), size.clone()),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            on_selected: None,
            image_filename: image_filename.clone(),
            tile_text: tile_text.clone(),
        }
    }

    //    fn draw_hovered(&mut self) {
    //        self.base_widget
    //            .set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    //        self.text_widget
    //            .set_color(CONFIG_COLOR_TEXT, Color::RGB(255, 255, 255));
    //        self.text_widget
    //            .set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    //        self.get_config().set_invalidated(true);
    //    }
    //
    //    fn draw_unhovered(&mut self) {
    //        self.base_widget
    //            .set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
    //        self.text_widget
    //            .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));
    //        self.text_widget
    //            .set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
    //        self.get_config().set_invalidated(true);
    //    }

    //    /// Assigns the callback closure that will be used when a button click is triggered.
    //    pub fn on_click<F>(&mut self, callback: F)
    //        where
    //            F: FnMut(&mut ImageButtonWidget, &[WidgetContainer], &[LayoutContainer]) + 'static,
    //    {
    //        self.on_click = Some(Box::new(callback));
    //    }
    //
    //    /// Internal function that triggers the `on_click` callback.
    //    fn call_click_callback(&mut self, widgets: &[WidgetContainer], layouts: &[LayoutContainer]) {
    //        if let Some(mut cb) = self.on_click.take() {
    //            cb(self, widgets, layouts);
    //            self.on_click = Some(cb);
    //        }
    //    }
}

/// This is the `Widget` implementation of the `TileWidget`.
impl Widget for TileWidget {
    fn draw(&mut self, _c: &mut Canvas<Window>) {
        //        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        //        // Invalidation is controlled by the top level widget (this box).
        //        self.base_widget.draw(c);
        //        self.text_widget.draw(c);
        //        self.image_widget.draw(c);
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        //        if self.active {
        //            self.draw_hovered();
        //        }
        //
        //        self.in_bounds = true;
        //        self.mouse_entered_callback(_widgets, _layouts);
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        //        if self.active {
        //            self.draw_unhovered();
        //        }
        //
        //        self.in_bounds = false;
        //        self.mouse_exited_callback(_widgets, _layouts);
    }

    fn button_clicked(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _button: u8,
        _clicks: u8,
        _state: bool,
    ) {
        //        if _button == 1 {
        //            if _state {
        //                self.draw_hovered();
        //                self.active = true;
        //                self.originated = true;
        //            } else {
        //                let had_bounds = self.active;
        //
        //                self.draw_unhovered();
        //                self.active = false;
        //
        //                if self.in_bounds && had_bounds && self.originated {
        //                    // Callback here
        //                    eprintln!("Call callback here: clicks={}", _clicks);
        //                    self.call_click_callback(_widgets, _layouts);
        //                }
        //
        //                self.originated = false;
        //            }
        //        }
        //
        //        self.button_clicked_callback(_widgets, _layouts, _button, _clicks, _state);
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
