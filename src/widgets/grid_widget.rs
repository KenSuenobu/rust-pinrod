// Pushrod Widget Library
// Grid Widget
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
use crate::render::{Points, Size, SIZE_HEIGHT, SIZE_WIDTH};

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::render::layout_cache::LayoutContainer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::any::Any;
use std::collections::HashMap;

/// This is the storage object for the `GridWidget`.  It stores the config, properties, callback registry.
pub struct GridWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    grid_size: u32,
}

/// This is the implementation of the `GridWidget`, a control that displays a grid inside its bounds.
impl GridWidget {
    /// Creates a new `GridWidget` given the `x, y, w, h` coordinates, sets the grid size.
    pub fn new(points: Points, size: Size, grid_size: u32) -> Self {
        Self {
            config: WidgetConfig::new(points.clone(), size.clone()),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            grid_size,
        }
    }

    /// Private function that draws the grid.
    fn draw_grid(&mut self, c: &mut Canvas<Window>) {
        let size = self.get_config().get_size(CONFIG_SIZE);

        c.set_draw_color(Color::RGB(192, 192, 192));

        for i in (0..size[SIZE_WIDTH]).step_by(self.grid_size as usize) {
            c.draw_line(
                Point::new(self.get_config().to_x(i as i32), self.get_config().to_y(0)),
                Point::new(
                    self.get_config().to_x(i as i32),
                    self.get_config().to_y(size[SIZE_HEIGHT] as i32),
                ),
            )
            .unwrap();
        }

        for i in (0..size[SIZE_HEIGHT]).step_by(self.grid_size as usize) {
            c.draw_line(
                Point::new(self.get_config().to_x(0), self.get_config().to_y(i as i32)),
                Point::new(
                    self.get_config().to_x(size[SIZE_WIDTH] as i32),
                    self.get_config().to_y(i as i32),
                ),
            )
            .unwrap();
        }
    }

    /// Adjusts the size of the grid, redrawing the object.
    pub fn set_grid_size(&mut self, grid_size: u32) {
        self.grid_size = grid_size;
        self.get_config().set_invalidated(true);
    }
}

/// This is the `Widget` implementation of the `GridWidget`.
impl Widget for GridWidget {
    /// Draws the `GridWidget` contents.
    fn draw(&mut self, c: &mut Canvas<Window>) {
        let base_color = self.get_color(CONFIG_COLOR_BASE);

        c.set_draw_color(base_color);
        c.fill_rect(self.get_drawing_area()).unwrap();

        self.draw_grid(c);

        let border_color = self.get_config().get_color(CONFIG_COLOR_BORDER);

        if self.get_config().get_numeric(CONFIG_BORDER_WIDTH) > 0 && base_color != border_color {
            c.set_draw_color(border_color);

            for border in 0..self.get_config().get_numeric(CONFIG_BORDER_WIDTH) {
                c.draw_rect(Rect::new(
                    self.config.to_x(border),
                    self.config.to_y(border),
                    self.get_config().get_size(CONFIG_SIZE)[0] - (border as u32 * 2),
                    self.get_config().get_size(CONFIG_SIZE)[1] - (border as u32 * 2),
                ))
                .unwrap();
            }
        }
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
