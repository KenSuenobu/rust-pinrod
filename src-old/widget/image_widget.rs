// Image Widget
// Draws an image in a specified bounding area.
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

use graphics::*;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::*;

use crate::core::callbacks::*;
use crate::core::widget_store::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// Draws an image.
pub struct ImageWidget {
    config: Configurable,
    image: Texture,
    image_size: crate::core::point::Size,
    widget_id: i32,
    callbacks: DefaultWidgetCallbacks,
}

impl ImageWidget {
    /// Constructor.  Requires the name of the image to be drawn.  The image is loaded locally from
    /// the `assets/` directory of your application.
    pub fn new(image_name: String) -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let texture =
            Texture::from_path(&assets.join(image_name), &TextureSettings::new()).unwrap();
        let texture_width = texture.get_width() as i32;
        let texture_height = texture.get_height() as i32;

        Self {
            config: Configurable::new(),
            image: texture,
            image_size: crate::core::point::Size {
                w: texture_width,
                h: texture_height,
            },
            widget_id: 0,
            callbacks: DefaultWidgetCallbacks::new(),
        }
    }

    inject_event_handler!();
}

impl Drawable for ImageWidget {
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size = self.config().get_size(CONFIG_BODY_SIZE);
        let transform = c.transform.scale(
            size.w as f64 / self.image_size.w as f64,
            size.h as f64 / self.image_size.h as f64,
        );

        Image::new().draw(&self.image, clip, transform, g);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}

impl InjectableSystemEvents for ImageWidget {}

impl InjectableCustomEvents for ImageWidget {}

impl Widget for ImageWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_widget_id(&mut self, widget_id: i32) {
        self.widget_id = widget_id;
    }

    fn get_widget_id(&mut self) -> i32 {
        self.widget_id
    }

    fn handle_event(
        &mut self,
        injected: bool,
        _event: CallbackEvent,
        _widget_store: Option<&Vec<WidgetContainer>>,
    ) -> Option<CallbackEvent> {
        if !injected {
            self.handle_event_callbacks(_event, _widget_store);
        }

        None
    }

    fn handles_events(&mut self) -> bool {
        true
    }

    fn get_injectable_custom_events(&mut self) -> &mut dyn InjectableCustomEvents {
        self
    }

    fn get_injectable_system_events(&mut self) -> &mut dyn InjectableSystemEvents {
        self
    }

    fn get_drawable(&mut self) -> &mut dyn Drawable {
        self
    }

    fn get_callbacks(&mut self) -> &mut DefaultWidgetCallbacks {
        &mut self.callbacks
    }
}
