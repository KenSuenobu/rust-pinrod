// Pushrod Widget Library
// Toggle Button Widget
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

use crate::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::any::Any;

/// This is the callback type that is used when an `on_toggle` callback is triggered from this
/// `Widget`.
pub type OnToggleCallbackType =
    Option<Box<dyn FnMut(&mut ToggleButtonWidget, &[WidgetContainer], bool)>>;

/// This is the storage object for the `ToggleButtonWidget`.  It stores the config, properties, callback registry.
pub struct ToggleButtonWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    base_widget: BaseWidget,
    text_widget: TextWidget,
    active: bool,
    selected: bool,
    in_bounds: bool,
    on_toggle: OnToggleCallbackType,
}

/// This is the implementation of the `ToggleButtonWidget` that draws a button on the screen that can be
/// toggled on or off.
impl ToggleButtonWidget {
    /// Creates a new `ToggleButtonWidget` given the `x, y, w, h` coordinates, the `text` to display
    /// inside the button, `font_size` of the font to display, and the initial `selected` state: `true`
    /// being selected, `false` otherwise.
    pub fn new(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        text: String,
        font_size: i32,
        selected: bool,
    ) -> Self {
        let mut base_widget = BaseWidget::new(x, y, w, h);
        let mut text_widget = TextWidget::new(
            String::from("assets/OpenSans-Regular.ttf"),
            sdl2::ttf::FontStyle::NORMAL,
            font_size,
            TextJustify::Center,
            text.clone(),
            x + 2,
            y + 2,
            w - 4,
            h - 4,
        );

        let base_color = if selected {
            Color::RGB(0, 0, 0)
        } else {
            Color::RGB(255, 255, 255)
        };
        let text_color = if selected {
            Color::RGB(255, 255, 255)
        } else {
            Color::RGB(0, 0, 0)
        };

        let mut config = WidgetConfig::new(x, y, w, h);

        base_widget.set_color(CONFIG_COLOR_BASE, base_color);
        base_widget.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
        base_widget.set_numeric(CONFIG_BORDER_WIDTH, 2);

        text_widget.set_color(CONFIG_COLOR_BASE, base_color);
        text_widget.set_color(CONFIG_COLOR_TEXT, text_color);

        config.set_toggle(CONFIG_SELECTED_STATE, selected);

        Self {
            config,
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            base_widget,
            text_widget,
            active: false,
            selected,
            in_bounds: false,
            on_toggle: None,
        }
    }

    /// Draws the state when the mouse is over the top of the `Widget`.
    fn draw_hovered(&mut self) {
        let base_color = if self.selected {
            Color::RGB(255, 255, 255)
        } else {
            Color::RGB(0, 0, 0)
        };
        let text_color = if self.selected {
            Color::RGB(0, 0, 0)
        } else {
            Color::RGB(255, 255, 255)
        };

        self.base_widget.set_color(CONFIG_COLOR_BASE, base_color);
        self.text_widget.set_color(CONFIG_COLOR_TEXT, text_color);
        self.text_widget.set_color(CONFIG_COLOR_BASE, base_color);
        self.get_config().set_invalidate(true);
    }

    /// Draws the state when the mouse leaves the scope of the `Widget`.
    fn draw_unhovered(&mut self) {
        let base_color = if self.selected {
            Color::RGB(0, 0, 0)
        } else {
            Color::RGB(255, 255, 255)
        };
        let text_color = if self.selected {
            Color::RGB(255, 255, 255)
        } else {
            Color::RGB(0, 0, 0)
        };

        self.base_widget.set_color(CONFIG_COLOR_BASE, base_color);
        self.text_widget.set_color(CONFIG_COLOR_TEXT, text_color);
        self.text_widget.set_color(CONFIG_COLOR_BASE, base_color);
        self.get_config().set_invalidate(true);
    }

    /// Assigns the callback closure that will be used when the `Widget` toggles state.
    pub fn on_toggle<F>(&mut self, callback: F)
    where
        F: FnMut(&mut ToggleButtonWidget, &[WidgetContainer], bool) + 'static,
    {
        self.on_toggle = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_toggle` callback.
    fn call_toggle_callback(&mut self, widgets: &[WidgetContainer]) {
        if let Some(mut cb) = self.on_toggle.take() {
            cb(self, widgets, self.selected);
            self.on_toggle = Some(cb);
        }
    }
}

/// This is the `Widget` implementation of the `ToggleButtonWidget`.
impl Widget for ToggleButtonWidget {
    /// Draws the `ToggleButtonWidget` contents.
    fn draw(&mut self, c: &mut Canvas<Window>) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c);
        self.text_widget.draw(c);
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer]) {
        if self.active {
            self.draw_hovered();
        }

        self.in_bounds = true;
        self.mouse_entered_callback(_widgets);
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer]) {
        if self.active {
            self.draw_unhovered();
        }

        self.in_bounds = false;
        self.mouse_exited_callback(_widgets);
    }

    /// Overrides the `button_clicked` callback to handle toggling.
    fn button_clicked(
        &mut self,
        _widgets: &[WidgetContainer],
        _button: u8,
        _clicks: u8,
        _state: bool,
    ) {
        if _button == 1 {
            if _state {
                self.draw_hovered();
                self.active = true;
            } else {
                self.active = false;

                if self.in_bounds {
                    self.selected = !self.selected;
                    self.set_toggle(CONFIG_SELECTED_STATE, self.selected);
                    self.call_toggle_callback(_widgets);
                }
            }
        }

        self.button_clicked_callback(_widgets, _button, _clicks, _state);
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
