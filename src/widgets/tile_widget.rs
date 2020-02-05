// Pushrod Widget Library
// Tile Widget
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

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::render::layout_cache::LayoutContainer;
use crate::render::texture_cache::TextureCache;
use crate::render::texture_store::TextureStore;
use crate::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::any::Any;
use std::collections::HashMap;

/// This is the callback type that is used when an `on_click` callback is triggered from this
/// `Widget`.  Returns a flag indicating the selected state - toggled on or off.
pub type OnClickedCallbackType =
    Option<Box<dyn FnMut(&mut TileWidget, &[WidgetContainer], &[LayoutContainer], bool)>>;

/// This is the storage object for the `TileWidget`.  It stores the config, properties, callback registry.
pub struct TileWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    texture_store: TextureStore,
    on_click: OnClickedCallbackType,
    base_widget: BaseWidget,
    text_widget: TextWidget,
    image_name: String,
    selected: bool,
    hovered: bool,
    originated: bool,
}

/// This is the implementation of the `TileWidget`, which displays an image next to some text.
impl TileWidget {
    /// Creates a new `TileWidget`, given the `x, y, w, h` coordinates, the `image_name` to load and
    /// display, and the text to show in the tile.
    pub fn new(points: Points, size: Size, image_name: String, tile_text: String) -> Self {
        let mut base_widget = BaseWidget::new(points.clone(), size.clone());
        let mut text_widget = TextWidget::new(
            String::from("assets/OpenSans-Regular.ttf"),
            sdl2::ttf::FontStyle::NORMAL,
            14,
            TextJustify::Center,
            tile_text,
            make_points(
                points[POINT_X] + 1,
                points[POINT_Y] + size[SIZE_HEIGHT] as i32 - 19,
            ),
            make_size(size[SIZE_WIDTH] - 2, 18),
        );

        base_widget.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
        base_widget.set_numeric(CONFIG_BORDER_WIDTH, 1);
        base_widget.set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
        text_widget.set_color(CONFIG_COLOR_BASE, Color::RGBA(255, 255, 255, 255));
        text_widget.set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

        Self {
            config: WidgetConfig::new(points, size),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            texture_store: TextureStore::default(),
            on_click: None,
            base_widget,
            text_widget,
            image_name,
            selected: false,
            hovered: false,
            originated: false,
        }
    }

    /// Assigns the callback closure that will be used when a button click is triggered.
    pub fn on_click<F>(&mut self, callback: F)
    where
        F: FnMut(&mut TileWidget, &[WidgetContainer], &[LayoutContainer], bool) + 'static,
    {
        self.on_click = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_click` callback.
    fn call_click_callback(
        &mut self,
        widgets: &[WidgetContainer],
        layouts: &[LayoutContainer],
        state: bool,
    ) {
        if let Some(mut cb) = self.on_click.take() {
            cb(self, widgets, layouts, state);
            self.on_click = Some(cb);
        }
    }

    fn adjust_widgets(&mut self) {
        if self.selected {
            let selected_color = self.get_color(CONFIG_COLOR_SELECTED);
            self.base_widget
                .set_color(CONFIG_COLOR_BASE, selected_color);
            self.text_widget
                .set_color(CONFIG_COLOR_BASE, selected_color);
            self.text_widget
                .set_color(CONFIG_COLOR_TEXT, inverse_color(selected_color));
        } else if self.hovered {
            let hover_color = self.get_color(CONFIG_COLOR_HOVER);
            self.base_widget.set_color(CONFIG_COLOR_BASE, hover_color);
            self.text_widget.set_color(CONFIG_COLOR_BASE, hover_color);
            self.text_widget
                .set_color(CONFIG_COLOR_TEXT, inverse_color(hover_color));
        } else {
            self.base_widget
                .set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
            self.text_widget
                .set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
            self.text_widget
                .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));
        }
    }
}

/// This is the `Widget` implementation of the `TileWidget`.
impl Widget for TileWidget {
    fn draw(&mut self, c: &mut Canvas<Window>, t: &mut TextureCache) -> Option<&Texture> {
        if self.get_config().invalidated() {
            let bounds = self.get_config().get_size(CONFIG_SIZE);
            let base_color = self.get_color(CONFIG_COLOR_BASE);

            self.texture_store
                .create_or_resize_texture(c, bounds[0] as u32, bounds[1] as u32);

            self.adjust_widgets();

            let base_widget_texture = self.base_widget.draw(c, t).unwrap();
            let text_widget_texture = self.text_widget.draw(c, t).unwrap();
            let image_texture = t.get_image(c, self.image_name.clone());

            c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
                texture.set_draw_color(base_color);
                texture.clear();

                let size_center = bounds[SIZE_WIDTH] / 2;

                texture
                    .copy(
                        base_widget_texture,
                        None,
                        Rect::new(0, 0, bounds[0], bounds[1]),
                    )
                    .unwrap();

                texture
                    .copy(
                        text_widget_texture,
                        None,
                        Rect::new(
                            1,
                            bounds[SIZE_HEIGHT] as i32 - 20,
                            bounds[SIZE_WIDTH] - 2,
                            18,
                        ),
                    )
                    .unwrap();

                texture
                    .copy(
                        image_texture,
                        None,
                        Rect::new(
                            (size_center - 16) as i32,
                            (bounds[SIZE_HEIGHT] / 2 - 32) as i32,
                            32,
                            32,
                        ),
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
        self.hovered = true;
        self.get_config().set_invalidated(true);
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.hovered = false;
        self.get_config().set_invalidated(true);
    }

    fn button_clicked(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _button: u8,
        _clicks: u8,
        _state: bool,
    ) {
        if _button == 1 {
            if _state {
                self.originated = true;
            } else {
                if self.originated && self.hovered {
                    self.selected = !self.selected;
                    self.call_click_callback(_widgets, _layouts, self.selected);
                }

                self.originated = false;
            }
        }
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
