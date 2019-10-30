// Window Container
// Contains a PistonWindow and a list of widgets
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

use gl::types::GLuint;
use graphics::*;
use opengl_graphics::GlGraphics;
use std::cell::RefCell;

use crate::core::layout_manager::*;
use crate::core::point::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// This is a container object that stores a `Widget`, assigns a name, a `Widget` ID, and its
/// parent.
pub struct WidgetContainer {
    /// The `Widget` being stored.
    pub widget: RefCell<Box<dyn Widget>>,

    /// The name of the `Widget`.
    pub widget_name: String,

    /// The `Widget`'s automatically assigned ID.
    pub widget_id: i32,

    /// The `Widget`'s parent ID.
    pub parent_id: i32,
}

pub struct LayoutManagerContainer {
    pub container_id: i32,
    pub widget_ids: RefCell<Vec<i32>>,
    pub widget_positions: RefCell<Vec<Point>>,
    pub layout_manager: RefCell<Box<dyn LayoutManager>>,
}

/// This is the `WidgetStore`, which contains a list of `Widget` objects for a GUI window.
pub struct WidgetStore {
    pub widgets: Vec<WidgetContainer>,
    pub layout_managers: Vec<LayoutManagerContainer>,
}

impl WidgetStore {
    /// Constructor, creates a new `WidgetStore`, assigning a top-level `CanvasWidget` as the
    /// very top-level widget.  All `Widget` objects added will be a parent to this `Widget`,
    /// which is stored at ID 0.  If this `Widget` object ever becomes invalidated, the entire
    /// window is force refreshed.
    pub fn new() -> Self {
        let mut widgets_list: Vec<WidgetContainer> = Vec::new();
        let mut base_widget = CanvasWidget::new();

        base_widget.set_size(CONFIG_BODY_SIZE, 800, 600);
        widgets_list.push(WidgetContainer {
            widget: RefCell::new(Box::new(base_widget)),
            widget_name: String::from("_WidgetStoreBase"),
            widget_id: 0,
            parent_id: 0,
        });

        Self {
            widgets: widgets_list,
            layout_managers: Vec::new(),
        }
    }

    /// Invalidates all `Widget`s in the GUI stack, forcing a redraw.
    pub fn invalidate_all_widgets(&mut self) {
        self.widgets
            .iter_mut()
            .for_each(|x| x.widget.borrow_mut().invalidate());
    }

    /// Indicates whether or not a widget in the store has been invalidated.
    pub fn needs_repaint(&mut self) -> bool {
        self.widgets
            .iter_mut()
            .map(|x| x.widget.borrow_mut().is_invalidated())
            .find(|x| x == &true)
            .unwrap_or(false)
    }

    /// Adds a `Widget` to the stack by name.
    pub fn add_widget(&mut self, name: &str, mut widget: Box<dyn Widget>) -> i32 {
        let widget_size = self.widgets.len() as i32;

        // #117 - assigns widget ID to itself
        widget.set_widget_id(widget_size);

        let container = WidgetContainer {
            widget: RefCell::new(widget),
            widget_name: String::from(name),
            widget_id: widget_size,
            parent_id: 0,
        };

        self.widgets.push(container);

        widget_size
    }

    fn set_parent_for_widget(&mut self, widget_id: i32, parent_id: i32) {
        let mut container = self.widgets.get_mut(widget_id as usize).unwrap();

        container.parent_id = parent_id;
    }

    /// Adds a `Widget` object to the parent specified by ID.
    pub fn add_widget_to_parent(
        &mut self,
        name: &str,
        widget: Box<dyn Widget>,
        parent_id: i32,
    ) -> i32 {
        let widget_id = self.add_widget(name, widget);

        self.set_parent_for_widget(widget_id, parent_id);

        widget_id
    }

    fn get_layout_manager_widget_id(&mut self, manager_id: i32) -> i32 {
        let layout_container = self.layout_managers[manager_id as usize]
            .layout_manager
            .borrow_mut();

        layout_container.get_widget_id()
    }

