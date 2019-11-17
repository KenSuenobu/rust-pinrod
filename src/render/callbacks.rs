// Pushrod Rendering Library
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

use crate::render::widget::Widget;
use crate::render::widget_cache::WidgetContainer;
use crate::render::layout_cache::LayoutContainer;

/// This is an `FnMut` type that takes no additional parameters, returning a mutable reference
/// to the current `Widget`, and borrowing the `WidgetContainer` and `LayoutContainer` lists.
pub type FunctionNoParametersType = Option<Box<dyn FnMut(&mut dyn Widget, &[WidgetContainer], &[LayoutContainer])>>;

/// This is an `FnMut` that takes a `Point` as a `Vec<i32>` of points: X and Y, returning a mutable reference
/// to the current `Widget`, and borrowing the `WidgetContainer` and `LayoutContainer` lists.
pub type FunctionPointParametersType =
    Option<Box<dyn FnMut(&mut dyn Widget, &[WidgetContainer], &[LayoutContainer], Vec<i32>)>>;

/// This is an `FnMut` that takes a button click ID, the number of clicks, the click state (`true` indicating
/// the click was pressed, `false` otherwise), returning a mutable reference
/// to the current `Widget`, and borrowing the `WidgetContainer` and `LayoutContainer` lists.
pub type FunctionClickParametersType =
    Option<Box<dyn FnMut(&mut dyn Widget, &[WidgetContainer], &[LayoutContainer], u8, u8, bool)>>;

/// This is a registry that contains a series of `FnMut` definitions for actions that can be applied
/// to a `Widget`.  These can vary from a screen refresh (`tick`), to a mouse move event, etc.  Each
/// callback gains access to the list of `WidgetContainer` objects stored by the cache.  This is
/// important in case you wish to modify other `Widget`s on the screen as a result of some action
/// that took place.
///
/// Keep in mind, however, that you _cannot_ re-borrow your own widget from the `WidgetContainer`
/// list, as this will cause a runtime exception.  For that, use the top-level `Widget` object that
/// was supplied.  This will allow you to make changes to the current `Widget` reference, since it
/// is an active, `mutable` reference.
#[derive(Default)]
pub struct CallbackRegistry {
    /// This is the function that is set when a screen refresh cycle occurs.  This function is
    /// always guaranteed to be called, but there is no guarantee it will call it consistently
    /// because of the screen refresh rate.  If there is a lot of activity on the screen, this
    /// callback will be called less often.
    pub on_tick: FunctionNoParametersType,

    /// This function is called when a mouse enters the scope of a `Widget`.
    pub on_mouse_entered: FunctionNoParametersType,

    /// This function is called when a mouse exits the scope of a `Widget`.
    pub on_mouse_exited: FunctionNoParametersType,

    /// This function is called when a mouse moves inside the scope of a `Widget`.  It contains
    /// the points as a `Vec<i32>` containing the X and Y coordinates of the position of the mouse
    /// inside the `Widget`.
    pub on_mouse_moved: FunctionPointParametersType,

    /// This function is called when a mouse scroll occurs inside the scope of a `Widget`.  It
    /// contains the points as a `Vec<u8>` indicating the amount of movement either horizontally or
    /// vertically.
    pub on_mouse_scrolled: FunctionPointParametersType,

    /// This function is called when a mouse button is pressed or released.  It contains the mouse
    /// button number, the number of clicks registered, and a boolean flag indicating whether or not
    /// the mouse button was pressed (`true`) or released (`false`).
    pub on_mouse_clicked: FunctionClickParametersType,

    has_on_tick: bool,
    has_on_mouse_entered: bool,
    has_on_mouse_exited: bool,
    has_on_mouse_moved: bool,
    has_on_mouse_scrolled: bool,
    has_on_mouse_clicked: bool,
}

/// Implementation of the `CallbackRegistry`.
impl CallbackRegistry {
    /// Creates a new instance of this object.
    pub fn new() -> Self {
        Self {
            on_tick: None,
            on_mouse_entered: None,
            on_mouse_exited: None,
            on_mouse_moved: None,
            on_mouse_scrolled: None,
            on_mouse_clicked: None,
            has_on_tick: false,
            has_on_mouse_entered: false,
            has_on_mouse_exited: false,
            has_on_mouse_moved: false,
            has_on_mouse_scrolled: false,
            has_on_mouse_clicked: false,
        }
    }

