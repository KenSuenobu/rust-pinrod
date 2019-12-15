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
use sdl2::video::Window;
use sdl2::Sdl;

use crate::render::layout::Layout;
use crate::render::layout_cache::LayoutCache;
use crate::render::widget::{BaseWidget, Widget};
use crate::render::widget_cache::WidgetCache;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// This is a storage container for the Pushrod event engine.
pub struct Engine {
    widget_cache: WidgetCache,
    layout_cache: LayoutCache,
    current_widget_id: i32,
    frame_rate: u8,
    running: bool,
}

/// This is the heart of the Pushrod event engine, and is what is used to drive the interaction
/// between the user and your application.  The suggested method of use for this code is as
/// follows:
///
/// ## Create a new object
/// Use `Engine::new(w, h, frame_rate)` to instantiate a new object.
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
    /// Creates a new `Engine` object.  Sets the engine up with the bounds of the screen, and the desired
    /// FPS rate, which must be provided at instantiation time.  This is in order to set up the
    /// `BaseWidget` in the top-level of the `Engine`, so that it knows what area of the screen to
    /// refresh when required as part of the draw cycle.
    ///
    /// **NOTE**: Setting a lower frame_rate will increase the efficiency of your API, however, it
    /// could lower responsiveness if you have a very active UI.
    pub fn new(w: u32, h: u32, frame_rate: u8) -> Self {
        let base_widget = BaseWidget::new(0, 0, w, h);
        let mut cache = WidgetCache::default();

        cache.add_widget(Box::new(base_widget), "base".to_string());

        Self {
            widget_cache: cache,
            layout_cache: LayoutCache::default(),
            current_widget_id: 0,
            frame_rate,
            running: true,
        }
    }

    /// Adds a `Widget` to the display list.  `Widget`s are rendered in the order in which they were
    /// created in the display list.
    pub fn add_widget(&mut self, widget: Box<dyn Widget>, widget_name: String) -> i32 {
        self.widget_cache.add_widget(widget, widget_name)
    }

    /// Adds a `Layout` to the `Layout` list.
    pub fn add_layout(&mut self, layout: Box<dyn Layout>) -> i32 {
        self.layout_cache.add_layout(layout)
    }

    /// Sets running flag: `false` shuts down the engine.
    pub fn set_running(&mut self, state: bool) {
        self.running = state;
    }

    /// Main application run loop, controls interaction between the user and the application.
    pub fn run(&mut self, sdl: Sdl, window: Window) {
        let mut canvas = window.into_canvas().software().build().unwrap();

        canvas.clear();
        canvas.present();

        let mut event_pump = sdl.event_pump().unwrap();
        let fps_as_ms = (1000.0 / self.frame_rate as f64) as u128;

        'running: loop {
            let start = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            for event in event_pump.poll_iter() {
                match event {
                    Event::MouseButtonDown {
                        mouse_btn, clicks, ..
                    } => {
                        self.widget_cache.button_clicked(
                            self.current_widget_id,
                            mouse_btn as u8,
                            clicks,
                            true,
                            self.layout_cache.get_layout_cache(),
                        );
                    }

                    Event::MouseButtonUp {
                        mouse_btn, clicks, ..
                    } => {
                        self.widget_cache.button_clicked(
                            -1,
                            mouse_btn as u8,
                            clicks,
                            false,
                            self.layout_cache.get_layout_cache(),
                        );
                    }

                    Event::MouseMotion { x, y, .. } => {
                        let cur_widget_id = self.current_widget_id;

                        self.current_widget_id = self.widget_cache.find_widget(x, y);

                        if cur_widget_id != self.current_widget_id {
                            self.widget_cache
                                .mouse_exited(cur_widget_id, self.layout_cache.get_layout_cache());
                            self.widget_cache.mouse_entered(
                                self.current_widget_id,
                                self.layout_cache.get_layout_cache(),
                            );
                        }

                        self.widget_cache.mouse_moved(
                            self.current_widget_id,
                            vec![x, y],
                            self.layout_cache.get_layout_cache(),
                        );
                    }

                    Event::MouseWheel { x, y, .. } => {
                        self.widget_cache.mouse_scrolled(
                            self.current_widget_id,
                            vec![x, y],
                            self.layout_cache.get_layout_cache(),
                        );
                    }

                    Event::Quit { .. } => {
                        break 'running;
                    }

                    remaining_event => {
                        self.widget_cache.other_event(
                            self.current_widget_id,
                            remaining_event,
                            self.layout_cache.get_layout_cache(),
                        );
                    }
                }
            }

            self.widget_cache.tick(self.layout_cache.get_layout_cache());
            self.layout_cache
                .do_layout(self.widget_cache.borrow_cache());
            self.widget_cache.draw_loop(&mut canvas);

            // This obeys thread sleep time.
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            if now - start < fps_as_ms {
                let diff = fps_as_ms - (now - start);

                sleep(Duration::from_millis(diff as u64));
            }

            if !self.running {
                break 'running;
            }
        }
    }
}