    pub fn add_widget_to_layout_manager(
        &mut self,
        name: &str,
        widget: Box<dyn Widget>,
        manager_id: i32,
        position: Point,
    ) -> i32 {
        let widget_id = self.add_widget(name, widget);

        eprintln!("[Add widget to layout manager] widget_id={}", widget_id);

        let layout_widget_id = self.get_layout_manager_widget_id(manager_id);

        self.set_parent_for_widget(widget_id, layout_widget_id);

        eprintln!(
            "[Add widget to layout manager] layout_widget_id={}",
            layout_widget_id
        );

        // Add to the widget_store with the widget_id and position
        let layout_container = &self.layout_managers[manager_id as usize];

        layout_container.widget_ids.borrow_mut().push(widget_id);
        layout_container
            .widget_positions
            .borrow_mut()
            .push(position);

        widget_id
    }

    /// Gets the parent of the child `Widget` by ID.  If the child has no assigned parent, the
    /// top-level `CanvasWidget` is returned (ID 0).
    pub fn get_parent_of(&mut self, widget_id: i32) -> i32 {
        if widget_id <= 0 {
            0
        } else {
            self.widgets[widget_id as usize].parent_id
        }
    }

    /// Returns a list of the children that are owned by a parent ID.  Does not return a list of
    /// siblings, only the first-level children.
    pub fn get_children_of(&self, parent_id: i32) -> Vec<i32> {
        self.widgets
            .iter()
            .filter(|x| x.parent_id == parent_id)
            .map(|x| x.widget_id)
            .collect()
    }

    /// Gets a `Widget` by ID for a point in the screen.  If the GUI object is hidden or disabled,
    /// the ID is not returned.  If no widget is found under the point specified, an ID of -1 is
    /// returned.
    pub fn get_widget_id_for_point(&mut self, point: Point) -> i32 {
        let mut found_id = -1;

        for (pos, obj) in self.widgets.iter_mut().enumerate() {
            let hidden = *&obj
                .widget
                .borrow_mut()
                .config()
                .get_toggle(CONFIG_WIDGET_HIDDEN);
            let disabled = *&obj
                .widget
                .borrow_mut()
                .config()
                .get_toggle(CONFIG_WIDGET_DISABLED);

            if !hidden && !disabled {
                let widget_point = &obj.widget.borrow_mut().config().get_point(CONFIG_ORIGIN);
                let widget_size: crate::core::point::Size =
                    obj.widget.borrow_mut().config().get_size(CONFIG_BODY_SIZE);

                // Skip over item widgets that have a width and height of 0.
                if widget_size.w > 0 && widget_size.h > 0 {
                    if point[0] >= widget_point[0]
                        && point[0] <= widget_point[0] + widget_size.w
                        && point[1] >= widget_point[1]
                        && point[1] <= widget_point[1] + widget_size.h
                    {
                        found_id = pos as i32;
                    }
                }
            }
        }

        found_id
    }

    /// Returns the name of the `Widget` by specified ID.
    pub fn get_name_for_widget_id(&mut self, widget_id: i32) -> &str {
        self.widgets[widget_id as usize].widget_name.as_str()
    }

    /// Retrieves a widget ID for the name specified.  Returns top-level `CanvasWidget` ID if
    /// not found.
    pub fn get_widget_id_for_name(&mut self, name: &str) -> i32 {
        match self
            .widgets
            .iter_mut()
            .find(|x| x.widget_name == String::from(name))
        {
            Some(x) => x.widget_id,
            None => 0,
        }
    }

    /// Retrieves a reference to a `Widget` by its name.  Returns the top-level `CanvasWidget`
    /// object if not found.
    pub fn get_widget_for_name(&mut self, name: &str) -> &RefCell<Box<dyn Widget>> {
        let widget_id = match self
            .widgets
            .iter_mut()
            .find(|x| x.widget_name == String::from(name))
        {
            Some(x) => x.widget_id,
            None => 0,
        };

        self.get_widget_for_id(widget_id)
    }

