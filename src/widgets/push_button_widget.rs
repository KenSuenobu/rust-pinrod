// Pushrod Widget Library
// Push Button Widget
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
use crate::render::widget_config::{WidgetConfig, CONFIG_COLOR_BASE, CONFIG_SIZE};
use crate::render::Points;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

use std::collections::HashMap;
use std::path::Path;

/// This is the storage object for the `PushButtonWidget`.  It stores the config, properties, callback registry.
pub struct PushButtonWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
}

impl PushButtonWidget {
    pub fn new(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> Self {
        Self {
            config: WidgetConfig::new(x, y, w, h),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
        }
    }
}

/// This is the `Widget` implementation of the `PushButtonWidget`.
impl Widget for PushButtonWidget {
    fn draw(&mut self, c: &mut Canvas<Window>) {
    }

    default_widget_properties!();
    default_widget_callbacks!();
}