    /// Assigns an `FnMut` that will be called when a screen `tick` refresh is performed.  If this
    /// is not set, this function will be bypassed.
    pub fn on_tick<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, &[WidgetContainer], &[LayoutContainer]) + 'static,
    {
        self.on_tick = Some(Box::new(callback));
        self.has_on_tick = true;
    }

    /// Assigns an `FnMut` that will be called when the mouse enters the scope of a `Widget`.  If this
    /// is not set, this function will be bypassed.
    pub fn on_mouse_entered<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, &[WidgetContainer], &[LayoutContainer]) + 'static,
    {
        self.on_mouse_entered = Some(Box::new(callback));
        self.has_on_mouse_entered = true;
    }

    /// Assigns an `FnMut` that will be called when the mouse exits the scope of a `Widget`.  If this
    /// is not set, this function will be bypassed.
    pub fn on_mouse_exited<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, &[WidgetContainer], &[LayoutContainer]) + 'static,
    {
        self.on_mouse_exited = Some(Box::new(callback));
        self.has_on_mouse_exited = true;
    }

    /// Assigns an `FnMut` that will be called when the mouse moves within the scope of a `Widget`.  If this
    /// is not set, this function will be bypassed.
    pub fn on_mouse_moved<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, &[WidgetContainer], &[LayoutContainer], Vec<i32>) + 'static,
    {
        self.on_mouse_moved = Some(Box::new(callback));
        self.has_on_mouse_moved = true;
    }

    /// Assigns an `FnMut` that will be called when the mouse scroll occurs within the scope of a
    /// `Widget`.  If this is not set, this function will be bypassed.
    pub fn on_mouse_scrolled<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, &[WidgetContainer], &[LayoutContainer], Vec<i32>) + 'static,
    {
        self.on_mouse_scrolled = Some(Box::new(callback));
        self.has_on_mouse_scrolled = true;
    }

    /// Assigns an `FnMut` that will be called when the mouse click occurs within the scope of a
    /// `Widget`.  If this is not set, this function will be bypassed.
    pub fn on_mouse_clicked<F>(&mut self, callback: F)
    where
        F: FnMut(&mut dyn Widget, &[WidgetContainer], &[LayoutContainer], u8, u8, bool) + 'static,
    {
        self.on_mouse_clicked = Some(Box::new(callback));
        self.has_on_mouse_clicked = true;
    }

    /// Tells the `Widget` whether or not an `on_tick` callback has been set.
    pub fn has_on_tick(&mut self) -> bool {
        self.has_on_tick
    }

    /// Tells the `Widget` whether or not an `on_mouse_entered` callback has been set.
    pub fn has_on_mouse_entered(&mut self) -> bool {
        self.has_on_mouse_entered
    }

    /// Tells the `Widget` whether or not an `on_mouse_exited` callback has been set.
    pub fn has_on_mouse_exited(&mut self) -> bool {
        self.has_on_mouse_exited
    }

    /// Tells the `Widget` whether or not an `on_mouse_moved` callback has been set.
    pub fn has_on_mouse_moved(&mut self) -> bool {
        self.has_on_mouse_moved
    }

    /// Tells the `Widget` whether or not an `on_mouse_scrolled` callback has been set.
    pub fn has_on_mouse_scrolled(&mut self) -> bool {
        self.has_on_mouse_scrolled
    }

    /// Tells the `Widget` whether or not an `on_mouse_clicked` callback has been set.
    pub fn has_on_mouse_clicked(&mut self) -> bool {
        self.has_on_mouse_clicked
    }
}

pub fn widget_id_for_name(widgets: &[WidgetContainer], name: String) -> usize {
    match widgets.iter().find(|x| x.get_widget_name() == name.clone()) {
        Some(x) => x.get_widget_id() as usize,
        None => 0 as usize,
    }
}
