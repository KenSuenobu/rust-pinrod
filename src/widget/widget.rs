// Widget Base Definition
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
use std::cell::RefMut;

use crate::core::callbacks::*;
use crate::core::point::{Point, Size};
use crate::core::widget_store::*;
use crate::widget::config::*;

pub trait Drawable {
    /// Draws the `Widget`'s contents.  Only gets called if the `Widget` is in invalidated
    /// state.  Provides a modified `Context` object that has an origin of `0x0` in drawing
    /// space for the draw routine.  Also provides a `mut G2d` object against which to draw,
    /// and a `clip`, which is automatically set to provide a clipping area for the `Widget`.  If
    /// the `Widget` draws outside of the clipped bounds, that will not be drawn on the
    /// screen.
    fn draw(&mut self, _c: Context, _g: &mut GlGraphics, _clip: &DrawState) {
        // Do nothing on the draw.
    }

    /// Draws an object at an offset on the screen.  This is a convenience method that is used
    /// by other `Widget`s that contain multiple widgets.  Draw with offset always uses 0x0 as
    /// the starting drawing coordinates, as this is what will be drawn within the bounds of the
    /// `Widget`.  (See `CheckboxWidget` and `ImageButtonWidget` for good examples of this use.)
    fn draw_with_offset(
        &mut self,
        c: Context,
        g: &mut GlGraphics,
        clip: &DrawState,
        point_offset: Point,
    ) {
        self.draw(
            c.trans(point_offset[0] as f64, point_offset[1] as f64),
            g,
            clip,
        );
    }
}

pub trait InjectableSystemEvents {
    /// Part of the main loop that queries the `Widget` for any system-level events that should
    /// be injected into the `PushrodCallbackEvents` trait, and not handled by the top-level
    /// run loop.  This sends out messages that are _bypassed_ from being used by the Run Loop,
    /// so be very careful.  Use this for sending things like custom messages (such as a `Widget`
    /// move or `Widget` resize message, which is irrelevant to the run loop.)
    fn inject_system_event(&mut self) -> Option<CallbackEvent> {
        None
    }
}

pub trait InjectableCustomEvents {
    /// Injects an event into the run loop.  This can be a timer event, a refresh event, or
    /// whatever the `Widget` wants to inject.  These should be custom events, not system
    /// events.  This method only gets called if `injects_events` returns `true`.
    fn inject_custom_event(&mut self, _widget_id: i32) -> Option<CallbackEvent> {
        None
    }
}

/// Master level trait object for describing a `Widget`.  A `Widget` is a GUI element that can
/// be interacted with and can receive and generate events.
pub trait Widget {
    /// Retrieves the `Configurable` object for this `Widget`.  All `Widget` implementations
    /// must provide this.  (See the `CanvasWidget` implementation.)
    fn config(&mut self) -> &mut Configurable;

    /// Indicates that a `Widget` object needs to be repainted.
    fn invalidate(&mut self) {
        if !self.is_invalidated() {
            self.config().set_toggle(CONFIG_INVALIDATE, true);
        }
    }

    /// Clears the invalidation flag.  Set this when the `draw` function completes.  Otherwise,
    /// this `Widget` object may be continuously repainted.
    fn clear_invalidate(&mut self) {
        self.config().remove(CONFIG_INVALIDATE);
    }

    /// Indicates whether or not a `Widget` needs to be repainted.
    fn is_invalidated(&mut self) -> bool {
        self.config().contains(CONFIG_INVALIDATE)
    }

