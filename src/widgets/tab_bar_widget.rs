// Pushrod Widget Library
// Tab Bar Widget
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
use crate::render::{
    inverse_color, make_points, make_size, Points, Size, POINT_X, POINT_Y, SIZE_HEIGHT, SIZE_WIDTH,
};

use sdl2::render::{Canvas, Texture, TextureQuery};
use sdl2::video::Window;

use crate::render::layout_cache::LayoutContainer;
use crate::render::texture_cache::TextureCache;
use crate::render::texture_store::TextureStore;
use crate::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;

/// This is the callback type that is used when an `on_click` callback is triggered from this
/// `Widget`.  Returns a flag indicating the selected state - toggled on or off.
pub type OnTabSelectedCallbackType =
Option<Box<dyn FnMut(&mut TabBarWidget, &[WidgetContainer], &[LayoutContainer], u16)>>;

/// This is the storage object for the `TabBarWidget`.  It stores the config, properties, callback registry.
pub struct TabBarWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    texture_store: TextureStore,
    tab_items: Vec<String>,
    tab_widths: Vec<u32>,
    on_tab_selected: OnTabSelectedCallbackType,
    base_widget: BaseWidget,
    selected_item: i16,
    hovered_item: i16,
    in_bounds: bool,
    calculated: bool,
}

/// This is the implementation of the `TabBarWidget`, which displays an image next to some text.
impl TabBarWidget {
    /// Creates a new `TabBarWidget`, given the `x, y, w, h` coordinates.
    pub fn new(points: Points, size: Size, tab_items: Vec<String>) -> Self {
        let mut base_widget = BaseWidget::new(points.clone(), size.clone());

        base_widget.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
        base_widget.set_numeric(CONFIG_BORDER_WIDTH, 1);
        base_widget.set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));

        Self {
            config: WidgetConfig::new(points, size),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            texture_store: TextureStore::default(),
            on_tab_selected: None,
            tab_items: tab_items.clone(),
            tab_widths: vec!(0),
            base_widget,
            selected_item: -1,
            hovered_item: -1,
            in_bounds: false,
            calculated: false,
        }
    }

    /// Assigns the callback closure that will be used when a button click is triggered.
    pub fn on_tab_selected<F>(&mut self, callback: F)
        where
            F: FnMut(&mut TabBarWidget, &[WidgetContainer], &[LayoutContainer], u16) + 'static,
    {
        self.on_tab_selected = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_click` callback.
    fn call_tab_selected_callback(
        &mut self,
        widgets: &[WidgetContainer],
        layouts: &[LayoutContainer],
        tab: u16,
    ) {
        if let Some(mut cb) = self.on_tab_selected.take() {
            cb(self, widgets, layouts, tab);
            self.on_tab_selected = Some(cb);
        }
    }

    fn adjust_widgets(&mut self, c: &mut Canvas<Window>, t: &mut TextureCache) {
        let ttf_context = t.get_ttf_context();
        let texture_creator = c.texture_creator();
        let num_tabs = self.tab_items.len();
        let mut font = ttf_context
            .load_font(Path::new(&String::from("assets/OpenSans-Regular.ttf")), 10)
            .unwrap();
        let mut tab_widths = Vec::new();
        let bounds = self.get_config().get_size(CONFIG_SIZE);

        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        for i in 0..num_tabs {
            let surface = font
                .render(&self.tab_items[i])
                .blended_wrapped(Color::RGB(0, 0, 0), bounds[SIZE_WIDTH])
                .map_err(|e| e.to_string())
                .unwrap();
            let font_texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();

            let TextureQuery { width, .. } = font_texture.query();

            tab_widths.push((width + 20) as u32);

            eprintln!("Pushed width: {}", width + 20);
        }

        self.tab_widths = tab_widths.clone();
        self.calculated = true;
    }

    fn find_hovered_item(&self, x: i32, y: i32) -> i16 {
        let mut selected_item = -1;
        let mut start_x: i32 = 20;

        for i in 0..self.tab_widths.len() {
            if x >= start_x && x <= (start_x + self.tab_widths[i] as i32 + 30) {
                selected_item = i as i16;
                break;
            }

            start_x += self.tab_widths[i] as i32 + 31;
        }

        selected_item
    }
}

/// This is the `Widget` implementation of the `TabBarWidget`.
impl Widget for TabBarWidget {
    fn draw(&mut self, c: &mut Canvas<Window>, t: &mut TextureCache) -> Option<&Texture> {
        if !self.calculated {
            self.adjust_widgets(c, t);
        }

        if self.get_config().invalidated() {
            let bounds = self.get_config().get_size(CONFIG_SIZE);
            let base_color = self.get_color(CONFIG_COLOR_BASE);

            self.texture_store
                .create_or_resize_texture(c, bounds[0] as u32, bounds[1] as u32);

            let base_widget_texture = self.base_widget.draw(c, t).unwrap();
            let tab_widths = self.tab_widths.clone();
            let selected_tab = self.selected_item;
            let hovered_tab = self.hovered_item;

            c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
                texture.set_draw_color(base_color);
                texture.clear();

                let mut start_x: u32 = 20;
                let num_tabs = tab_widths.len();

                for i in 0..tab_widths.len() {
                    if selected_tab == i as i16 {
                        texture.set_draw_color(Color::RGB(128, 128, 128));
                    } else if hovered_tab == i as i16 {
                        texture.set_draw_color(Color::RGB(192, 192, 192));
                    } else {
                        texture.set_draw_color(Color::RGB(224, 224, 224));
                    }

                    texture.fill_rect(
                        Rect::new(start_x as i32,
                        0,
                        tab_widths[i] + 30,
                        bounds[SIZE_HEIGHT])
                    ).unwrap();

                    start_x += tab_widths[i] + 30 + 1;
                }

                texture.set_draw_color(Color::RGB(0, 0, 0));
                texture
                    .draw_line(
                        Point::new(0, bounds[SIZE_HEIGHT] as i32 - 1),
                        Point::new(bounds[SIZE_WIDTH] as i32, bounds[SIZE_HEIGHT] as i32 - 1),
                    )
                    .unwrap();
            })
                .unwrap();
        }

        self.texture_store.get_optional_ref()
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = true;
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = false;
        self.hovered_item = -1;
        self.set_invalidated(true);
    }

    fn mouse_moved(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        points: Points,
    ) {
        if self.calculated {
            let origin = self.get_config().get_point(CONFIG_ORIGIN);
            let true_x = points[POINT_X] - origin[POINT_X];
            let true_y = points[POINT_Y] - origin[POINT_Y];
            let previous_hovered_item = self.hovered_item;
            let hovered_item = self.find_hovered_item(true_x, true_y);

            eprintln!("Mouse moved: {} x {} hovered={}", true_x, true_y, hovered_item);

            self.hovered_item = hovered_item;

            if previous_hovered_item != hovered_item {
                self.set_invalidated(true);
            }
        }
    }

    fn button_clicked(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        button: u8,
        _clicks: u8,
        state: bool,
    ) {
        if button == 1 && self.in_bounds && self.calculated && state {
            if self.hovered_item != -1 {
                self.selected_item = self.hovered_item;
                self.set_invalidated(true);

                if self.selected_item > -1 {
                    self.call_tab_selected_callback(_widgets, _layouts, self.selected_item as u16);
                }
            }
        }
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
