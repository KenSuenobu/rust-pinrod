// Pushrod Widget Library
// List Widget
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

use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

use crate::render::canvas_helper::CanvasHelper;
use crate::render::layout_cache::LayoutContainer;
use crate::render::{Points, Size, POINT_Y};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;

/// This is the callback type that is used when an `on_selected` callback is triggered from this
/// `Widget`.
pub type OnSelectedCallbackType =
    Option<Box<dyn FnMut(&mut ListWidget, &[WidgetContainer], &[LayoutContainer], i32)>>;

/// This is the storage object for the `ListWidget`.  It stores the config, properties, callback registry.
pub struct ListWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    list_items: Vec<String>,
    highlighted_item: i32,
    selected_item: i32,
    in_bounds: bool,
    on_selected: OnSelectedCallbackType,
}

/// This is the implementation of the `ListWidget`, a control that displays a list of items that can be
/// selected.
impl ListWidget {
    /// Creates a new `ListWidget` given the `x, y, w, h` coordinates.
    pub fn new(points: Points, size: Size) -> Self {
        Self {
            config: WidgetConfig::new(points, size),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            list_items: vec![],
            highlighted_item: -1,
            selected_item: -1,
            in_bounds: false,
            on_selected: None,
        }
    }

    /// Adds a text item to the `ListWidget`.
    pub fn add_item(&mut self, item: String) -> usize {
        let item_size = self.list_items.len() + 1;

        self.list_items.push(item);

        item_size
    }

    fn draw_text(
        &mut self,
        c: &mut Canvas<Window>,
        msg: String,
        x: u32,
        y: u32,
        back_color: Color,
        text_color: Color,
    ) {
        let text_max_width =
            self.get_size(CONFIG_SIZE)[0] - ((self.get_numeric(CONFIG_BORDER_WIDTH) * 2) as u32);

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let texture_creator = c.texture_creator();
        let mut font = ttf_context
            .load_font(
                Path::new(&String::from("assets/OpenSans-Regular.ttf")),
                16 as u16,
            )
            .unwrap();

        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        let surface = font
            .render(&msg)
            .blended_wrapped(text_color, text_max_width)
            .map_err(|e| e.to_string())
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let texture_y = self.get_config().to_y(y as i32);
        let texture_x = self.get_config().to_x(10);

        c.set_draw_color(back_color);
        c.fill_rect(Rect::new(
            self.get_config().to_x(x as i32),
            self.get_config().to_y(y as i32),
            self.get_size(CONFIG_SIZE)[0],
            30,
        ))
        .unwrap();

        c.copy(
            &texture,
            None,
            Rect::new(texture_x, texture_y, width, height),
        )
        .unwrap();
    }

    fn draw_list(&mut self, c: &mut Canvas<Window>) {
        let list_size = self.list_items.len();
        let list_height: u32 = 30;

        for i in 0..list_size {
            let mut text_color = Color::RGB(0, 0, 0);
            let mut color = if self.highlighted_item == i as i32 {
                self.get_color(CONFIG_COLOR_HOVER)
            } else {
                Color::RGB(255, 255, 255)
            };

            if self.selected_item == i as i32 {
                color = Color::RGB(0, 0, 0);
                text_color = Color::RGB(255, 255, 255);
            }

            self.draw_text(
                c,
                self.list_items[i].clone(),
                0,
                list_height * i as u32 as u32,
                color,
                text_color,
            );
        }
    }

    /// Assigns the callback closure that will be used when the `Widget` changes value, based on a selected
    /// item.
    pub fn on_selected<F>(&mut self, callback: F)
    where
        F: FnMut(&mut ListWidget, &[WidgetContainer], &[LayoutContainer], i32) + 'static,
    {
        self.on_selected = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_selected` callback.
    fn call_selected_callback(&mut self, widgets: &[WidgetContainer], layouts: &[LayoutContainer]) {
        if let Some(mut cb) = self.on_selected.take() {
            cb(self, widgets, layouts, self.selected_item);
            self.on_selected = Some(cb);
        }
    }
}

impl CanvasHelper for ListWidget {}

/// This is the `Widget` implementation of the `ListWidget`.
impl Widget for ListWidget {
    /// Draws the `ListWidget` contents.
    fn draw(&mut self, c: &mut Canvas<Window>) {
        let base_color = self.get_color(CONFIG_COLOR_BASE);

        c.set_draw_color(base_color);
        c.fill_rect(self.get_drawing_area()).unwrap();

        self.draw_list(c);

        let border_color = self.get_config().get_color(CONFIG_COLOR_BORDER);

        c.set_draw_color(border_color);
        self.draw_bounding_box(c);
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = true;
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = false;
        self.highlighted_item = -1;
        self.get_config().set_invalidated(true);
    }

    /// When a mouse is moved in the bounds of this `Widget`, this function is triggered.
    fn mouse_moved(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        points: Points,
    ) {
        if self.in_bounds {
            let position_y =
                points[POINT_Y] - self.get_config().get_point(CONFIG_ORIGIN)[POINT_Y] as i32;
            let previous_highlighted_item = self.highlighted_item;

            self.highlighted_item = position_y / 30;

            if self.highlighted_item >= self.list_items.len() as i32 {
                self.highlighted_item = -1;
            }

            if self.highlighted_item != previous_highlighted_item {
                self.get_config().set_invalidated(true);
            }
        }
    }

    /// Overrides the `button_clicked` callback to handle toggling.
    fn button_clicked(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        button: u8,
        _clicks: u8,
        state: bool,
    ) {
        if button == 1 && state {
            self.selected_item = self.highlighted_item;
            self.get_config().set_invalidated(true);

            self.call_selected_callback(_widgets, _layouts);
        }
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
