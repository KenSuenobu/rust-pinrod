// Pushrod Widget Library
// Push Button Widget
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
use crate::render::widget_config::{
    WidgetConfig, CONFIG_BORDER_WIDTH, CONFIG_COLOR_BASE, CONFIG_COLOR_BORDER, CONFIG_COLOR_TEXT,
};
use crate::render::Points;

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
use std::any::Any;
use std::collections::HashMap;
use crate::render::layout_cache::LayoutContainer;

/// This is the callback type that is used when an `on_click` callback is triggered from this
/// `Widget`.
pub type OnClickCallbackType = Option<Box<dyn FnMut(&mut PushButtonWidget, &[WidgetContainer], &[LayoutContainer])>>;

/// This is the storage object for the `PushButtonWidget`.  It stores the config, properties, callback registry.
pub struct PushButtonWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    base_widget: BaseWidget,
    text_widget: TextWidget,
    active: bool,
    in_bounds: bool,
    on_click: OnClickCallbackType,
}

/// This is the `PushButtonWidget` implementation, which displays a block of text inside a clickable
/// box.  Clicking the box will cause an `on_click` callback to be triggered, which will call a block
/// of text, if the callback has been configured.
impl PushButtonWidget {
    /// Creates a new `PushButtonWidget`, given `x, y, w, h` coordinates, some `text` to display,
    /// and the `font_size` to use.
    pub fn new(x: i32, y: i32, w: u32, h: u32, text: String, font_size: i32) -> Self {
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

        base_widget.set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
        base_widget.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
        base_widget.set_numeric(CONFIG_BORDER_WIDTH, 2);

        text_widget.set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            base_widget,
            text_widget,
            active: false,
            in_bounds: false,
            on_click: None,
        }
    }

    fn draw_hovered(&mut self) {
        self.base_widget
            .set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
        self.text_widget
            .set_color(CONFIG_COLOR_TEXT, Color::RGB(255, 255, 255));
        self.text_widget
            .set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
        self.get_config().set_invalidate(true);
    }

    fn draw_unhovered(&mut self) {
        self.base_widget
            .set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
        self.text_widget
            .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));
        self.text_widget
            .set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
        self.get_config().set_invalidate(true);
    }

    /// Assigns the callback closure that will be used when a button click is triggered.
    pub fn on_click<F>(&mut self, callback: F)
    where
        F: FnMut(&mut PushButtonWidget, &[WidgetContainer], &[LayoutContainer]) + 'static,
    {
        self.on_click = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_click` callback.
    fn call_click_callback(&mut self, widgets: &[WidgetContainer], layouts: &[LayoutContainer]) {
        if let Some(mut cb) = self.on_click.take() {
            cb(self, widgets, layouts);
            self.on_click = Some(cb);
        }
    }
}

/// This is the `Widget` implementation of the `PushButtonWidget`.
impl Widget for PushButtonWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c);
        self.text_widget.draw(c);
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        if self.active {
            self.draw_hovered();
        }

        self.in_bounds = true;
        self.mouse_entered_callback(_widgets, _layouts);
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        if self.active {
            self.draw_unhovered();
        }

        self.in_bounds = false;
        self.mouse_exited_callback(_widgets, _layouts);
    }

    /// When a mouse button is clicked within (or outside of) the bounds of the `Widget`, this
    /// function is called.  If a mouse button is clicked, and the mouse leaves the bounds of the
    /// `Widget`, the mouse release event will still be triggered for the last `Widget` which
    /// received the mouse down state.  This prevents `Widget`s from becoming confused.  This
    /// behavior is tracked by the main loop, not by the `Widget` code.  Therefore, when a mouse
    /// button is released outside of the bounds of _this_ `Widget`, you must adjust your state
    /// accordingly, if you pay attention to the `button_clicked` function.  This function
    /// implementation is **optional**.
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
                self.draw_hovered();
                self.active = true;
            } else {
                let had_bounds = self.active;

                self.draw_unhovered();
                self.active = false;

                if self.in_bounds && had_bounds {
                    // Callback here
                    eprintln!("Call callback here: clicks={}", _clicks);
                    self.call_click_callback(_widgets, _layouts);
                }
            }
        }

        self.button_clicked_callback(_widgets, _layouts, _button, _clicks, _state);
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
