// Callback Event System
// Callback Events
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

use crate::core::point::{Point, Size};
use crate::core::widget_store::*;
use piston::input::*;

/// This is the callback event class that is used to handle events that are produced when a widget
/// is interacted with in the Pushrod Run Loop.  This callback is triggered when an event happens
/// either in the main GUI, or within a GUI object.
pub trait PushrodCallbackEvents {
    /// This is the main event handling function, and can be overridden if necessary.  This function
    /// is called when an event is handled by the main run loop.  As an event happens in real time,
    /// this function is called and the translated event is sent here.  If this function is
    /// _not_ overridden, it will call the helper methods below, which can be overridden by the
    /// main application as it sees fit for each event type.  It is not necessary to implement
    /// each callback - only the ones you wish to implement.
    fn handle_event(&mut self, event: CallbackEvent, widget_store: &mut WidgetStore) {
        match event {
            CallbackEvent::MouseEntered { widget_id } => {
                self.mouse_entered(widget_id, widget_store)
            }
            CallbackEvent::MouseExited { widget_id } => self.mouse_exited(widget_id, widget_store),
            CallbackEvent::MouseScrolled { widget_id, point } => {
                self.mouse_scrolled(widget_id, point, widget_store)
            }
            CallbackEvent::MouseMoved { widget_id, point } => {
                self.mouse_moved(widget_id, point, widget_store)
            }
            CallbackEvent::KeyPressed {
                widget_id,
                key,
                state,
            } => self.key_pressed(widget_id, key, state, widget_store),
            CallbackEvent::WindowResized { size } => self.window_resized(size, widget_store),
            CallbackEvent::WindowFocused { flag } => self.window_focused(flag, widget_store),
            CallbackEvent::MouseButtonDown { widget_id, button } => {
                self.mouse_button_down(widget_id, button, widget_store)
            }
            CallbackEvent::MouseButtonUpInside { widget_id, button } => {
                self.mouse_button_up_inside(widget_id, button, widget_store)
            }
            CallbackEvent::MouseButtonUpOutside { widget_id, button } => {
                self.mouse_button_up_outside(widget_id, button, widget_store)
            }
            CallbackEvent::WidgetClicked { widget_id, button } => {
                self.widget_clicked(widget_id, button, widget_store)
            }
            CallbackEvent::WidgetSelected {
                widget_id,
                button,
                selected,
            } => self.widget_selected(widget_id, button, selected, widget_store),
            CallbackEvent::WidgetMoved { widget_id, point } => {
                self.widget_moved(widget_id, point, widget_store)
            }
            CallbackEvent::WidgetResized { widget_id, size } => {
                self.widget_resized(widget_id, size, widget_store)
            }
            CallbackEvent::TimerTriggered { widget_id } => {
                self.timer_triggered(widget_id, widget_store)
            }
            // Radio button deselection is handled internally by the `RadioButtonWidget`, and
            // are specifically filtered out here.  If you wish to respond to this widget, you must
            // implement the handle_event function yourself.
            CallbackEvent::UnselectRadioButtons {
                widget_id: _,
                group_id: _,
            } => (),
        }
    }

    /// Called when a mouse enters a `Widget`.
    fn mouse_entered(&mut self, _widget_id: i32, _widget_store: &mut WidgetStore) {}

    /// Called when a mouse exits a `Widget`.
    fn mouse_exited(&mut self, _widget_id: i32, _widget_store: &mut WidgetStore) {}

    /// Called when a mouse scroll wheel is used inside a `Widget`.
    fn mouse_scrolled(&mut self, _widget_id: i32, _point: Point, _widget_store: &mut WidgetStore) {}

    /// Called when a mouse moves inside a `Widget`.
    fn mouse_moved(&mut self, _widget_id: i32, _point: Point, _widget_store: &mut WidgetStore) {}

    /// Called when a keyboard keypress is detected.  The state of the key press is passed as well.
    fn key_pressed(
        &mut self,
        _widget_id: i32,
        _key: Key,
        _state: ButtonState,
        _widget_store: &mut WidgetStore,
    ) {
    }

    /// Called when the main window is resized.
    fn window_resized(&mut self, _size: crate::core::point::Size, _widget_store: &mut WidgetStore) {
    }

    /// Called when the window gains or loses focus.
    fn window_focused(&mut self, _flag: bool, _widget_store: &mut WidgetStore) {}

    /// Called when a mouse button is pressed.
    fn mouse_button_down(
        &mut self,
        _widget_id: i32,
        _button: Button,
        _widget_store: &mut WidgetStore,
    ) {
    }

    /// Called when a mouse button is released within the same `Widget` which it was pressed.
    fn mouse_button_up_inside(
        &mut self,
        _widget_id: i32,
        _button: Button,
        _widget_store: &mut WidgetStore,
    ) {
    }

    /// Called when a mouse button is released outside of the scope of the `Widget` from which it was
    /// pressed.
    fn mouse_button_up_outside(
        &mut self,
        _widget_id: i32,
        _button: Button,
        _widget_store: &mut WidgetStore,
    ) {
    }

