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

use crate::render::layout::{Layout, LayoutPosition};
use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::CONFIG_SIZE;
use crate::render::{Points, Size, SIZE_HEIGHT, SIZE_WIDTH};
use std::any::Any;

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
        let widget_width = self.size[SIZE_WIDTH] / num_widgets as u32;
        let subtractor_right = self.spacing / 2;
        let subtractor_left = subtractor_right - 1;

        eprintln!(
            "HorizontalLayout: rightside={} leftside={}",
            subtractor_right, subtractor_left
        );

        for i in 0..num_widgets {
            let set_x: i32;
            let mut set_width: u32 = widget_width;
            let widget_id = self.widget_ids[i as usize];

            if i == 0 {
                set_x = (i * set_width) as i32;
                set_width = widget_width - subtractor_right;
            } else if i == num_widgets - 1 {
                set_x = (i * set_width) as i32 - subtractor_left as i32;
                set_width = widget_width - subtractor_left;
            } else {
                set_x = (i * set_width) as i32 - subtractor_left as i32;
                set_width = widget_width - subtractor_left - subtractor_right;
            }

            _widgets[widget_id as usize]
                .widget
                .borrow_mut()
                .get_config()
                .to_x(set_x);

            let widget_size = _widgets[widget_id as usize]
                .widget
                .borrow_mut()
                .get_config()
                .get_size(CONFIG_SIZE);

            _widgets[widget_id as usize]
                .widget
                .borrow_mut()
                .get_config()
                .set_size(CONFIG_SIZE, set_width, self.size[SIZE_HEIGHT]);
        }
    }
}
