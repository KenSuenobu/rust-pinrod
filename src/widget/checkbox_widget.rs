// Checkbox Widget
// Extensible widget for the widget library - handles a checkbox with text button.
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
use crate::widget::image_widget::*;
use crate::widget::text_widget::*;
use crate::widget::widget::*;

/// Draws a box with a toggleable checkbox, and text next to it.
pub struct CheckboxWidget {
    config: Configurable,
    base_widget: BoxWidget,
    text_widget: TextWidget,
    selected: bool,
    selected_widget: ImageWidget,
    unselected_widget: ImageWidget,
    widget_id: i32,
    callbacks: DefaultWidgetCallbacks,
}

impl CheckboxWidget {
    /// Constructor.  Requires the name of the font, the text to display, the image name to display, the size of the font,
    /// and the font justification when rendered.  Images are loaded from the `assets/` directory.
    pub fn new(
        font_name: String,
        text: String,
        font_size: u32,
        justify: TextJustify,
        selected: bool,
    ) -> Self {
        let mut selected_widget = ImageWidget::new("checkbox_selected.png".to_string());
        selected_widget.set_point(CONFIG_ORIGIN, 2, 2);
        selected_widget.set_toggle(CONFIG_WIDGET_HIDDEN, true);

        let mut unselected_widget = ImageWidget::new("checkbox_unselected.png".to_string());
        unselected_widget.set_point(CONFIG_ORIGIN, 2, 2);
        unselected_widget.set_toggle(CONFIG_WIDGET_HIDDEN, false);

        let text_widget =
            TextWidget::new(font_name.to_string(), text.to_string(), font_size, justify);

        Self {
            config: Configurable::new(),
            base_widget: BoxWidget::new(),
            text_widget,
            selected,
            selected_widget,
            unselected_widget,
            widget_id: 0,
            callbacks: DefaultWidgetCallbacks::new(),
        }
    }

    inject_event_handler!();
}

impl Drawable for CheckboxWidget {
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
        // Invalidation is controlled by the top level widget (this box).
        self.base_widget.draw(c, g, &clip);

        let size = self.config().get_size(CONFIG_BODY_SIZE);

        // Clear the drawing backing
        g.rectangle(
            &Rectangle::new(self.config().get_color(CONFIG_MAIN_COLOR)),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
        );

        if self.selected {
            self.selected_widget
                .get_drawable()
                .draw_with_offset(c, g, &clip, [0; 2]);
        } else {
            self.unselected_widget.get_drawable().draw_with_offset(
                c,
                g,
                &clip,
                [0; 2],
            );
        }

        self.text_widget
            .get_drawable()
            .draw_with_offset(c, g, &clip, [38, 0]);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}

impl InjectableSystemEvents for CheckboxWidget {}

impl InjectableCustomEvents for CheckboxWidget {}

impl Widget for CheckboxWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.base_widget.set_config(config, config_value.clone());
        self.text_widget.set_config(config, config_value.clone());

        if config == CONFIG_BODY_SIZE {
            let size = self.config().get_size(CONFIG_BODY_SIZE);

            if size.h < 32 {
                self.selected_widget
                    .set_size(CONFIG_BODY_SIZE, size.h, size.h);
                self.unselected_widget
                    .set_size(CONFIG_BODY_SIZE, size.h, size.h);
            } else {
                self.selected_widget
                    .set_point(CONFIG_ORIGIN, 0, (size.h - 32) / 2);
                self.selected_widget.set_size(CONFIG_BODY_SIZE, 32, 32);
                self.unselected_widget
                    .set_point(CONFIG_ORIGIN, 0, (size.h - 32) / 2);
                self.unselected_widget.set_size(CONFIG_BODY_SIZE, 32, 32);
            }
        }
    }

    fn handle_event(
        &mut self,
        injected: bool,
        event: CallbackEvent,
        widget_store: Option<&Vec<WidgetContainer>>,
    ) -> Option<CallbackEvent> {
        if !injected {
            match event {
                CallbackEvent::MouseButtonUpInside { widget_id, button } => match button {
                    Button::Mouse(mouse_button) => {
                        if mouse_button == MouseButton::Left {
                            self.selected = !self.selected;

                            let selected = self.selected;

                            if self.get_callbacks().has_on_toggle() {
                                match widget_store {
                                    Some(widgets) => {
                                        if let Some(mut cb) = self.get_callbacks().on_toggle.take()
                                        {
                                            cb(self, selected, widgets);
                                            self.get_callbacks().on_toggle = Some(cb);
                                        }
                                    }
                                    None => (),
                                }
                            }

                            self.invalidate();

                            return Some(CallbackEvent::WidgetSelected {
                                widget_id,
                                button,
                                selected: self.selected,
                            });
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