    /// Called when a full click is detected inside a `Widget`.  A "click event" consists of a mouse
    /// button down and release within the confines of the same `Widget`.
    fn widget_clicked(
        &mut self,
        _widget_id: i32,
        _button: Button,
        _widget_store: &mut WidgetStore,
    ) {
    }

    /// Called when a `Widget` is selected.  This is a generated event by a `Widget`, and is not part
    /// of the main run loop.  This `Widget` is generated by event injection.
    fn widget_selected(
        &mut self,
        _widget_id: i32,
        _button: Button,
        _selected: bool,
        _widget_store: &mut WidgetStore,
    ) {
    }

    /// Called when a `Widget` is moved.  Contains the ID of the `Widget`, along with its new
    /// position as a `Point`.
    fn widget_moved(&mut self, _widget_id: i32, _point: Point, _widget_store: &mut WidgetStore) {}

    /// Called when a `Widget` is resized.  Contains the ID of the `Widget`, along with its new
    /// `Size`.
    fn widget_resized(&mut self, _widget_id: i32, _size: Size, _widget_store: &mut WidgetStore) {}

    /// Called when a timer expires for a widget.  The ID of the widget is the timer widget that
    /// generated the expiration timeout.
    fn timer_triggered(&mut self, _widget_id: i32, _widget_store: &mut WidgetStore) {}
}

/// These are the different types of events that can be triggered.  Any other callback events
/// should be extended in this enum definition.
#[derive(Clone, Debug)]
pub enum CallbackEvent {
    /// Indicates a mouse entered the bounds of a `Widget`.  Contains the ID of the `Widget` that was
    /// affected.
    MouseEntered { widget_id: i32 },

    /// Indicates a mouse exited the bounds of a `Widget`.  Contains the ID of the `Widget` that was
    /// affected.
    MouseExited { widget_id: i32 },

    /// Indicates that the scroll wheel was moved inside a `Widget`.  Contains the ID of the
    /// `Widget` that had the mouse scroll action, and the point in the direction of the scroll,
    /// along with the amount of points the mouse scroll moved.
    MouseScrolled { widget_id: i32, point: Point },

    /// Indicates that a mouse moved within the bounds of a `Widget`.  Contains the ID of the
    /// `Widget` that was affected.
    MouseMoved { widget_id: i32, point: Point },

    /// Indicates that a keyboard key was pressed/released inside the bounds of a `Widget`.  Contains
    /// the ID of the `Widget` that received the keypress, along with the `Key` value, and any
    /// associated `Button` modifier states.
    KeyPressed {
        widget_id: i32,
        key: Key,
        state: ButtonState,
    },

    /// Indicates that the main application window was resized.  Contains the `Size` of the new
    /// bounds.
    WindowResized { size: crate::core::point::Size },

    /// Indicates whether or not focus was gained or lost for the main application.  Contains a
    /// boolean flag indicating focus: `true` is focused, `false` if lost.
    WindowFocused { flag: bool },

    /// Indicates that a mouse button was pressed within the bounds of a `Widget`.  Contains the
    /// ID of the `Widget`, along with the `Button` that was clicked.
    MouseButtonDown { widget_id: i32, button: Button },

    /// Indicates that a mouse button was released within the bounds of a `Widget`.  Contains
    /// the ID of the `Widget`, along with the `Button` that was released.
    MouseButtonUpInside { widget_id: i32, button: Button },

    /// Indicates that a mouse button was released outside of the bounds of a `Widget`.  Contains
    /// the ID of the `Widget`, along with the `Button` that was released.
    MouseButtonUpOutside { widget_id: i32, button: Button },

    /// Indicates that a mouse button triggered a click action within a `Widget`.  Contains the
    /// ID of the `Widget`, along with the `Button` that was used to indicate the click action.
    WidgetClicked { widget_id: i32, button: Button },

    /// Indicates that a `Widget`'s selected state has been toggled.  Contains the ID of the
    /// `Widget` that was toggled, along with the mouse `Button` that was clicked, and the
    /// final `selected` state of the widget.
    WidgetSelected {
        widget_id: i32,
        button: Button,
        selected: bool,
    },

    /// Indicates that a `Widget`'s position has been moved either through physical interaction,
    /// or through a `LayoutManager` repositioning.  Contains the ID of the `Widget` that moved,
    /// along with its new position as a `Point`.
    WidgetMoved { widget_id: i32, point: Point },

    /// Indicates that a `Widget`'s `Size` has changed.  Contains the ID of the `Widget`, along with
    /// the new `Size` of the `Widget`.
    WidgetResized { widget_id: i32, size: Size },

    /// Indicates that a timer timeout has been triggered.  Contains the ID of the `Widget` that was
    /// affected.
    TimerTriggered { widget_id: i32 },

    /// Indicates that a group of `RadioButtonWidget` objects has been deselected.  Contains the
    /// ID of the `Widget` that was unselected, along with the group ID.
    UnselectRadioButtons { widget_id: i32, group_id: i32 },
}
