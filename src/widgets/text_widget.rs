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

use crate::render::callbacks::CallbackRegistry;
use crate::render::widget::*;
use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::{WidgetConfig, COLOR_TEXT};

use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::FontStyle;
use sdl2::video::Window;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::path::Path;

/// This enum is used by the `TextWidget`, which controls the justification of the text being
/// rendered within the bounds of the `Widget`.
pub enum TextJustify {
    /// Left-justified text.
    Left,

    /// Center-justified text: `(total width - text width) / 2`
    Center,

    /// Right-justified text: `(total width - text width)`
    Right,
}

/// This is the storage object for the `TextWidget`.  It stores the config, properties, callback registry,
/// the font name, style, size, justification, and text message.
pub struct TextWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    font_name: String,
    font_style: FontStyle,
    font_size: i32,
    justification: TextJustify,
    msg: String,
}

/// Creates a new `TextWidget`, which draws a unit of text on the screen, given the specified font,
/// size, justification, and layout coordinates.
impl TextWidget {
    /// Creates a new `TextWidget` object.  Requires the name of the font (the path to the font file),
    /// the style of font (`sdl2::ttf::FontStyle`), the size in pixels of the font, the `TextJustify`
    /// layout of the font, the message to display, and the x, y, w, h coordinates of the text.
    pub fn new(
        font_name: String,
        font_style: FontStyle,
        font_size: i32,
        justification: TextJustify,
        msg: String,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> Self {
        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            font_name,
            font_style,
            font_size,
            justification,
            msg: msg.clone(),
        }
    }
}

/// This is the `Widget` implementation of the `TextWidget`.  Text is rendered onto a 3D texture, then
/// copied to the canvas after rendering.  It uses blended mode texture mapping, which may be slow (as
/// described by the SDL2 documentation), so this might change later to use 8 bit color mapping.
impl Widget for TextWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) {
        let base_color = Color::RGB(255, 255, 255);

        c.set_draw_color(base_color);
        c.fill_rect(self.get_drawing_area()).unwrap();

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let texture_creator = c.texture_creator();
        let mut font = ttf_context
            .load_font(Path::new(&self.font_name), self.font_size as u16)
            .unwrap();

        font.set_style(self.font_style);

        let surface = font
            .render(&self.msg)
            .blended(
                *self
                    .config
                    .colors
                    .get(&COLOR_TEXT)
                    .unwrap_or(&Color::RGB(0, 0, 0)),
            )
            .map_err(|e| e.to_string())
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let texture_y = self.get_config().to_y(0);
        let widget_w = self.get_config().size[0] as i32;
        let texture_x = match self.justification {
            TextJustify::Left => self.get_config().to_x(0),

            TextJustify::Right => self.get_config().to_x(widget_w - width as i32),

            TextJustify::Center => self.get_config().to_x((widget_w - width as i32) / 2),
        };

        c.copy(&texture,
        None,
        Rect::new(texture_x, texture_y, width, height));
    }

    // FIXME USE MACROS HERE -- They didn't work before!

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
    fn button_clicked_callback(
        &mut self,
        _widgets: &[WidgetContainer],
        _button: u8,
        _clicks: u8,
        _state: bool,
    ) {
        if self.get_callbacks().has_on_mouse_clicked() {
            if let Some(mut cb) = self.get_callbacks().on_mouse_clicked.take() {
                cb(self, _widgets, _button, _clicks, _state);
                self.get_callbacks().on_mouse_clicked = Some(cb);
            }
        }
    }
}
