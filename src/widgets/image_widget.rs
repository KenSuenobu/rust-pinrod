// Pushrod Widget Library
// Image Widget
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
use crate::render::widget_config::{
    CompassPosition, Config, WidgetConfig, CONFIG_COLOR_BASE, CONFIG_IMAGE_POSITION, CONFIG_SIZE,
};
use crate::render::Points;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

use std::any::Any;
use std::collections::HashMap;
use std::path::Path;

/// This is the storage object for the `TextWidget`.  It stores the config, properties, callback registry,
/// the font name, style, size, justification, and text message.
pub struct ImageWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    image_name: String,
    scaled: bool,
}

/// Creates a new `ImageWidget`, which draws an image in a supported image format for SDL2 at a specific
/// location on the screen.  Requires the name of the image (the full path to the file), the position
/// within the widget (defined as `ImagePosition`), the xywh bounds, and whether or not the image is
/// scaled within the bounds of the `Widget`.
impl ImageWidget {
    /// Creates a new instance of the `ImageWidget` object.  Requires an image name (full path of the file),
    /// image position (defined in `ImagePosition`), the `xywh` bounds of the `Widget`, and a scale flag.
    /// If `scaled` is set to `true`, the image will be scaled within the `Widget` bounds, and the
    /// `ImagePosition` will be ignored.  Likewise, if set to `false`, the image will be displayed for
    /// the size of the image, and will be placed in the bounds of the `Widget` based on the position
    /// specified in the `ImagePosition`.
    pub fn new(image_name: String, x: i32, y: i32, w: u32, h: u32, scaled: bool) -> Self {
        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            image_name,
            scaled,
        }
    }
}

/// This is the `Widget` implementation of the `ImageWidget`.  Image is rendered onto a 3D texture, then
/// copied to the canvas after rendering.
impl Widget for ImageWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) {
        let base_color = self.get_color(CONFIG_COLOR_BASE);

        c.set_draw_color(base_color);
        c.fill_rect(self.get_drawing_area()).unwrap();

        let texture_creator = c.texture_creator();
        let texture = texture_creator
            .load_texture(Path::new(&self.image_name))
            .unwrap();
        let widget_w = self.get_size(CONFIG_SIZE)[0] as i32;
        let widget_h = self.get_size(CONFIG_SIZE)[1] as i32;
        let TextureQuery { width, height, .. } = texture.query();

        let texture_x = match self.get_compass(CONFIG_IMAGE_POSITION) {
            CompassPosition::NW | CompassPosition::W | CompassPosition::SW => {
                self.get_config().to_x(0)
            }

            CompassPosition::N | CompassPosition::Center | CompassPosition::S => {
                self.get_config().to_x((widget_w - width as i32) / 2)
            }

            CompassPosition::NE | CompassPosition::E | CompassPosition::SE => {
                self.get_config().to_x(widget_w - width as i32)
            }
        };

        let texture_y = match self.get_compass(CONFIG_IMAGE_POSITION) {
            CompassPosition::NW | CompassPosition::N | CompassPosition::NE => {
                self.get_config().to_y(0)
            }

            CompassPosition::W | CompassPosition::Center | CompassPosition::E => {
                self.get_config().to_y((widget_h - height as i32) / 2)
            }

            CompassPosition::SW | CompassPosition::S | CompassPosition::SE => {
                self.get_config().to_y(widget_h - height as i32)
            }
        };

        if !self.scaled {
            c.copy(
                &texture,
                None,
                Rect::new(texture_x, texture_y, width, height),
            )
            .unwrap();
        } else {
            c.copy(
                &texture,
                None,
                Rect::new(
                    self.get_config().to_x(0),
                    self.get_config().to_y(0),
                    widget_w as u32,
                    widget_h as u32,
                ),
            )
            .unwrap();
        }
    }

    /// Responds to a screen redraw only if the `CONFIG_IMAGE_POSITION` key was changed.
    fn on_config_changed(&mut self, _k: u8, _v: Config) {
        if _k == CONFIG_IMAGE_POSITION {
            self.get_config().set_invalidate(true);
        }
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
