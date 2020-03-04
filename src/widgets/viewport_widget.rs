// Pushrod Widget Library
// Viewport Widget
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

use std::collections::HashMap;

use crate::render::widget_cache::WidgetContainer;
use crate::render::layout_cache::LayoutContainer;
use crate::render::{Size, Points, make_points_origin, SIZE_WIDTH, make_points, make_size, SIZE_HEIGHT};
use crate::render::widget_config::{WidgetConfig, CONFIG_SIZE, CONFIG_COLOR_BASE};
use crate::render::callbacks::CallbackRegistry;
use crate::render::texture_store::TextureStore;
use crate::widgets::slider_widget::SliderWidget;
use crate::widgets::slider_widget::SliderOrientation::{SliderVertical, SliderHorizontal};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use crate::render::texture_cache::TextureCache;
use crate::render::widget::Widget;
use sdl2::rect::Rect;
use std::any::Any;
use sdl2::pixels::Color;

/// This is the callback type that is used when an `on_view_changed` callback is triggered from this
/// `Widget`.  This simply indicates the area of the view that is visible when the visible viewport
/// bounds are modified.
pub type OnViewChangedCallbackType =
Option<Box<dyn FnMut(&mut ViewportWidget, &[WidgetContainer], &[LayoutContainer], Points, Size)>>;

pub struct ViewportWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    texture_store: TextureStore,
    view_point: Points,
    view_size: Size,
}

/// This is the implementation of the `ViewportWidget` that contains a viewable area within a scrollable
/// container.
impl ViewportWidget {
    /// Creates a new `CheckboxWidget` given the `x, y, w, h` coordinates
    pub fn new(points: Points, size: Size) -> Self {
        Self {
            config: WidgetConfig::new(points.clone(), size.clone()),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            texture_store: TextureStore::default(),
            view_point: make_points_origin(),
            view_size: size.clone(),
        }
    }

    /// Moves the position of the viewable area and redraws it after moving.
    pub fn move_viewport(&mut self, points: Points) {
        self.view_point = points;
        self.config.set_invalidated(true);
    }
}

impl Widget for ViewportWidget {
    fn draw(&mut self, c: &mut Canvas<Window>, t: &mut TextureCache) -> Option<&Texture> {
        if self.get_config().invalidated() {
            let bounds = self.get_config().get_size(CONFIG_SIZE);
            let base_color = self.get_color(CONFIG_COLOR_BASE);

            self.texture_store
                .create_or_resize_texture(c, bounds[0] as u32, bounds[1] as u32);

            // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
            // Invalidation is controlled by the top level widget (this box).
            let vertical_scroll_texture = self.vertical_scroll.draw(c, t).unwrap();
            let horizontal_scroll_texture = self.horizontal_scroll.draw(c, t).unwrap();

            c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
                texture.set_draw_color(base_color);
                texture.clear();

                texture.set_draw_color(Color::RGB(0, 0, 0));
                texture.draw_rect(Rect::new(0, 0, bounds[SIZE_WIDTH] - 28, bounds[SIZE_HEIGHT] - 28))
                    .unwrap();
            })
                .unwrap();
        }

        self.texture_store.get_optional_ref()
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();

}