    /// Master config setter - use convenience methods.
    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.invalidate();
    }

    /// Master config getter - use convenience methods.
    fn get_config(&mut self, config: u8) -> Option<&Config> {
        self.config().get(config)
    }

    /// Sets a point value for a configuration key.
    fn set_point(&mut self, config: u8, x: i32, y: i32) {
        self.set_config(config, Config::Point([x, y]));
    }

    /// Sets a size value for a configuration key.
    fn set_size(&mut self, config: u8, w: i32, h: i32) {
        self.set_config(config, Config::Size(Size { w, h }));
    }

    /// Sets a color for a configuration key.
    fn set_color(&mut self, config: u8, color: types::Color) {
        self.set_config(config, Config::Color(color));
    }

    /// Sets a numeric value for a configuration key.
    fn set_numeric(&mut self, config: u8, value: u64) {
        self.set_config(config, Config::Numeric(value));
    }

    /// Sets a text value for a configuration key.
    fn set_text(&mut self, config: u8, text: String) {
        self.set_config(config, Config::Text(text.clone()));
    }

    /// Sets a toggle value for a configuration key.
    fn set_toggle(&mut self, config: u8, flag: bool) {
        self.set_config(config, Config::Toggle(flag));
    }

    fn set_widget_id(&mut self, widget_id: i32);

    fn get_widget_id(&mut self) -> i32;

    /// Custom handler to receive an event.  Any `Widget` that implements this does so to handle
    /// top-level GUI events, such as a mouse entering or exiting the bounds of this `Widget`.
    /// If the `injected` flag is set, it indicates that the event supplied was generate by
    /// a `Widget`, and not by the run loop.
    fn handle_event(
        &mut self,
        _injected: bool,
        _event: CallbackEvent,
        _widget_store: Option<&Vec<WidgetContainer>>,
    ) -> Option<CallbackEvent> {
        None
    }

    /// Indicates to the run loop whether or not the `Widget` handles system-generated events.
    fn handles_events(&mut self) -> bool {
        false
    }

    /// Retrieves the `InjectableCustomEvents` trait of this class, which is responsible for
    /// injecting custom events when appropriate.  Injecting system events is used with the
    /// `InjectableSystemEvents`, and things like mouse clicks and widget clicks are used
    /// with the `handle_event` block.  This code is used to inject events that are not
    /// triggered by other events in the system.
    fn get_injectable_custom_events(&mut self) -> &mut dyn InjectableCustomEvents;

    /// If this `Widget` provides custom injected events that are generated outside of the
    /// `handle_event` loop, indicate `true`.  Only override if necessary.  (See `TimerWidget`
    /// for reference.)
    fn injects_custom_events(&mut self) -> bool {
        false
    }

    /// Retrieves the `Drawable` functionality of this `Widget`.
    fn get_drawable(&mut self) -> &mut dyn Drawable;

    /// Describes whether or not the `Widget` returns a `Drawable` trait.  This function is called each
    /// time a frame is refreshed, so if there is no `Drawable` available, this function could
    /// serve as a way to indicate a frame tick.  Only override this to set it to `false` if your
    /// `Widget` does not draw anything on the screen.
    fn is_drawable(&mut self) -> bool {
        true
    }

    /// Retrieves the trait for injecting system events.  Only use this if your `Widget` injects
    /// custom system-level events that the top-level application needs to use.  Anything other
    /// than that should be ignored completely.
    fn get_injectable_system_events(&mut self) -> &mut dyn InjectableSystemEvents;

    /// Indicates to the run loop whether or not a `Widget` injects system-level events.
    fn injects_system_events(&mut self) -> bool {
        false
    }

    /// Retrieves the callbacks that are registered for the `Widget`.  These callbacks are used
    /// when a specific event triggers an action - a click, mouse enter/exit, or movement event,
    /// for example.  The callbacks stored in the `DefaultWidgetCallbacks` object can be used
    /// to perform static actions.
    fn get_callbacks(&mut self) -> &mut DefaultWidgetCallbacks;
}

/// Base `Widget` object.  Displays a blank canvas, with the color set by the `CONFIG_MAIN_COLOR`
/// configuration option.  Defaults to white.
pub struct CanvasWidget {
    config: Configurable,
    event_list: Vec<CallbackEvent>,
    widget_id: i32,
    callbacks: DefaultWidgetCallbacks,
}

impl CanvasWidget {
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            event_list: vec![],
            widget_id: 0,
            callbacks: DefaultWidgetCallbacks::new(),
        }
    }
}

impl Drawable for CanvasWidget {
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);

        g.rectangle(
            &Rectangle::new(self.config().get_color(CONFIG_MAIN_COLOR)),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
        );

        self.clear_invalidate();
    }
}

impl InjectableSystemEvents for CanvasWidget {
    fn inject_system_event(&mut self) -> Option<CallbackEvent> {
        self.event_list.pop().clone()
    }
}

impl InjectableCustomEvents for CanvasWidget {}

impl Widget for CanvasWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_size(&mut self, config: u8, w: i32, h: i32) {
        self.set_config(config, Config::Size(Size { w, h }));

