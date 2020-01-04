// Pushrod Widget Library
// Progress Widget
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

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;

use crate::render::canvas_helper::CanvasHelper;
use sdl2::render::{Canvas, Texture};
use std::any::Any;
use std::collections::HashMap;

/// This is the storage object for the `ProgressWidget`.  It stores the config, properties, callback registry,
/// the base widget, and progress from 0 to 100.
pub struct ProgressWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    progress: u8,
    canvas_texture: Option<Texture>,
}

/// Creates a new `ProgressWidget`, which draws a progress bar inside a `BaseWidget`.
impl ProgressWidget {
    /// Creates a new instance of the `ProgressWidget` object.  It draws a progress bar-style
    /// `Widget` given the `xywh` coordinates, and the `percentage` of fill from 0-100.  The
    /// base color and border colors are set to white and black, respectively.  Use the
    /// `COLOR_SECONDARY` setting to change the color of the fill for the progress bar.
    pub fn new(points: Points, size: Size, progress: u8) -> Self {
        Self {
            config: WidgetConfig::new(points, size),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            progress,
            canvas_texture: None,
        }
    }

    /// Sets the progress for the widget.  Progress value is between 0 and 100.  Anything over
    /// 100 will just set the progress to 100.
    pub fn set_progress(&mut self, progress: u8) {
        if progress > 100 {
            self.progress = 100;
        } else {
            self.progress = progress;
        }

        self.get_config().set_invalidated(true);
    }

    /// Retrieves the current progress value as a `u8` value.
    pub fn get_progress(&mut self) -> u8 {
        self.progress
    }

    /// Creates a drawable `Texture` that can be drawn against, instead of drawing directly to the
    /// canvas.  This way, the canvas is not refreshed, only the `Texture` is drawn.
    fn create_texture(&mut self, c: &mut Canvas<Window>) {
        if self.canvas_texture.is_none() {
            let widget_width = self.get_config().get_size(CONFIG_SIZE)[0];
            let widget_height = self.get_config().get_size(CONFIG_SIZE)[1];
            self.canvas_texture = Some(
                c.create_texture_target(None, widget_width, widget_height)
                    .unwrap(),
            );
            eprintln!(
                "Creating canvas texture: {}x{}",
                widget_width, widget_height
            );
        }
    }
}

impl CanvasHelper for ProgressWidget {}

/// This is the `Widget` implementation of the `ProgressWidget`.  It contains a `BaseWidget` within
/// its bounds to draw the base background, then draws the progress fill over the top.
impl Widget for ProgressWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) {
        self.create_texture(c);

        if self.get_config().invalidated() {
            let base_color = self.get_color(CONFIG_COLOR_SECONDARY);
            let progress_width = (f64::from(self.get_size(CONFIG_SIZE)[0])
                * (f64::from(self.progress))
                / 100.0) as u32;
            let progress_height = self.get_size(CONFIG_SIZE)[1] - 2;
            let border_color = self.get_config().get_color(CONFIG_COLOR_BORDER);
            let bounds = self.get_config().get_size(CONFIG_SIZE);

            match &mut self.canvas_texture {
                Some(ref mut ref_texture) => {
                    c.with_texture_canvas(ref_texture, |texture| {
                        texture.set_draw_color(Color::RGB(255, 255, 255));
                        texture.clear();

                        texture.set_draw_color(base_color);
                        texture
                            .fill_rect(Rect::new(1, 1, progress_width, progress_height))
                            .unwrap();

                        texture.set_draw_color(border_color);
                        texture
                            .draw_rect(Rect::new(0, 0, bounds[0], bounds[1]))
                            .unwrap();
                    })
                    .unwrap();
                }
                None => (),
            }
        }

        let draw_rect = self.get_rect_dest();

        match &self.canvas_texture {
            Some(ref x) => c.copy(x, None, draw_rect).unwrap(),
            None => {}
        };
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
