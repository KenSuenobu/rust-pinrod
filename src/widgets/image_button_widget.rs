// Pushrod Widget Library
// Image Button Widget
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

use crate::render::widget_config::CompassPosition::Center;
use crate::widgets::image_widget::ImageWidget;
use crate::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
use std::collections::HashMap;

/// This is the callback type that is used when an `on_click` callback is triggered from this
/// `Widget`.
pub type OnClickCallbackType = Option<Box<dyn FnMut(&mut ImageButtonWidget, &[WidgetContainer])>>;

/// This is the storage object for the `ImageButtonWidget`.  It stores the config, properties, callback registry.
pub struct ImageButtonWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    base_widget: BaseWidget,
    text_widget: TextWidget,
    image_widget: ImageWidget,
    active: bool,
    in_bounds: bool,
    on_click: OnClickCallbackType,
}

/// This is the implementation of the `ImageButtonWidget`, which displays an image next to some text.
impl ImageButtonWidget {
    /// Creates a new `ImageButtonWidget`, given the `x, y, w, h` coordinates, a block of `text`, the
    /// `font_size` to use, and the `image_name` to load and display.
    pub fn new(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        text: String,
        font_size: i32,
        image_name: String,
    ) -> Self {
        let mut base_widget = BaseWidget::new(x, y, w, h);
        let mut text_widget = TextWidget::new(
            String::from("assets/OpenSans-Regular.ttf"),
            sdl2::ttf::FontStyle::NORMAL,
            font_size,
            TextJustify::Left,
            text.clone(),
            x + h as i32 + 6,
            y + 2,
            w - h - 10,
            h - 4,
        );
        let mut image_widget = ImageWidget::new(image_name, x + 2, y + 2, h - 4, h - 4, false);

        base_widget.set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
        text_widget.set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));
        image_widget.set_compass(CONFIG_IMAGE_POSITION, Center);

        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            base_widget,
            text_widget,
            image_widget,
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
        F: FnMut(&mut ImageButtonWidget, &[WidgetContainer]) + 'static,
    {
        self.on_click = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_click` callback.
    fn call_click_callback(&mut self, widgets: &[WidgetContainer]) {
        if let Some(mut cb) = self.on_click.take() {
            cb(self, widgets);
            self.on_click = Some(cb);
        }
    }
}

/// This is the `Widget` implementation of the `ImageButtonWidget`.
impl Widget for ImageButtonWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c);
        self.text_widget.draw(c);
        self.image_widget.draw(c);
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer]) {
        if self.active {
            self.draw_hovered();
        }

        self.in_bounds = true;
        self.mouse_entered_callback(_widgets);
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer]) {
        if self.active {
            self.draw_unhovered();
        }

        self.in_bounds = false;
        self.mouse_exited_callback(_widgets);
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
                    self.call_click_callback(_widgets);
                }
            }
        }

        self.button_clicked_callback(_widgets, _button, _clicks, _state);
    }

    default_widget_properties!();
    default_widget_callbacks!();
}