        if self.widget_id != 0 {
            self.event_list.push(CallbackEvent::WidgetResized {
                widget_id: self.widget_id,
                size: Size { w, h },
            });
        }
    }

    fn set_point(&mut self, config: u8, x: i32, y: i32) {
        self.set_config(config, Config::Point([x, y]));

        if self.widget_id != 0 {
            self.event_list.push(CallbackEvent::WidgetMoved {
                widget_id: self.widget_id,
                point: [x, y],
            });
        }
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

pub struct DefaultWidgetCallbacks {
    pub on_click: Option<Box<dyn FnMut(&mut dyn Widget, &Vec<WidgetContainer>)>>,
    pub on_toggle: Option<Box<dyn FnMut(&mut dyn Widget, bool, &Vec<WidgetContainer>)>>,
    pub on_mouse_move: Option<Box<dyn FnMut(&mut dyn Widget, Point, &Vec<WidgetContainer>)>>,
    pub on_mouse_button:
        Option<Box<dyn FnMut(&mut dyn Widget, Button, bool, &Vec<WidgetContainer>)>>,
    pub on_tick: Option<Box<dyn FnMut(&mut dyn Widget, &Vec<WidgetContainer>)>>,
    pub on_mouse_bounds: Option<Box<dyn FnMut(&mut dyn Widget, bool, &Vec<WidgetContainer>)>>,
    on_click_populated: bool,
    on_toggle_populated: bool,
    on_mouse_move_populated: bool,
    on_mouse_button_populated: bool,
    on_tick_populated: bool,
    on_mouse_bounds_populated: bool,
}

impl DefaultWidgetCallbacks {
    pub fn new() -> Self {
        Self {
            on_click: None,
            on_toggle: None,
            on_mouse_move: None,
            on_mouse_button: None,
            on_tick: None,
            on_mouse_bounds: None,
            on_click_populated: false,
            on_toggle_populated: false,
            on_mouse_move_populated: false,
            on_mouse_button_populated: false,
            on_tick_populated: false,
            on_mouse_bounds_populated: false,
        }
    }

    pub fn on_click<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, &Vec<WidgetContainer>) + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self.on_click_populated = true;
    }

    pub fn has_on_click(&mut self) -> bool {
        self.on_click_populated
    }

    pub fn on_toggle<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, bool, &Vec<WidgetContainer>) + 'static,
    {
        self.on_toggle = Some(Box::new(callback));
        self.on_toggle_populated = true;
    }

    pub fn has_on_toggle(&mut self) -> bool {
        self.on_toggle_populated
    }

    pub fn on_mouse_move<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, Point, &Vec<WidgetContainer>) + 'static,
    {
        self.on_mouse_move = Some(Box::new(callback));
        self.on_mouse_move_populated = true;
    }

    pub fn has_on_mouse_move(&mut self) -> bool {
        self.on_mouse_move_populated
    }

    pub fn on_mouse_button<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, Button, bool, &Vec<WidgetContainer>) + 'static,
    {
        self.on_mouse_button = Some(Box::new(callback));
        self.on_mouse_button_populated = true;
    }

    pub fn has_on_mouse_button(&mut self) -> bool {
        self.on_mouse_move_populated
    }

    pub fn on_tick<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, &Vec<WidgetContainer>) + 'static,
    {
        self.on_tick = Some(Box::new(callback));
        self.on_tick_populated = true;
    }

    pub fn has_on_tick(&mut self) -> bool {
        self.on_tick_populated
    }

    pub fn on_mouse_bounds<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, bool, &Vec<WidgetContainer>) + 'static,
    {
        self.on_mouse_bounds = Some(Box::new(callback));
        self.on_mouse_bounds_populated = true;
    }

    pub fn has_on_mouse_bounds(&mut self) -> bool {
        self.on_mouse_bounds_populated
    }
}

pub fn get_widget_by_name(widgets: &Vec<WidgetContainer>, name: String) -> RefMut<Box<dyn Widget>> {
    let pos = match widgets
        .iter()
        .find(|x| x.widget_name == String::from(name.clone()))
    {
        Some(x) => x.widget_id as usize,
        None => 0 as usize,
    };

    widgets[pos].widget.borrow_mut()
}

pub fn invalidate_all_widgets_except(widgets: &Vec<WidgetContainer>, skip_id: i32) {
    widgets.iter().for_each(|x| {
        if x.widget_id != skip_id {
            x.widget.borrow_mut().invalidate()
        }
    });
}

#[macro_export]
macro_rules! inject_event_handler {
    () => {
        fn handle_event_callbacks(
            &mut self,
            event: CallbackEvent,
            widget_store: Option<&Vec<WidgetContainer>>)
        {
            let widgets = match widget_store {
                Some(widgets) => widgets,
                _ => return,
            };

            match event {
                CallbackEvent::MouseButtonUpInside { widget_id: _, button } => match button {
                    Button::Mouse(mouse_button) => {
                        if mouse_button == MouseButton::Left {
                            if self.get_callbacks().has_on_click() {
                                if let Some(mut cb) = self.get_callbacks().on_click.take() {
                                    cb(self, widgets);
                                    self.get_callbacks().on_click = Some(cb);
                                }
                            }
                        }
                    }
                    _ => (),
                },

                CallbackEvent::MouseMoved { widget_id: _, point } => {
                    if self.get_callbacks().has_on_mouse_move() {
                        if let Some(mut cb) = self.get_callbacks().on_mouse_move.take() {
                            cb(self, point, widgets);
                            self.get_callbacks().on_mouse_move = Some(cb);
                        }
                    }
                },

                CallbackEvent::MouseEntered { widget_id: _ } => {
                    if self.get_callbacks().has_on_mouse_bounds() {
                        if let Some(mut cb) = self.get_callbacks().on_mouse_bounds.take() {
                            cb(self, true, widgets);
                            self.get_callbacks().on_mouse_bounds = Some(cb);
                        }
                    }
                }

                CallbackEvent::MouseExited { widget_id: _ } => {
                    if self.get_callbacks().has_on_mouse_bounds() {
                        if let Some(mut cb) = self.get_callbacks().on_mouse_bounds.take() {
                            cb(self, false, widgets);
                            self.get_callbacks().on_mouse_bounds = Some(cb);
                        }
                    }
                }

                _ => (),
            }
        }

    }
}
