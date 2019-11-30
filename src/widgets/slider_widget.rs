// Pushrod Widget Library
// Slider Widget
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
use crate::render::{Points, SIZE_HEIGHT, SIZE_WIDTH, POINT_X};

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::render::layout_cache::LayoutContainer;
use crate::render::widget_config::CompassPosition::Center;
use crate::widgets::image_widget::ImageWidget;
use crate::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
use std::any::Any;
use std::collections::HashMap;
use sdl2::rect::{Point, Rect};

/// This is the callback type that is used when an `on_value_changed` callback is triggered from this
/// `Widget`.
pub type OnValueChangedCallbackType = Option<Box<dyn FnMut(&mut SliderWidget, &[WidgetContainer], &[LayoutContainer], u32)>>;

pub enum SliderOrientation {
    SliderHorizontal,
    SliderVertical,
}

/// This is the storage object for the `SliderWidget`.  It stores the config, properties, callback registry.
pub struct SliderWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    min: u32,
    max: u32,
    current: u32,
    orientation: SliderOrientation,
    in_bounds: bool,
    active: bool,
    originated: bool,
    on_value_changed: OnValueChangedCallbackType,
}

/// This is the implementation of the `SliderWidget` that draws a button on the screen that can be
/// toggled on or off.
impl SliderWidget {
    /// Creates a new `CheckboxWidget` given the `x, y, w, h` coordinates, the `text` to display
    /// inside the button, `font_size` of the font to display, and the initial `selected` state: `true`
    /// being checked, `false` otherwise.
    pub fn new(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        min: u32,
        max: u32,
        current: u32,
        orientation: SliderOrientation,
    ) -> Self {
        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            min,
            max,
            current,
            orientation,
            in_bounds: false,
            active: false,
            originated: false,
            on_value_changed: None,
        }
    }

    /// Assigns the callback closure that will be used when the `Widget` changes value.
    pub fn on_value_changed<F>(&mut self, callback: F)
        where
            F: FnMut(&mut SliderWidget, &[WidgetContainer], &[LayoutContainer], u32) + 'static,
    {
        self.on_value_changed = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_value_changed` callback.
    fn call_value_changed_callback(&mut self, widgets: &[WidgetContainer], layouts: &[LayoutContainer]) {
        if let Some(mut cb) = self.on_value_changed.take() {
            cb(self, widgets, layouts, self.current);
            self.on_value_changed = Some(cb);
        }
    }
}

/// This is the `Widget` implementation of the `SliderWidget`.
impl Widget for SliderWidget {
    /// Draws the `CheckboxWidget` contents.
    fn draw(&mut self, c: &mut Canvas<Window>) {
        let base_color = self.get_color(CONFIG_COLOR_BASE);

        c.set_draw_color(base_color);
        c.fill_rect(self.get_drawing_area()).unwrap();

        // Draw base - three lines in the center
        let half_height = (self.get_config().get_size(CONFIG_SIZE)[SIZE_HEIGHT] / 2) as i32;
        let width = (self.get_config().get_size(CONFIG_SIZE)[SIZE_WIDTH]) as i32;

        c.set_draw_color(Color::RGB(192, 192, 192));
        c.draw_line(Point::new(self.get_config().to_x(0), self.get_config().to_y(half_height)),
                    Point::new(self.get_config().to_x(width), self.get_config().to_y(half_height))).unwrap();
        c.draw_line(Point::new(self.get_config().to_x(0), self.get_config().to_y(half_height - 1)),
                    Point::new(self.get_config().to_x(width), self.get_config().to_y(half_height - 1))).unwrap();
        c.draw_line(Point::new(self.get_config().to_x(0), self.get_config().to_y(half_height + 1)),
                    Point::new(self.get_config().to_x(width), self.get_config().to_y(half_height + 1))).unwrap();

        // Draw slider at current value
        let slider_center = (width as u32 / (self.max - self.min)) * self.current;
        let slider_start = if slider_center <= 10 { 0 } else { slider_center - 10 };
        let slider_end = slider_center + 10;

        c.set_draw_color(base_color);
        c.fill_rect(Rect::new(self.get_config().to_x(slider_start as i32),
        self.get_config().to_y(0), 20, self.get_config().get_size(CONFIG_SIZE)[SIZE_HEIGHT]));

        c.set_draw_color(Color::RGB(0, 0, 0));
        c.draw_rect(Rect::new(self.get_config().to_x(slider_start as i32),
                              self.get_config().to_y(0), 20, self.get_config().get_size(CONFIG_SIZE)[SIZE_HEIGHT]));

        eprintln!("Center: {}", slider_center);
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = true;
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = false;
    }

    /// When a mouse is moved in the bounds of this `Widget`, this function is triggered.
    fn mouse_moved(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        points: Points,
    ) {
        if self.in_bounds && self.active && self.originated {
            let width = (self.get_config().get_size(CONFIG_SIZE)[SIZE_WIDTH]) as i32;
            let slider_width = (width as u32 / (self.max - self.min));
            self.current = (points[POINT_X] - self.get_config().get_point(CONFIG_ORIGIN)[POINT_X]) as u32 / slider_width;

            self.get_config().set_invalidated(true);
        }
    }

    /// Handles the scrolling functionality.
    fn mouse_scrolled(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _points: Points,
    ) {
        let mut current_i32 = self.current as i32;

        current_i32 += _points[POINT_X];

        if current_i32 >= self.max as i32 {
            current_i32 = self.max as i32;
        } else if current_i32 <= self.min as i32 {
            current_i32 = self.min as i32;
        }

        self.current = current_i32 as u32;

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
