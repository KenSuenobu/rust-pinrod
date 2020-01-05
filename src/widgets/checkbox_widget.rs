// Pushrod Widget Library
// Checkbox Widget
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

use crate::render::canvas_helper::CanvasHelper;
use crate::render::layout_cache::LayoutContainer;
use crate::render::widget_config::CompassPosition::Center;
use crate::widgets::image_widget::ImageWidget;
use crate::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
use std::any::Any;
use std::collections::HashMap;

/// This is the callback type that is used when an `on_toggle` callback is triggered from this
/// `Widget`.
pub type OnToggleCallbackType =
    Option<Box<dyn FnMut(&mut CheckboxWidget, &[WidgetContainer], &[LayoutContainer], bool)>>;

/// This is the storage object for the `CheckboxWidget`.  It stores the config, properties, callback registry.
pub struct CheckboxWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    text_widget: TextWidget,
    unchecked_widget: ImageWidget,
    checked_widget: ImageWidget,
    active: bool,
    selected: bool,
    in_bounds: bool,
    originated: bool,
    on_toggle: OnToggleCallbackType,
}

/// This is the implementation of the `CheckboxWidget` that draws a button on the screen that can be
/// toggled on or off.
impl CheckboxWidget {
    /// Creates a new `CheckboxWidget` given the `x, y, w, h` coordinates, the `text` to display
    /// inside the button, `font_size` of the font to display, and the initial `selected` state: `true`
    /// being checked, `false` otherwise.
    pub fn new(points: Points, size: Size, text: String, font_size: i32, selected: bool) -> Self {
        let mut text_widget = TextWidget::new(
            String::from("assets/OpenSans-Regular.ttf"),
            sdl2::ttf::FontStyle::NORMAL,
            font_size,
            TextJustify::Left,
            text,
            make_points(
                points[POINT_X] + size[SIZE_HEIGHT] as i32 + 6,
                points[POINT_Y] + 2,
            ),
            make_size(
                size[SIZE_WIDTH] - size[SIZE_HEIGHT] - 10,
                size[SIZE_HEIGHT] - 4,
            ),
        );

        let mut config = WidgetConfig::new(points.clone(), size.clone());
        let mut unchecked_widget = ImageWidget::new(
            String::from("assets/checkbox_unselected.png"),
            make_points(points[POINT_X] + 2, points[POINT_Y] + 2),
            make_size(size[SIZE_HEIGHT] - 4, size[SIZE_HEIGHT] - 4),
            true,
        );
        let mut checked_widget = ImageWidget::new(
            String::from("assets/checkbox_selected.png"),
            make_points(points[POINT_X] + 2, points[POINT_Y] + 2),
            make_size(size[SIZE_HEIGHT] - 4, size[SIZE_HEIGHT] - 4),
            true,
        );

        text_widget.set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));
        unchecked_widget.set_compass(CONFIG_IMAGE_POSITION, Center);
        checked_widget.set_compass(CONFIG_IMAGE_POSITION, Center);

        config.set_toggle(CONFIG_SELECTED_STATE, selected);

        Self {
            config,
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            text_widget,
            unchecked_widget,
            checked_widget,
            active: false,
            selected,
            in_bounds: false,
            originated: false,
            on_toggle: None,
        }
    }

    /// Assigns the callback closure that will be used when the `Widget` toggles state.
    pub fn on_toggle<F>(&mut self, callback: F)
    where
        F: FnMut(&mut CheckboxWidget, &[WidgetContainer], &[LayoutContainer], bool) + 'static,
    {
        self.on_toggle = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_toggle` callback.
    fn call_toggle_callback(&mut self, widgets: &[WidgetContainer], layouts: &[LayoutContainer]) {
        if let Some(mut cb) = self.on_toggle.take() {
            cb(self, widgets, layouts, self.selected);
            self.on_toggle = Some(cb);
        }
    }
}

impl CanvasHelper for CheckboxWidget {}

/// This is the `Widget` implementation of the `CheckboxWidget`.
impl Widget for CheckboxWidget {
    /// Draws the `CheckboxWidget` contents.
    fn draw(&mut self, c: &mut Canvas<Window>) -> Option<&Texture> {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        if self.active {
            if self.in_bounds {
                if self.selected {
                    self.unchecked_widget.draw(c);
                } else {
                    self.checked_widget.draw(c);
                }
            } else if !self.in_bounds {
                if self.selected {
                    self.checked_widget.draw(c);
                } else {
                    self.unchecked_widget.draw(c);
                }
            }
        } else if !self.active {
            if self.selected {
                self.checked_widget.draw(c);
            } else {
                self.unchecked_widget.draw(c);
            }
        }

        self.text_widget.draw(c);
        self.draw_bounding_box(c);

        None
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = true;
        self.mouse_entered_callback(_widgets, _layouts);
        self.get_config().set_invalidated(true);
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = false;
        self.mouse_exited_callback(_widgets, _layouts);
        self.get_config().set_invalidated(true);
    }

    /// Overrides the `button_clicked` callback to handle toggling.
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
                self.active = true;
                self.originated = true;
            } else {
                self.active = false;

                if self.in_bounds && self.originated {
                    self.selected = !self.selected;
                    self.set_toggle(CONFIG_SELECTED_STATE, self.selected);
                    self.call_toggle_callback(_widgets, _layouts);
                }

                self.originated = false;
            }

            self.get_config().set_invalidated(true);
        }

        self.button_clicked_callback(_widgets, _layouts, _button, _clicks, _state);
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
