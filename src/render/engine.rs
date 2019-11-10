// Pushrod Rendering Library
// Extensible Core Library
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

use sdl2::event::Event;
//use sdl2::messagebox::*;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::render::widget::{BaseWidget, Widget};
use crate::render::widget_cache::WidgetCache;
use std::time::Duration;

/// This is a storage container for the Pushrod event engine.
pub struct Engine {
    cache: WidgetCache,
    current_widget_id: i32,
}

/// This is the heart of the Pushrod event engine, and is what is used to drive the interaction
/// between the user and your application.  The suggested method of use for this code is as
/// follows:
///
/// ## Create a new object
/// Use `Engine::new()` to instantiate a new object.
///
/// ## Set up the base
/// Call `engine.setup(width, height)`, by passing the width and height of the window that is
/// being created for the main application.  This is required, as a base widget is added to the
/// render list.  This `BaseWidget` is considered the parent of the application screen.
///
/// ## Add Widgets to the Engine
/// Call `add_widget(Box::new(widget), "name".to_string())` to add your `Widget` to the managed
/// display list.
///
/// ## Call Run()
/// Calling `run(sdl, window)` will manage the application screen after all widgets have been added
/// to the display list.
///
/// That's all there is to it.  If you want to see more interactions on how the `Engine` is used in
/// an application, check out the demo test code, and look at `rust-pushrod-chassis`.
impl Engine {
    /// Creates a new `Engine` object.
    pub fn new() -> Self {
        Self {
            cache: WidgetCache::new(),
            current_widget_id: 0,
        }
    }

    /// Sets up the top-level widget so that other widgets can be added to the screen.
    pub fn setup(&mut self, window_width: u32, window_height: u32) {
        let base_widget = BaseWidget::new(0, 0, window_width, window_height);

        self.cache
            .add_widget(Box::new(base_widget), "base".to_string());
    }

    /// Adds a widget to the display list.  Widgets are rendered in the order in which they were
    /// created in the display list.
    pub fn add_widget(&mut self, widget: Box<dyn Widget>, widget_name: String) -> i32 {
        self.cache.add_widget(widget, widget_name)
    }

    /// Main application run loop, controls interaction between the user and the application.
    pub fn run(&mut self, sdl: Sdl, window: Window) {
        let mut canvas = window.into_canvas().software().build().unwrap();

        canvas.clear();
        canvas.present();

        let mut event_pump = sdl.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::MouseButtonDown {
                        mouse_btn, clicks, ..
                    } => {
                        self.cache.button_clicked(
                            self.current_widget_id,
                            mouse_btn as u8,
                            clicks,
                            true,
                        );
                    }

                    Event::MouseButtonUp {
                        mouse_btn, clicks, ..
                    } => {
                        self.cache
                            .button_clicked(-1, mouse_btn as u8, clicks, false);
                    }

                    Event::MouseMotion { x, y, .. } => {
                        let cur_widget_id = self.current_widget_id;

                        self.current_widget_id = self.cache.find_widget(x, y);

                        if cur_widget_id != self.current_widget_id {
                            self.cache.mouse_exited(cur_widget_id);
                            self.cache.mouse_entered(self.current_widget_id);
                        }

                        self.cache.mouse_moved(self.current_widget_id, vec![x, y]);
                    }

                    Event::MouseWheel { x, y, .. } => {
                        self.cache
                            .mouse_scrolled(self.current_widget_id, vec![x, y]);
                    }

                    Event::Quit { .. } => {
                        //                        let buttons: Vec<_> = vec![
                        //                            ButtonData {
                        //                                flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
                        //                                button_id: 1,
                        //                                text: "Yes",
                        //                            },
                        //                            ButtonData {
                        //                                flags: MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
                        //                                button_id: 2,
                        //                                text: "No",
                        //                            },
                        //                        ];
                        //
                        //                        let res = show_message_box(
                        //                            MessageBoxFlag::WARNING,
                        //                            buttons.as_slice(),
                        //                            "Quit",
                        //                            "Are you sure?",
                        //                            None,
                        //                            None,
                        //                        )
                        //                        .unwrap();
                        //
                        //                        if let ClickedButton::CustomButton(x) = res {
                        //                            if x.button_id == 1 {
                        break 'running;
                        //                            }
                        //                        }
                    }

                    remaining_event => {
                        self.cache
                            .other_event(self.current_widget_id, remaining_event);
                    }
                }
            }

            self.cache.tick();
            self.cache.draw_loop(&mut canvas);

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