    /// Retrieves a `Widget` by its ID.
    pub fn get_widget_for_id(&mut self, id: i32) -> &RefCell<Box<dyn Widget>> {
        &self.widgets[id as usize].widget
    }

    // --- Layout Manager Routines ---

    pub fn add_layout_manager(&mut self, manager: Box<dyn LayoutManager>) -> i32 {
        let managers_size = self.layout_managers.len() as i32;

        self.layout_managers.push(LayoutManagerContainer {
            container_id: managers_size,
            widget_ids: RefCell::new(Vec::new()),
            widget_positions: RefCell::new(Vec::new()),
            layout_manager: RefCell::new(manager),
        });

        managers_size
    }

    fn get_widget_origins(&mut self, manager_id: i32) -> Vec<Point> {
        let mut widget_ids_copy = self.layout_managers[manager_id as usize]
            .widget_ids
            .borrow_mut()
            .clone();

        widget_ids_copy
            .iter_mut()
            .map(|x| {
                self.get_widget_for_id(*x)
                    .borrow_mut()
                    .config()
                    .get_point(CONFIG_ORIGIN)
            })
            .collect()
    }

    fn get_widget_sizes(&mut self, manager_id: i32) -> Vec<Size> {
        let mut widget_ids_copy = self.layout_managers[manager_id as usize]
            .widget_ids
            .borrow_mut()
            .clone();

        widget_ids_copy
            .iter_mut()
            .map(|x| {
                self.get_widget_for_id(*x)
                    .borrow_mut()
                    .config()
                    .get_size(CONFIG_BODY_SIZE)
            })
            .collect()
    }

    pub fn do_layout_for_manager(&mut self, manager_id: i32) {
        let widget_origins = self.get_widget_origins(manager_id);
        let widget_sizes = self.get_widget_sizes(manager_id);
        let widget_positions = self.layout_managers[manager_id as usize]
            .widget_positions
            .borrow()
            .clone();
        let container_widget_id = self.layout_managers[manager_id as usize]
            .layout_manager
            .borrow_mut()
            .get_widget_id();
        let master_container_origin = self
            .get_widget_for_id(container_widget_id)
            .borrow_mut()
            .config()
            .get_point(CONFIG_ORIGIN);
        let master_container_size = self
            .get_widget_for_id(container_widget_id)
            .borrow_mut()
            .config()
            .get_size(CONFIG_BODY_SIZE);
        let adjusted_sizes = self.layout_managers[manager_id as usize]
            .layout_manager
            .borrow_mut()
            .do_layout(
                master_container_origin,
                master_container_size,
                LayoutManagerCoordinates {
                    widget_origins,
                    widget_sizes,
                    widget_positions,
                },
            );

        self.layout_managers[manager_id as usize].widget_positions =
            RefCell::new(adjusted_sizes.widget_positions.clone());

        let num_widgets = adjusted_sizes.widget_positions.len();

        for x in 0..num_widgets {
            let widget_id = self.layout_managers[manager_id as usize]
                .widget_ids
                .borrow_mut()[x as usize];
            let point: Point = adjusted_sizes.widget_origins[x].clone();
            let size: Size = adjusted_sizes.widget_sizes[x].clone();
            let mut widget = self.get_widget_for_id(widget_id).borrow_mut();

            eprintln!("Resizing widget: id={}", widget.get_widget_id());

            widget.set_point(CONFIG_ORIGIN, point[0], point[1]);
            widget.set_size(CONFIG_BODY_SIZE, size.w, size.h);
        }

        eprintln!("Doing manager layout.");
    }

    pub fn adjust_layout_manager(&mut self, manager_id: i32, coordinates: LayoutManagerPadding) {
        self.layout_managers[manager_id as usize]
            .layout_manager
            .borrow_mut()
            .adjust_layout(coordinates.clone());
        self.do_layout_for_manager(manager_id);
    }

