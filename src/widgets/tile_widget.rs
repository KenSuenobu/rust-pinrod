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
    make_points, make_size, Points, Size, POINT_X, POINT_Y, SIZE_HEIGHT, SIZE_WIDTH,
};

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::render::layout_cache::LayoutContainer;
use crate::render::texture_cache::TextureCache;
use crate::render::texture_store::TextureStore;
use crate::render::widget_config::CompassPosition::Center;
use crate::widgets::image_widget::ImageWidget;
use crate::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
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
    //    image_filename: String,
    //    image_size: Size,
    //    tile_text: String,
    base_widget: BaseWidget,
    text_widget: TextWidget,
    image_widget: ImageWidget,
    selected: bool,
    hovered: bool,
}

/// This is the implementation of the `TileWidget`, which displays an image next to some text.
impl TileWidget {
    /// Creates a new `TileWidget`, given the `x, y, w, h` coordinates, a block of `text`, the
    /// `font_size` to use, and the `image_name` to load and display.
    pub fn new(
        points: Points,
        size: Size,
        image_filename: String,
        image_size: Size,
        tile_text: String,
    ) -> Self {
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
        let mut image_widget = ImageWidget::new(
            image_filename,
            make_points(
                points[POINT_X] + size[SIZE_WIDTH] as i32 / 2 - image_size[SIZE_WIDTH] as i32 / 2,
                points[POINT_Y] + image_size[SIZE_HEIGHT] as i32 / 2 + 1,
            ),
            make_size(image_size[SIZE_WIDTH], image_size[SIZE_HEIGHT]),
            false,
        );

        base_widget.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
        base_widget.set_numeric(CONFIG_BORDER_WIDTH, 1);
        base_widget.set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
        text_widget.set_color(CONFIG_COLOR_BASE, Color::RGBA(255, 255, 255, 255));
        text_widget.set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));
        image_widget.set_compass(CONFIG_IMAGE_POSITION, Center);

        Self {
            config: WidgetConfig::new(points, size),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            texture_store: TextureStore::new(),
            on_click: None,
            //            image_filename: image_filename.clone(),
            //            image_size,
            //            tile_text: tile_text.clone(),
            base_widget,
            text_widget,
            image_widget,
            selected: false,
            hovered: false,
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
}

/// This is the `Widget` implementation of the `TileWidget`.
impl Widget for TileWidget {
    fn draw(&mut self, c: &mut Canvas<Window>, _t: &mut TextureCache) -> Option<&Texture> {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        if self.selected {
            let selected_color = self.get_color(CONFIG_COLOR_SELECTED);
            self.base_widget
                .set_color(CONFIG_COLOR_BASE, selected_color);
        } else if self.hovered {
            let hover_color = self.get_color(CONFIG_COLOR_HOVER);
            self.base_widget.set_color(CONFIG_COLOR_BASE, hover_color);
        } else {
            self.base_widget
                .set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
        }

        self.base_widget.draw(c, _t);
        self.image_widget.draw(c, _t);
        self.text_widget.draw(c, _t);

        None
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
            self.selected = !self.selected;
            self.call_click_callback(_widgets, _layouts, self.selected);
        }
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
