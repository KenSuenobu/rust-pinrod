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
use crate::render::widget::*;
use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::{WidgetConfig, COLOR_BASE, COLOR_BORDER, COLOR_SECONDARY};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;

use sdl2::render::Canvas;
use std::collections::HashMap;

/// This is the storage object for the `ProgressWidget`.  It stores the config, properties, callback registry,
/// the base widget, and progress from 0 to 100.
pub struct ProgressWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    base_widget: BaseWidget,
    progress: u16,
}

/// Creates a new `ProgressWidget`, which draws a progress bar inside a `BaseWidget`.
impl ProgressWidget {
    /// Creates a new instance of the `ImageWidget` object.  Requires an image name (full path of the file),
    /// image position (defined in `ImagePosition`), the `xywh` bounds of the `Widget`, and a scale flag.
    /// If `scaled` is set to `true`, the image will be scaled within the `Widget` bounds, and the
    /// `ImagePosition` will be ignored.  Likewise, if set to `false`, the image will be displayed for
    /// the size of the image, and will be placed in the bounds of the `Widget` based on the position
    /// specified in the `ImagePosition`.
    pub fn new(x: i32, y: i32, w: u32, h: u32, progress: u16) -> Self {
        let mut base_widget = BaseWidget::new(x, y, w, h);

        base_widget
            .get_config()
            .colors
            .insert(COLOR_BASE, Color::RGB(255, 255, 255));

        base_widget
            .get_config()
            .colors
            .insert(COLOR_BORDER, Color::RGB(0, 0, 0));

        base_widget.get_config().border_width = 1;

        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            base_widget,
            progress,
        }
    }

    /// Sets the progress of this `ProgressWidget`, redrawing after setting.  Anything larger than a
    /// value of 100 will be locked at 100.
    pub fn set_progress(&mut self, progress: u16) {
        if progress > 100 {
            self.progress = 100;
        }

        if progress != self.progress {
            self.progress = progress;
            self.get_config().set_invalidate(true);
        }
    }

    /// Retrieves the progress setting for this `ProgressWidget`.
    pub fn get_progress(&self) -> u16 {
        self.progress
    }
}

/// This is the `Widget` implementation of the `ImageWidget`.  Image is rendered onto a 3D texture, then
/// copied to the canvas after rendering.
impl Widget for ProgressWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) {
        self.base_widget.draw(c);

        let base_color = *self
            .config
            .colors
            .get(&COLOR_SECONDARY)
            .unwrap_or(&Color::RGB(0, 0, 0));
        let progress =
            (self.get_config().size[0] as f64 * (self.progress as f64 / 100.0) as f64) as u32;

        c.set_draw_color(base_color);
        c.fill_rect(Rect::new(
            self.config.to_x(1),
            self.config.to_y(1),
            progress,
            self.get_config().size[1] - 2,
        ))
        .unwrap();
    }

    default_widget_properties!();
    default_widget_callbacks!();
}