    pub fn resize_layout_managers(&mut self, _w: u32, _h: u32) {
        let num_layout_managers = self.layout_managers.len();

        for pos in 0..num_layout_managers {
            let _layout_manager = self.layout_managers[pos as usize]
                .layout_manager
                .borrow_mut();

            eprintln!("WARNING: Unimplemented resize_layout_managers");

            //            let widget_ids = self.layout_managers[pos as usize].widget_ids.clone();
            //            let widget_positions = self.layout_managers[pos as usize].widget_positions.clone();
            //
            //            layout_manager.resize(
            //                Size {
            //                    w: w as i32,
            //                    h: h as i32,
            //                },
            //                widget_ids.borrow().clone(),
            //                widget_positions.borrow().clone(),
            //                &self.widgets,
            //            );
        }
    }

    // -- Display-related routines --

    /// Sets the hidden toggle for a parent, and all of its children.
    pub fn set_hidden(&mut self, widget_id: i32, state: bool) {
        if widget_id != 0 {
            let children = self.get_children_of(widget_id);

            children.iter().for_each(|w_id| {
                if *w_id != 0 && *w_id != widget_id {
                    &self.widgets[*w_id as usize]
                        .widget
                        .borrow_mut()
                        .set_toggle(CONFIG_WIDGET_HIDDEN, state);
                    self.set_hidden(*w_id, state);
                }
            });

            &self.widgets[widget_id as usize]
                .widget
                .borrow_mut()
                .set_toggle(CONFIG_WIDGET_HIDDEN, state);
        }
    }

    /// Draws a `Widget` by ID, and any children contained in that `Widget`.  Submitting a draw
    /// request from ID 0 will redraw the entire screen.
    pub fn draw(&mut self, widget_id: i32, c: Context, g: &mut GlGraphics, original_fbo: GLuint) {
        let parents_of_widget = self.get_children_of(widget_id);

        if parents_of_widget.len() == 0 {
            return;
        }

        for pos in 0..parents_of_widget.len() {
            c.reset();

            let paint_id = parents_of_widget[pos];
            let paint_widget = &mut self.widgets[paint_id as usize];

            if paint_widget.widget.borrow_mut().is_drawable() {
                let is_hidden = paint_widget
                    .widget
                    .borrow_mut()
                    .config()
                    .get_toggle(CONFIG_WIDGET_HIDDEN);

                // If the object is hidden, do not traverse objects underneath the hidden object, as
                // it stops drawing any children at that point.
                if !is_hidden {
                    let is_invalidated = paint_widget.widget.borrow_mut().is_invalidated();

                    if is_invalidated {
                        let origin: Point = paint_widget
                            .widget
                            .borrow_mut()
                            .config()
                            .get_point(CONFIG_ORIGIN);

                        let new_context: Context = Context {
                            viewport: c.viewport,
                            view: c.view,
                            transform: c.transform.trans(origin[0] as f64, origin[1] as f64),
                            draw_state: c.draw_state,
                        };

                        &paint_widget.widget.borrow_mut().get_drawable().draw(
                            new_context,
                            g,
                            &c.draw_state,
                        );

                        if paint_widget
                            .widget
                            .borrow_mut()
                            .config()
                            .get_toggle(CONFIG_WIDGET_DISABLED)
                        {
                            let size: Size = paint_widget
                                .widget
                                .borrow_mut()
                                .config()
                                .get_size(CONFIG_BODY_SIZE);

                            g.rectangle(
                                &Rectangle::new([0.0, 0.0, 0.0, 0.8]),
                                [
                                    origin[0] as f64,
                                    origin[1] as f64,
                                    size.w as f64,
                                    size.h as f64,
                                ],
                                &c.draw_state,
                                c.transform,
                            );
                        }
                    }

                    if parents_of_widget[pos] != widget_id {
                        self.draw(paint_id, c, g, original_fbo);
                    }
                }
            }
        }
    }
}
