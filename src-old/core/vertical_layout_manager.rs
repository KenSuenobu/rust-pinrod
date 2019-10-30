// Vertical Layout Manager
// Lays out Widgets Vertically in a Bounding Box
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

use crate::core::layout_manager::*;
use crate::core::point::{Point, Size};

pub struct VerticalLayoutManager {
    container_widget_id: i32,
    padding: LayoutManagerPadding,
}

impl VerticalLayoutManager {
    pub fn new(widget_id: i32, padding: LayoutManagerPadding) -> Self {
        Self {
            container_widget_id: widget_id,
            padding,
        }
    }
}

impl LayoutManager for VerticalLayoutManager {
    fn do_layout(
        &mut self,
        origin: Point,
        size: Size,
        coordinates: LayoutManagerCoordinates,
    ) -> LayoutManagerCoordinates {
        let num_widgets = coordinates.widget_sizes.len() as i32;
        let height_per_widget = (size.h / num_widgets) - (self.padding.spacing / 2);
        let mut widget_origins: Vec<Point> = vec![];
        let mut widget_sizes: Vec<Size> = vec![];
        let current_x: i32 = origin[0];
        let mut current_y: i32 = origin[1];

        for x in 0..num_widgets {
            if x == 0 {
                current_y = self.padding.top + origin[1];
            } else {
                current_y += height_per_widget + (self.padding.spacing / 2);
            }

            widget_origins.push([
                current_x + self.padding.left,
                current_y,
            ]);

            if x == num_widgets - 1 {
                widget_sizes.push(Size {
                    w: size.w - (self.padding.left + self.padding.right),
                    h: height_per_widget - self.padding.bottom,
                });
            } else {
                widget_sizes.push(Size {
                    w: size.w - (self.padding.left + self.padding.right),
                    h: height_per_widget - (self.padding.spacing / 2),
                });
            }
        }

        LayoutManagerCoordinates {
            widget_origins: widget_origins.clone(),
            widget_sizes: widget_sizes.clone(),
            widget_positions: coordinates.widget_positions.clone(),
        }
    }

    fn adjust_layout(&mut self, coordinates: LayoutManagerPadding) {
        self.padding = coordinates.clone()
    }

    fn get_widget_id(&self) -> i32 {
        self.container_widget_id
    }
}
