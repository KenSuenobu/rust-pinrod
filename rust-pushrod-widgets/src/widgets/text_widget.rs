// Pushrod Widget Library
// Text Widget
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

use pushrod_render::render::widget::*;
use pushrod_render::render::widget_config::WidgetConfig;
use pushrod_render::render::callbacks::CallbackRegistry;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::ttf::{FontStyle, Sdl2TtfContext, Font};

use std::collections::HashMap;
use std::path::Path;
use pushrod_render::render::widget_cache::WidgetContainer;

pub enum TextJustify {
    /// Left-justified text.
    Left,

    /// Center-justified text: `(total width - text width) / 2`
    Center,

    /// Right-justified text: `(total width - text width)`
    Right,
}

pub struct TextWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    font_style: FontStyle,
    font_size: i32,
    msg: String,
}

impl TextWidget {
    fn new(font_name: String, font_style: FontStyle, font_size: i32, msg: String, x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            font_style,
            font_size,
            msg: msg.clone(),
        }
    }
}

impl Widget for TextWidget {
    fn draw(&mut self, mut _canvas: &mut Canvas<Window>) {
//        let base_color = *self
//            .config
//            .colors
//            .get(&COLOR_BASE)
//            .unwrap_or(&Color::RGB(255, 255, 255));
//        let border_color = *self.config.colors.get(&COLOR_BORDER).unwrap_or(&base_color);
//
//        _canvas.set_draw_color(base_color);
//
//        _canvas.fill_rect(self.get_drawing_area()).unwrap();
//
//        if self.get_config().border_width > 0 && base_color != border_color {
//            _canvas.set_draw_color(border_color);
//
//            for border in 0..self.get_config().border_width {
//                _canvas
//                    .draw_rect(Rect::new(
//                        self.config.to_x(i32::from(border)),
//                        self.config.to_y(i32::from(border)),
//                        self.get_config().size[0] - (u32::from(border) * 2),
//                        self.get_config().size[1] - (u32::from(border) * 2),
//                    ))
//                    .unwrap();
//            }
//        }
    }

    /// This function is a macro-created getter function that returns the `Widget`'s configuration
    /// object as a borrowed mutable reference.  This code is auto-generated using the
    /// `default_widget_properties!()` macro.
    fn get_config(&mut self) -> &mut WidgetConfig {
        &mut self.config
    }

    /// This function is a macro-created getter function that returns the `Widget`'s system
    /// properties as a borrowed mutable reference.  This code is auto-generated using the
    /// `default_widget_properties!()` macro.
    fn get_system_properties(&mut self) -> &mut HashMap<i32, String> {
        &mut self.system_properties
    }

    /// This function is a macro-created getter function that returns the `Widget`'s
    /// `CallbackRegistry` object as a borrowed mutable reference.  This code is auto-generated
    /// using the `default_widget_properties!()` macro.
    fn get_callbacks(&mut self) -> &mut CallbackRegistry {
        &mut self.callback_registry
    }

    /// This function is a macro-created tick callback override, created by the
    /// `default_widget_callbacks!()` macro.
    fn tick_callback(&mut self, _widgets: &[WidgetContainer]) {
        if self.get_callbacks().has_on_tick() {
            if let Some(mut cb) = self.get_callbacks().on_tick.take() {
                cb(self, _widgets);
                self.get_callbacks().on_tick = Some(cb);
            }
        }
    }

    /// This function is a macro-created mouse entered callback override, created by the
    /// `default_widget_callbacks!()` macro.
    fn mouse_entered_callback(&mut self, _widgets: &[WidgetContainer]) {
        if self.get_callbacks().has_on_mouse_entered() {
            if let Some(mut cb) = self.get_callbacks().on_mouse_entered.take() {
                cb(self, _widgets);
                self.get_callbacks().on_mouse_entered = Some(cb);
            }
        }
    }

    /// This function is a macro-created mouse exited callback override, created by the
    /// `default_widget_callbacks!()` macro.
    fn mouse_exited_callback(&mut self, _widgets: &[WidgetContainer]) {
        if self.get_callbacks().has_on_mouse_exited() {
            if let Some(mut cb) = self.get_callbacks().on_mouse_exited.take() {
                cb(self, _widgets);
                self.get_callbacks().on_mouse_exited = Some(cb);
            }
        }
    }

    /// This function is a macro-created mouse moved callback override, created by the
    /// `default_widget_callbacks!()` macro.
    fn mouse_moved_callback(&mut self, _widgets: &[WidgetContainer], _points: Vec<i32>) {
        if self.get_callbacks().has_on_mouse_moved() {
            if let Some(mut cb) = self.get_callbacks().on_mouse_moved.take() {
                cb(self, _widgets, _points);
                self.get_callbacks().on_mouse_moved = Some(cb);
            }
        }
    }

    /// This function is a macro-created mouse scrolled callback override, created by the
    /// `default_widget_callbacks!()` macro.
    fn mouse_scrolled_callback(&mut self, _widgets: &[WidgetContainer], _points: Vec<i32>) {
        if self.get_callbacks().has_on_mouse_scrolled() {
            if let Some(mut cb) = self.get_callbacks().on_mouse_scrolled.take() {
                cb(self, _widgets, _points);
                self.get_callbacks().on_mouse_scrolled = Some(cb);
            }
        }
    }

    /// This function is a macro-created mouse scrolled callback override, created by the
    /// `default_widget_callbacks!()` macro.
    fn button_clicked_callback(&mut self, _widgets: &[WidgetContainer], _button: u8, _clicks: u8, _state: bool) {
        if self.get_callbacks().has_on_mouse_clicked() {
            if let Some(mut cb) = self.get_callbacks().on_mouse_clicked.take() {
                cb(self, _widgets, _button, _clicks, _state);
                self.get_callbacks().on_mouse_clicked = Some(cb);
            }
        }
    }
}