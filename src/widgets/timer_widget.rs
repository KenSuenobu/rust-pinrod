// Pushrod Widget Library
// Timer Widget
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
use crate::render::widget_config::WidgetConfig;

use sdl2::render::Canvas;
use sdl2::video::Window;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn time_ms() -> u64 {
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    (since_the_epoch.as_secs() * 1_000) + u64::from(since_the_epoch.subsec_millis())
}

/// This is the storage object for the `TimerWidget`.  It stores the config, properties, callback registry,
/// an enabled flag, timeout, a last-time-triggered value, and a timeout callback store.
pub struct TimerWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    enabled: bool,
    timeout: u64,
    initiated: u64,
    on_timeout: Option<Box<dyn FnMut(&mut TimerWidget, &[WidgetContainer])>>,
}

/// Creates a new `TimerWidget`.  This `Widget` will call a function defined in `on_timeout` when
/// a specific number of milliseconds has elapsed.
impl TimerWidget {
    /// Creates a new `TimerWidget` object to call the `on_timeout` timeout callback every `timeout`
    /// milliseconds.  Setting `enabled` to `true` will automatically enable the timer, where as
    /// `false` will add the timer, but it will not be enabled.
    pub fn new(timeout: u64, enabled: bool) -> Self {
        Self {
            config: WidgetConfig::new(0, 0, 0, 0),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            enabled,
            timeout,
            initiated: time_ms(),
            on_timeout: None,
        }
    }

    /// Re-enables the timer.  This will also reset the elapsed timer.
    pub fn enable(&mut self) {
        self.initiated = time_ms();
        self.enabled = true;
    }

    /// Disables the timer.  Once disabled, the `on_timeout` callback will never be called.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns the `enabled` state.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Assigns the callback closure that will be used when a timer tick is triggered.
    pub fn on_timeout<F>(&mut self, callback: F)
    where
        F: FnMut(&mut TimerWidget, &[WidgetContainer]) + 'static,
    {
        self.on_timeout = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_timeout` callback.
    fn call_timeout_callback(&mut self, widgets: &[WidgetContainer]) {
        if let Some(mut cb) = self.on_timeout.take() {
            cb(self, widgets);
            self.on_timeout = Some(cb);
        }
    }
}

/// This is the `Widget` implementation of the `TimerWidget`.
impl Widget for TimerWidget {
    fn draw(&mut self, _c: &mut Canvas<Window>) {}

    /// When a timer tick goes by (ie. a frame is displayed on the screen), this function is
    /// called.  This function implementation is **optional**.
    fn tick(&mut self, _widgets: &[WidgetContainer]) {
        if !self.enabled {
            return;
        }

        let elapsed = time_ms() - self.initiated;

        if elapsed > self.timeout {
            self.initiated = time_ms();
            self.call_timeout_callback(_widgets);
        }
    }

    default_widget_properties!();
}
