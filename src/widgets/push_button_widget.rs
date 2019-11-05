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
use crate::render::widget_config::{WidgetConfig, CONFIG_COLOR_BASE, CONFIG_SIZE};
use crate::render::Points;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

use std::collections::HashMap;
use std::path::Path;

/// This is the storage object for the `PushButtonWidget`.  It stores the config, properties, callback registry.
pub struct PushButtonWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
//    base_widget: BoxWidget,
//    text_widget: TextWidget,
    active: bool,
}

impl PushButtonWidget {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
//        let mut text_widget =
//            TextWidget::new(font_name.to_string(), text.to_string(), font_size, justify);
//
//        text_widget.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 0.0]);

        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            active: false,
        }
    }

//    fn draw_hovered(&mut self) {
//        self.base_widget
//            .set_color(CONFIG_MAIN_COLOR, [0.0, 0.0, 0.0, 1.0]);
//        self.text_widget.set_color(CONFIG_TEXT_COLOR, [1.0; 4]);
//        self.invalidate();
//    }
//
//    fn draw_unhovered(&mut self) {
//        self.base_widget.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
//        self.text_widget
//            .set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
//        self.invalidate();
//    }
}

/// This is the `Widget` implementation of the `PushButtonWidget`.
impl Widget for PushButtonWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) {
//        // Paint the base widget first.  Forcing a draw() call here will ignore invalidation.
//        // Invalidation is controlled by the top level widget (this box).
//        self.base_widget.draw(c, g, &clip);
//        self.text_widget.draw(c, g, &clip);
//
//        // Then clear invalidation.
//        self.clear_invalidate();
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer]) {
        self.mouse_entered_callback(_widgets);
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.  This function
    /// implementation is **optional**.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer]) {
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
        self.button_clicked_callback(_widgets, _button, _clicks, _state);
    }

//    fn set_config(&mut self, config: u8, config_value: Config) {
//        self.config().set(config, config_value.clone());
//        self.base_widget.set_config(config, config_value.clone());
//        self.text_widget.set_config(config, config_value.clone());
//    }
//
//    fn handle_event(
//        &mut self,
//        injected: bool,
//        event: CallbackEvent,
//        widget_store: Option<&Vec<WidgetContainer>>,
//    ) -> Option<CallbackEvent> {
//        if !injected {
//            match event {
//                CallbackEvent::MouseEntered { widget_id: _ } => {
//                    if self.active {
//                        self.draw_hovered();
//                    }
//
//                    self.handle_event_callbacks(event, widget_store);
//                }
//
//                CallbackEvent::MouseExited { widget_id: _ } => {
//                    if self.active {
//                        self.draw_unhovered();
//                    }
//
//                    self.handle_event_callbacks(event, widget_store);
//                }
//
//                CallbackEvent::MouseButtonDown {
//                    widget_id: _,
//                    button,
//                } => match button {
//                    Button::Mouse(mouse_button) => {
//                        if mouse_button == MouseButton::Left {
//                            self.draw_hovered();
//                            self.active = true;
//                        }
//                    }
//                    _ => (),
//                },
//
//                CallbackEvent::MouseButtonUpInside { widget_id, button } => match button {
//                    Button::Mouse(mouse_button) => {
//                        if mouse_button == MouseButton::Left {
//                            self.draw_unhovered();
//                            self.active = false;
//                            self.handle_event_callbacks(event, widget_store);
//
//                            return Some(WidgetClicked { widget_id, button });
//                        }
//                    }
//                    _ => (),
//                },
//
//                CallbackEvent::MouseButtonUpOutside {
//                    widget_id: _,
//                    button,
//                } => match button {
//                    Button::Mouse(mouse_button) => {
//                        if mouse_button == MouseButton::Left {
//                            self.draw_unhovered();
//                            self.active = false;
//                        }
//                    }
//                    _ => (),
//                },
//
//                _ => self.handle_event_callbacks(event, widget_store),
//            }
//        }
//
//        None
//    }

    default_widget_properties!();
    default_widget_callbacks!();
}
