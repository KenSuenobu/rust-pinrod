// Image Button Widget
// Extensible widget for the widget library - handles an image button object.
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

use crate::core::callbacks::CallbackEvent::WidgetClicked;
use crate::core::callbacks::*;
use crate::core::widget_store::*;
use crate::widget::box_widget::*;
use crate::widget::config::*;
use crate::widget::image_widget::*;
use crate::widget::text_widget::*;
use crate::widget::widget::*;

/// Draws an image in a box with specified `Text` next to it.
pub struct ImageButtonWidget {
    config: Configurable,
    base_widget: BoxWidget,
    text_widget: TextWidget,
    image_widget: ImageWidget,
    active: bool,
    widget_id: i32,
    callbacks: DefaultWidgetCallbacks,
}

impl ImageButtonWidget {
    /// Constructor.  Requires the name of the font, the text to display, the image name to display, the size of the font,
    /// and the font justification when rendered.  Images are loaded from the `assets/`
    /// directory.
    pub fn new(
        font_name: String,
        text: String,
        image_name: String,
        font_size: u32,
        justify: TextJustify,
    ) -> Self {
        let mut image_widget = ImageWidget::new(image_name.to_string());
        let mut text_widget =
            TextWidget::new(font_name.to_string(), text.to_string(), font_size, justify);

        text_widget.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 0.0]);
        image_widget.set_point(CONFIG_ORIGIN, 2, 2);

        Self {
            config: Configurable::new(),
            base_widget: BoxWidget::new(),
            text_widget,
            image_widget,
            active: false,
            widget_id: 0,
            callbacks: DefaultWidgetCallbacks::new(),
        }
    }

    fn draw_hovered(&mut self) {
        self.base_widget
            .set_color(CONFIG_MAIN_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.text_widget.set_color(CONFIG_TEXT_COLOR, [1.0; 4]);
        self.invalidate();
    }

    fn draw_unhovered(&mut self) {
        self.base_widget.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
        self.text_widget
            .set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.invalidate();
    }

    inject_event_handler!();
}

impl Drawable for ImageButtonWidget {
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        let size = self.base_widget.config().get_size(CONFIG_BODY_SIZE);
        let border = (self.base_widget.config().get_numeric(CONFIG_BORDER_WIDTH) * 2) as i32;

        self.image_widget.config().set_size(
            CONFIG_BODY_SIZE,
            size.h - border as i32,
            size.h - (border * 2) as i32,
        );

        self.base_widget.draw(c, g, &clip);

        self.image_widget
            .get_drawable()
            .draw_with_offset(c, g, &clip, [2, 2]);
        self.text_widget.get_drawable().draw_with_offset(
            c,
            g,
            &clip,
            [
                size.h + border + 4,
                0,
            ],
        );

        // Then clear invalidation.
        self.clear_invalidate();
    }
}

impl InjectableSystemEvents for ImageButtonWidget {}

impl InjectableCustomEvents for ImageButtonWidget {}

impl Widget for ImageButtonWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());

        self.base_widget.set_config(config, config_value.clone());
        self.text_widget.set_config(config, config_value.clone());
        self.image_widget.set_config(config, config_value.clone());
    }

    fn handle_event(
        &mut self,
        injected: bool,
        event: CallbackEvent,
        widget_store: Option<&Vec<WidgetContainer>>,
    ) -> Option<CallbackEvent> {
        if !injected {
            match event {
                CallbackEvent::MouseEntered { widget_id: _ } => {
                    if self.active {
                        self.draw_hovered();
                    }

                    self.handle_event_callbacks(event, widget_store);
                }

                CallbackEvent::MouseExited { widget_id: _ } => {
                    if self.active {
                        self.draw_unhovered();
                    }

                    self.handle_event_callbacks(event, widget_store);
                }

                CallbackEvent::MouseButtonDown {
                    widget_id: _,
                    button,
                } => match button {
                    Button::Mouse(mouse_button) => {
                        if mouse_button == MouseButton::Left {
                            self.draw_hovered();
                            self.active = true;
                        }
                    }
                    _ => (),
                },

                CallbackEvent::MouseButtonUpInside { widget_id, button } => match button {
                    Button::Mouse(mouse_button) => {
                        if mouse_button == MouseButton::Left {
                            self.draw_unhovered();
                            self.active = false;
                            self.handle_event_callbacks(event, widget_store);

                            return Some(WidgetClicked { widget_id, button });
                        }
                    }
                    _ => (),
                },

                CallbackEvent::MouseButtonUpOutside {
                    widget_id: _,
                    button,
                } => match button {
                    Button::Mouse(mouse_button) => {
                        if mouse_button == MouseButton::Left {
                            self.draw_unhovered();
                            self.active = false;
                        }
                    }
                    _ => (),
                },

                _ => self.handle_event_callbacks(event, widget_store),
            }
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
