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
use crate::render::layout_cache::LayoutContainer;
use crate::render::widget::*;
use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::*;
use crate::render::{Points, Size};

use sdl2::render::{Canvas, Texture, TextureQuery};
use sdl2::ttf::FontStyle;
use sdl2::video::Window;

use crate::render::texture_cache::TextureCache;
use crate::render::texture_store::TextureStore;
use sdl2::rect::Rect;
use std::any::Any;
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
    texture_store: TextureStore,
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
        points: Points,
        size: Size,
    ) -> Self {
        Self {
            config: WidgetConfig::new(points, size),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            texture_store: TextureStore::default(),
            font_name,
            font_style,
            font_size,
            justification,
            msg,
        }
    }

    /// Changes the text displayed in the body of the `Widget`.
    pub fn set_text(&mut self, msg: String) {
        self.msg = msg;
        self.get_config().set_invalidated(true);
    }

    /// Retrieves the text currently being displayed in the `TextWidget`.
    pub fn get_text(&self) -> String {
        self.msg.clone()
    }
}

/// This is the `Widget` implementation of the `TextWidget`.  Text is rendered onto a 3D texture, then
/// copied to the canvas after rendering.  It uses blended mode texture mapping, which may be slow (as
/// described by the SDL2 documentation), so this might change later to use 8 bit color mapping.
impl Widget for TextWidget {
    fn draw(&mut self, c: &mut Canvas<Window>, t: &mut TextureCache) -> Option<&Texture> {
        if self.get_config().invalidated() {
            let bounds = self.get_config().get_size(CONFIG_SIZE);

            self.texture_store
                .create_or_resize_texture(c, bounds[0] as u32, bounds[1] as u32);

            let base_color = self.get_color(CONFIG_COLOR_BASE);
            let text_max_width = self.get_size(CONFIG_SIZE)[0]
                - ((self.get_numeric(CONFIG_BORDER_WIDTH) * 2) as u32);

            let ttf_context = t.get_ttf_context();
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
            let font_texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();

            let TextureQuery { width, height, .. } = font_texture.query();
            let texture_y = 0;
            let widget_w = self.get_size(CONFIG_SIZE)[0] as i32;
            let texture_x = match self.justification {
                TextJustify::Left => 0,
                TextJustify::Right => widget_w - width as i32,
                TextJustify::Center => (widget_w - width as i32) / 2,
            };

            c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
                texture.set_draw_color(base_color);
                texture.clear();

                texture
                    .copy(
                        &font_texture,
                        None,
                        Rect::new(texture_x, texture_y, width, height),
                    )
                    .unwrap();
            })
            .unwrap();
        }

        self.texture_store.get_optional_ref()
    }

    /// Monitors for changes in the text, color changes, or font sizes.
    fn on_config_changed(&mut self, _k: u8, _v: Config) {
        match _k {
            CONFIG_COLOR_TEXT => self.get_config().set_invalidated(true),
            CONFIG_COLOR_BASE => self.get_config().set_invalidated(true),
            CONFIG_FONT_SIZE => {
                if let Config::Numeric(size) = _v {
                    self.font_size = size;
                    self.get_config().set_invalidated(true);
                }
            }
            CONFIG_TEXT => {
                if let Config::Text(text) = _v {
                    self.msg = text;
                    self.get_config().set_invalidated(true);
                }
            }

            _ => (),
        };
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
