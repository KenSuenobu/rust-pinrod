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

pub struct LayoutContainer {
    pub layout: RefCell<Box<dyn Layout>>,
    layout_id: i32,
}

impl LayoutContainer {
    pub fn new(layout: Box<dyn Layout>, layout_id: i32) -> Self {
        Self {
            layout: RefCell::new(layout),
            layout_id,
        }
    }
}

pub struct LayoutCache {
    cache: Vec<LayoutContainer>,
}

impl LayoutCache {
    pub fn new() -> Self {
        Self { cache: Vec::new() }
    }

    pub fn add_layout(&mut self, layout: Box<dyn Layout>) -> i32 {
        let layout_id = self.cache.len() as i32;

        self.cache.push(LayoutContainer::new(layout, layout_id));

        layout_id
    }

    pub fn get_layout_by_id(&mut self, id: i32) -> &mut LayoutContainer {
        &mut self.cache[id as usize]
    }

    pub fn get_layout_cache(&self) -> &[LayoutContainer] {
        &self.cache
    }

    pub fn do_layout(&self, widgets: &[WidgetContainer]) {
        for x in &self.cache {
            let needs_layout = x.layout.borrow().needs_layout();

            if needs_layout {
                x.layout.borrow_mut().do_layout(widgets);
            }
        }
    }
}
