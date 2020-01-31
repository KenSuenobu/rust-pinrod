// Pushrod Rendering Library
// Texture Caching Component
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

use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use std::collections::HashMap;
use std::path::Path;

/// This is the structure for the `TextureCache`.
pub struct TextureCache {
    images: HashMap<String, Texture>,
    ttf_context: Sdl2TtfContext,
}

/// This is a `Texture` cache object that is used by the `WidgetCache`.  This is responsible for loading
/// in images into a cache in memory so that it can be copied multiple times as required by the
/// application.
impl TextureCache {
    /// Creates a new `TextureCache`.
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            ttf_context: sdl2::ttf::init().map_err(|e| e.to_string()).unwrap(),
        }
    }

    /// Retrieves the current Text Rendering context (`Sdl2TtfContext`)
    pub fn get_ttf_context(&self) -> &Sdl2TtfContext {
        &self.ttf_context
    }

    /// Loads an image based on the `image_name`, which is the filename for the image to load.
    /// Returns a reference to the `Texture` that was loaded.
    pub fn get_image(&mut self, c: &mut Canvas<Window>, image_name: String) -> &Texture {
        self.images.entry(image_name.clone()).or_insert({
            c.texture_creator()
                .load_texture(Path::new(&image_name))
                .unwrap()
        })
    }
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}
