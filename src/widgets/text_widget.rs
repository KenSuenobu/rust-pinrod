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
use crate::render::widget_config::*;
use crate::render::Points;

use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::FontStyle;
use sdl2::video::Window;

use sdl2::rect::Rect;
use std::collections::HashMap;
use std::path::Path;
use std::any::Any;

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
        let base_color = self.get_color(CONFIG_COLOR_BASE);
        let text_max_width =
            self.get_size(CONFIG_SIZE)[0] - ((self.get_numeric(CONFIG_BORDER_WIDTH) * 2) as u32);

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let texture_creator = c.texture_creator();
        let mut font = ttf_context
            .load_font(Path::new(&self.font_name), self.font_size as u16)
            .unwrap();
        let font_color = self.get_color(CONFIG_COLOR_TEXT);

        font.set_style(self.font_style);

        let surface = font
            .render(&self.msg)
            .blended_wrapped(font_color, text_max_width)
            .map_err(|e| e.to_string())
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let texture_y = self.get_config().to_y(0);
        let widget_w = self.get_size(CONFIG_SIZE)[0] as i32;
        let texture_x = match self.justification {
            TextJustify::Left => self.get_config().to_x(0),

            TextJustify::Right => self.get_config().to_x(widget_w - width as i32),

            TextJustify::Center => self.get_config().to_x((widget_w - width as i32) / 2),
        };

        c.set_draw_color(base_color);
        c.fill_rect(self.get_drawing_area()).unwrap();

        c.copy(
            &texture,
            None,
            Rect::new(texture_x, texture_y, width, height),
        )
        .unwrap();
    }

    /// Monitors for changes in the text, color changes, or font sizes.
    fn on_config_changed(&mut self, _k: u8, _v: Config) {
        match _k {
            CONFIG_COLOR_TEXT => self.get_config().set_invalidate(true),
            CONFIG_COLOR_BASE => self.get_config().set_invalidate(true),
            CONFIG_FONT_SIZE => {
                if let Config::Numeric(size) = _v {
                    self.font_size = size;
                    self.get_config().set_invalidate(true);
                }
            }
            CONFIG_TEXT => {
                if let Config::Text(text) = _v {
                    self.msg = text.clone();
                    self.get_config().set_invalidate(true);
                }
            }

            _ => (),
        };
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
