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

use sdl2::render::{Canvas, Texture, TextureQuery};
use sdl2::video::Window;

use crate::render::canvas_helper::CanvasHelper;
use crate::render::layout_cache::LayoutContainer;
use crate::render::texture_cache::TextureCache;
use crate::render::texture_store::TextureStore;
use crate::render::{Points, Size, POINT_Y, SIZE_WIDTH};
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
    texture_store: TextureStore,
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
            texture_store: TextureStore::default(),
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

    /// Assigns the callback closure that will be used when the `Widget` changes value, based on a selected
    /// item.
    pub fn on_selected<F>(&mut self, callback: F)
    where
        F: FnMut(&mut ListWidget, &[WidgetContainer], &[LayoutContainer], i32) + 'static,
    {
        self.on_selected = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_selected` callback.  The selected item ID indicates the value
    /// in the `ListWidget` that has been selected.  If the value is set to `-1`, it means the list items
    /// have been de-selected.
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
    fn draw(&mut self, c: &mut Canvas<Window>, t: &mut TextureCache) -> Option<&Texture> {
        if self.get_config().invalidated() {
            let bounds = self.get_config().get_size(CONFIG_SIZE);

            self.texture_store
                .create_or_resize_texture(c, bounds[0] as u32, bounds[1] as u32);

            let base_color = self.get_color(CONFIG_COLOR_BASE);
            let hover_color = self.get_color(CONFIG_COLOR_HOVER);
            let border_color = self.get_config().get_color(CONFIG_COLOR_BORDER);
            let list_size = self.list_items.len();
            let highlighted_item = self.highlighted_item;
            let selected_item = self.selected_item;
            let list_items = self.list_items.clone();

            let ttf_context = t.get_ttf_context();
            let texture_creator = c.texture_creator();
            let mut font = ttf_context
                .load_font(Path::new(&String::from("assets/OpenSans-Regular.ttf")), 16)
                .unwrap();

            font.set_style(sdl2::ttf::FontStyle::NORMAL);

            c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
                texture.set_draw_color(base_color);
                texture.clear();

                let list_height: u32 = 30;

                for i in 0..list_size {
                    let mut text_color = Color::RGB(0, 0, 0);
                    let mut color = if highlighted_item == i as i32 {
                        hover_color
                    } else {
                        Color::RGB(255, 255, 255)
                    };

                    if selected_item == i as i32 {
                        color = Color::RGB(0, 0, 0);
                        text_color = Color::RGB(255, 255, 255);
                    }

                    texture.set_draw_color(color);
                    texture
                        .fill_rect(Rect::new(
                            0,
                            (list_height * i as u32) as i32,
                            bounds[SIZE_WIDTH],
                            30,
                        ))
                        .unwrap();

                    let surface = font
                        .render(&list_items[i].clone())
                        .blended_wrapped(text_color, bounds[SIZE_WIDTH])
                        .map_err(|e| e.to_string())
                        .unwrap();
                    let font_texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .map_err(|e| e.to_string())
                        .unwrap();

                    let TextureQuery { width, height, .. } = font_texture.query();
                    let texture_y = (list_height * i as u32) as i32 + 3;
                    let texture_x = 10;

                    texture
                        .copy(
                            &font_texture,
                            None,
                            Rect::new(texture_x, texture_y, width, height),
                        )
                        .unwrap();
                }

                texture.set_draw_color(border_color);
                texture
                    .draw_rect(Rect::new(0, 0, bounds[0], bounds[1]))
                    .unwrap();
            })
            .unwrap();
        }

        self.texture_store.get_optional_ref()
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
