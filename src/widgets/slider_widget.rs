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
use crate::render::Points;

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::render::layout_cache::LayoutContainer;
use crate::render::widget_config::CompassPosition::Center;
use crate::widgets::image_widget::ImageWidget;
use crate::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
use std::any::Any;
use std::collections::HashMap;

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
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = true;
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = false;
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
//                    self.selected = !self.selected;
//                    self.set_toggle(CONFIG_SELECTED_STATE, self.selected);
//                    self.call_toggle_callback(_widgets, _layouts);
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
