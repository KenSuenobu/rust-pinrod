// Timer Widget
// Timer-based widget that fires off a callback every time a certain time period is reached.
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

use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::callbacks::CallbackEvent;
use crate::core::widget_store::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// Creates a timer that can be used to generate callbacks based on a timeout.  When a timeout
/// has been reached, a `TimerTriggered` event is generated.  Set the timer timeout in
/// milliseconds by setting `CONFIG_TIMER_TIMEOUT` values.
///
/// Timer timeouts are triggered on a per-refresh basis, so if the FPS of the screen for the
/// main window is set to 30 FPS, the timer will only receive a tick every 1/30th of a second.  If
/// you need higher resolution timers, consider using threads.
pub struct TimerWidget {
    config: Configurable,
    enabled: bool,
    initiated: u64,
    triggered: bool,
    widget_id: i32,
    callbacks: DefaultWidgetCallbacks,
    on_tick: Option<Box<dyn FnMut(&mut TimerWidget)>>,
}

fn time_ms() -> u64 {
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    (since_the_epoch.as_secs() * 1_000) + (since_the_epoch.subsec_nanos() / 1_000_000) as u64
}

impl TimerWidget {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            enabled: true,
            initiated: time_ms(),
            triggered: false,
            widget_id: 0,
            callbacks: DefaultWidgetCallbacks::new(),
            on_tick: None,
        }
    }

    // Called to check the time since initiation, and call the timeout function when a timer has
    // been triggered.
    fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        let elapsed = time_ms() - self.initiated;

        if elapsed > self.config().get_numeric(CONFIG_TIMER_TIMEOUT) {
            self.initiated = time_ms();
            self.triggered = true;
        }
    }

    /// Assigns the callback closure that will be used when a timer tick is triggered.
    pub fn on_tick<F>(&mut self, callback: F)
    where
        F: FnMut(&mut TimerWidget) + 'static,
    {
        self.on_tick = Some(Box::new(callback));
    }

    /// Calls the click `on_tick` callback, if set.  Otherwise, ignored.  Sends a reference
    /// of the current `Widget` object as a parameter, so this object can be modified when
    /// a click is registered, if necessary.
    fn trigger_tick(&mut self) {
        if let Some(mut cb) = self.on_tick.take() {
            cb(self);
            self.on_tick = Some(cb);
        }
    }
}

impl Drawable for TimerWidget {}

impl InjectableSystemEvents for TimerWidget {}

impl InjectableCustomEvents for TimerWidget {
    fn inject_custom_event(&mut self, widget_id: i32) -> Option<CallbackEvent> {
        if self.triggered {
            self.triggered = false;
            self.trigger_tick();
            Some(CallbackEvent::TimerTriggered { widget_id })
        } else {
            None
        }
    }
}

impl Widget for TimerWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn is_invalidated(&mut self) -> bool {
        true
    }

    fn injects_custom_events(&mut self) -> bool {
        true
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
        if !injected {}

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

    fn is_drawable(&mut self) -> bool {
        self.tick();
        false
    }

    fn get_callbacks(&mut self) -> &mut DefaultWidgetCallbacks {
        &mut self.callbacks
    }
}
