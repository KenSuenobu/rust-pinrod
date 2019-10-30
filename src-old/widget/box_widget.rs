// Box Widget
// Extensible widget for the widget library - handles drawing a box with a border and a fill color
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

use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::core::callbacks::*;
use crate::core::point::Size;
use crate::core::widget_store::WidgetContainer;
use crate::widget::config::*;
use crate::widget::widget::*;

/// A `BoxWidget` is a `CanvasWidget` with a bounding box.  Takes two additional options:
/// * `CONFIG_BORDER_WIDTH` specifies the width of the border to be drawn in pixels.
/// * `CONFIG_BORDER_COLOR` specifies the color of the border to be drawn.
pub struct BoxWidget {
    config: Configurable,
    event_list: Vec<CallbackEvent>,
    widget_id: i32,
    callbacks: DefaultWidgetCallbacks,
}

impl BoxWidget {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            event_list: vec![],
            widget_id: 0,
            callbacks: DefaultWidgetCallbacks::new(),
        }
    }

    fn draw_box(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);
        let border: f64 = self.config().get_numeric(CONFIG_BORDER_WIDTH) as f64;
        let color: types::Color = self.config().get_color(CONFIG_BORDER_COLOR);
        let fill_color: types::Color = self.config().get_color(CONFIG_MAIN_COLOR);

        g.rectangle(
            &Rectangle::new(fill_color),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform.clone(),
        );

        Rectangle::new_border(color, border).draw(
            [
                0.0 as f64 + border as f64,
                0.0 as f64 + border as f64,
                size.w as f64 - (border as f64 * 2.0),
                size.h as f64 - (border as f64 * 2.0),
            ],
            clip,
            c.transform.clone(),
            g,
        );
    }

    inject_event_handler!();
}

impl Drawable for BoxWidget {
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        // Paint the box.
        self.draw_box(c, g, &clip);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}

impl InjectableSystemEvents for BoxWidget {
    fn inject_system_event(&mut self) -> Option<CallbackEvent> {
        self.event_list.pop().clone()
    }
}

impl InjectableCustomEvents for BoxWidget {}

impl Widget for BoxWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value);
        self.invalidate();
    }

    fn set_size(&mut self, config: u8, w: i32, h: i32) {
        self.set_config(config, Config::Size(Size { w, h }));

        if self.widget_id != 0 {
            self.event_list.push(CallbackEvent::WidgetResized {
                widget_id: self.widget_id,
                size: Size { w, h },
            });
        }

        self.invalidate();
    }

    fn set_point(&mut self, config: u8, x: i32, y: i32) {
        self.set_config(config, Config::Point([x, y]));

        if self.widget_id != 0 {
            self.event_list.push(CallbackEvent::WidgetMoved {
                widget_id: self.widget_id,
                point: [x, y],
            });
        }

        self.invalidate();
    }

    fn handle_event(
        &mut self,
        injected: bool,
        _event: CallbackEvent,
        _widget_store: Option<&Vec<WidgetContainer>>,
    ) -> Option<CallbackEvent> {
        if !injected {
            self.handle_event_callbacks(_event, _widget_store);
        }

        None
    }

    fn handles_events(&mut self) -> bool {
        true
    }

    fn set_widget_id(&mut self, widget_id: i32) {
        self.widget_id = widget_id;
    }

    fn get_widget_id(&mut self) -> i32 {
        self.widget_id
    }

    fn injects_system_events(&mut self) -> bool {
        true
    }

    fn get_injectable_custom_events(&mut self) -> &mut dyn InjectableCustomEvents {
        self
    }

    fn get_injectable_system_events(&mut self) -> &mut dyn InjectableSystemEvents {
        self
    }

    fn get_drawable(&mut self) -> &mut dyn Drawable {
        self
    }

    fn get_callbacks(&mut self) -> &mut DefaultWidgetCallbacks {
        &mut self.callbacks
    }
}
