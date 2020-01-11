// Pushrod Rendering Library
// Layout Caching Library
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

use crate::render::layout::Layout;
use crate::render::widget_cache::WidgetContainer;
use std::cell::RefCell;

/// This is a container object that stores a `Layout` object, and its ID.
pub struct LayoutContainer {
    pub layout: RefCell<Box<dyn Layout>>,
    layout_id: i32,
}

/// This is an implementation that allows for creation of a `LayoutContainer`.
impl LayoutContainer {
    /// Creates a new `LayoutContainer`-wrapped `Layout` object.
    pub fn new(layout: Box<dyn Layout>, layout_id: i32) -> Self {
        Self {
            layout: RefCell::new(layout),
            layout_id,
        }
    }

    /// Retrieves the current layout ID.
    pub fn get_layout_id(&self) -> i32 {
        self.layout_id
    }
}

/// This is a container object that stores a `Vec` of `LayoutContainer` objects for its cache.
pub struct LayoutCache {
    cache: Vec<LayoutContainer>,
}

/// This is the implementation of the `LayoutCache`.
impl LayoutCache {
    pub fn new() -> Self {
        Self {
            cache: Vec::new(),
        }
    }

    /// Adds a `Box<Layout>` to the `Layout` stack.
    pub fn add_layout(&mut self, layout: Box<dyn Layout>) -> i32 {
        let layout_id = self.cache.len() as i32;

        self.cache.push(LayoutContainer::new(layout, layout_id));

        layout_id
    }

    /// Retrieves a `&mut LayoutContainer` object by its ID.
    pub fn get_layout_by_id(&mut self, id: i32) -> &mut LayoutContainer {
        &mut self.cache[id as usize]
    }

    /// Retrieves a borrowed slice of the `LayoutContainer` cache that can be sent to callbacks.
    pub fn get_layout_cache(&self) -> &[LayoutContainer] {
        &self.cache
    }

    /// Performs the `do_layout` call on `Layout` objects only if their `needs_layout` flag is set
    /// to `true`.
    pub fn do_layout(&self, widgets: &[WidgetContainer]) {
        for x in &self.cache {
            let needs_layout = x.layout.borrow().needs_layout();

            if needs_layout {
                x.layout.borrow_mut().do_layout(widgets);
            }
        }
    }
}
