// Pushrod Rendering Library
// Vertical Layout Manager
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
use crate::render::widget_config::{PaddingConstraint, CONFIG_ORIGIN, CONFIG_SIZE};
use crate::render::{Points, Size, SIZE_HEIGHT, SIZE_WIDTH};

/// This is the `VerticalLayout` storage structure for the `VerticalLayout` implementation.
pub struct VerticalLayout {
    widget_ids: Vec<i32>,
    widget_positions: Vec<LayoutPosition>,
    origin: Points,
    size: Size,
    padding: PaddingConstraint,
    invalidated: bool,
}

/// Creates a new `VerticalLayout` manager.
impl VerticalLayout {
    pub fn new(x: i32, y: i32, w: u32, h: u32, padding: PaddingConstraint) -> Self {
        Self {
            widget_ids: Vec::new(),
            widget_positions: Vec::new(),
            origin: vec![x, y],
            size: vec![w, h],
            padding,
            invalidated: false,
        }
    }
}

/// This is the `Layout` implementation for the `VerticalLayout` manager.  This `Layout` manager will
/// not reposition any objects within the bounds of the `Layout` until at least 2 objects have been
/// added to the bounds of the `Layout`.
impl Layout for VerticalLayout {
    /// Adds a widget to the `VerticalLayout` managed stack.
    fn insert_widget(&mut self, widget_id: i32, widget_position: LayoutPosition) {
        self.widget_ids.push(widget_id);
        self.widget_positions.push(widget_position);
        self.invalidated = true;
    }

    /// Appends a widget to the `VerticalLayout` managed stack.
    fn append_widget(&mut self, widget_id: i32) {
        let positions = self.widget_positions.len();
        let widget_position = if self.widget_positions.is_empty() {
            LayoutPosition::new(0, 0)
        } else {
            LayoutPosition::new(self.widget_positions[positions - 1].x + 1, 0)
        };

        self.insert_widget(widget_id, widget_position);
    }

    fn set_padding(&mut self, padding: PaddingConstraint) {
        self.padding = padding.clone();
        self.invalidated = true;
    }

    fn get_padding(&self) -> PaddingConstraint {
        self.padding.clone()
    }

    /// Adjusts the layout of the `Widget`s managed by this `Layout` manager.  Currently only obeys
    /// the spacing in the object.  The rest of the padding is not (yet) honored.
    fn do_layout(&mut self, _widgets: &[WidgetContainer]) {
        if self.widget_ids.len() <= 1 {
            return;
        }

        let offset_x: i32 = self.origin[0];
        let offset_y: i32 = self.origin[1];
        let num_widgets = self.widget_ids.len() as u32;
        let widget_height = self.size[SIZE_HEIGHT] / num_widgets as u32;
        let subtractor_bottom = ((self.padding.spacing as f64 / 2.0).ceil()) as u32;
        let subtractor_top = ((self.padding.spacing as f64 / 2.0).floor()) as u32;

        for i in 0..num_widgets {
            let set_y: i32;
            let mut set_height: u32 = widget_height;
            let widget_id = self.widget_ids[i as usize];

            if i == 0 {
                set_y = (i * set_height) as i32 + self.padding.top;
                set_height = widget_height - subtractor_bottom - self.padding.top as u32;
            } else if i == num_widgets - 1 {
                set_y = (i * set_height) as i32 + subtractor_top as i32;
                set_height = widget_height - subtractor_top - self.padding.bottom as u32;
            } else {
                set_y = (i * set_height) as i32 + subtractor_top as i32;
                set_height = widget_height - subtractor_top - subtractor_bottom;
            }

            _widgets[widget_id as usize]
                .widget
                .borrow_mut()
                .get_config()
                .set_point(
                    CONFIG_ORIGIN,
                    offset_x + self.padding.left,
                    offset_y + set_y,
                );

            _widgets[widget_id as usize]
                .widget
                .borrow_mut()
                .get_config()
                .set_size(
                    CONFIG_SIZE,
                    self.size[SIZE_WIDTH] - self.padding.right as u32 - self.padding.left as u32,
                    set_height,
                );

            _widgets[widget_id as usize]
                .widget
                .borrow_mut()
                .get_config()
                .set_invalidated(true);
        }

        self.invalidated = false;
    }

    fn needs_layout(&self) -> bool {
        self.invalidated
    }
}
