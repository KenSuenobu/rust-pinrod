// Progress Widget
// Handles the display of a progress bar
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
use crate::core::widget_store::*;
use crate::widget::box_widget::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// Draws a progress bar, with progress being a value from 0 to 100.  Configurable options
/// are:
/// * `CONFIG_BORDER_WIDTH` - configures the border width of the progress bar.
/// * `CONFIG_BORDER_COLOR` - configures the border color of the progress bar.
/// * `CONFIG_SECONDARY_COLOR` - configures the fill color of the progress bar.
/// * `CONFIG_PROGRESS` - configures the progress by percentage from 0-100.
pub struct ProgressWidget {
    config: Configurable,
    base_widget: BoxWidget,
    widget_id: i32,
    callbacks: DefaultWidgetCallbacks,
}

impl ProgressWidget {
    pub fn new() -> Self {
        let mut base = BoxWidget::new();

        base.config().set(CONFIG_BORDER_WIDTH, Config::Numeric(1));
        base.config()
            .set(CONFIG_BORDER_COLOR, Config::Color([0.0, 0.0, 0.0, 1.0]));

        // Configurable set: set progress.

        Self {
            config: Configurable::new(),
            base_widget: base,
            widget_id: 0,
            callbacks: DefaultWidgetCallbacks::new(),
        }
    }

    inject_event_handler!();
}

impl Drawable for ProgressWidget {
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size = self.config().get_size(CONFIG_BODY_SIZE);

        // Clear the drawing backing
        g.rectangle(
            &Rectangle::new(self.config().get_color(CONFIG_MAIN_COLOR)),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
        );

        self.base_widget.draw(c, g, clip);

        let mut draw_width =
            (size.w as f64 * (self.config().get_numeric(CONFIG_PROGRESS) as f64 / 100.0)) as f64
                - 2.0;

        if draw_width < 0.0 {
            draw_width = 0.0;
        }

        // Paint the secondary color to display the progress color.
        Rectangle::new(self.config().get_color(CONFIG_SECONDARY_COLOR)).draw(
            [1.0 as f64, 1.0 as f64, draw_width, (size.h - 2) as f64],
            clip,
            c.transform,
            g,
        );

        // Then clear invalidation.
        self.clear_invalidate();
    }
}

impl InjectableSystemEvents for ProgressWidget {}

impl InjectableCustomEvents for ProgressWidget {}

impl Widget for ProgressWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.base_widget.config().set(config, config_value.clone());
        self.invalidate();
    }

    fn set_widget_id(&mut self, widget_id: i32) {
        self.widget_id = widget_id;
    }

    fn get_widget_id(&mut self) -> i32 {
        self.widget_id
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
