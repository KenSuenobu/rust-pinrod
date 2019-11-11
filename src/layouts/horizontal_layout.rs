// Pushrod Rendering Library
// Horizontal Layout Manager
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

use crate::render::widget_cache::WidgetContainer;
use crate::render::layout::{LayoutPosition, Layout};
use crate::render::{Points, Size};

/// This is the `HorizontalLayout` storage structure for the `HorizontalLayout` implementation.
pub struct HorizontalLayout {
    widget_ids: Vec<i32>,
    widget_positions: Vec<LayoutPosition>,
    origin: Points,
    size: Size,
    spacing: u32,
}

/// Creates a new `HorizontalLayout` manager.
impl HorizontalLayout {
    pub fn new(x: i32, y: i32, w: u32, h: u32, spacing: u32) -> Self {
        Self {
            widget_ids: Vec::new(),
            widget_positions: Vec::new(),
            origin: vec![x, y],
            size: vec![w, h],
            spacing,
        }
    }
}

/// This is the `Layout` implementation for the `HorizontalLayout` manager.
impl Layout for HorizontalLayout {
    /// Adds a widget to the `HorizontalLayout` managed stack.
    fn add_widget(&mut self, widget_id: i32, widget_position: LayoutPosition) {
        self.widget_ids.push(widget_id);
        self.widget_positions.push(widget_position);
    }

    /// Adjusts the layout of the `Widget`s managed by this `Layout` manager.
    fn do_layout(&mut self, _widgets: &[WidgetContainer]) {
        if self.widget_ids.len() <= 1 {
            return;
        }

        let num_widgets = self.widget_ids.len() as u32;
        let widget_width = self.size[0] / num_widgets as u32;
        let subtractor_right = self.spacing / 2;
        let subtractor_left = subtractor_right - 1;

        for i in 0..num_widgets {
            if i == 0 {
                // skip subtractor left
                // apply only subtractor right
            } else if i == num_widgets - 1 {
                // skip subtractor right
                // apply only subtractor left
            } else {
                // apply subtractor left
                // apply subtractor right
            }
        }
    }
}