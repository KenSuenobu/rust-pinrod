// Pushrod Rendering Library
// Texture Store
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

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

/// This is a store used by the `TextureStore`.
pub struct TextureStore {
    store: Option<Texture>,
    width: u32,
    height: u32,
}

/// This is a `TextureStore` that is used by `Widget`s to draw against.  It serves as a GPU-based
/// `Texture` store that can be changed at any time during the drawing loop.
impl TextureStore {
    /// Creates a new `TextureStore` object.
    pub fn new() -> Self {
        TextureStore {
            store: None,
            width: 0,
            height: 0,
        }
    }

    /// Retrieves a `&mut Texture` reference to the stored `Texture` object.
    ///
    /// Example use:
    /// ```rust,no_run
    /// c.with_texture_canvas(texture_store.get_mut_ref(), |texture| {
    ///   texture.set_draw_color(base_color);
    ///   texture.clear();
    ///
    ///   texture.set_draw_color(border_color);
    ///   texture
    ///     .draw_rect(Rect::new(0, 0, 200, 200))
    ///     .unwrap();
    /// })
    ///   .unwrap();
    /// ```
    pub fn get_mut_ref(&mut self) -> &mut Texture {
        self.store.as_mut().unwrap()
    }

    /// Retrieves a `Option<&Texture>` object for the `Texture` object store.  Use this as a shortcut
    /// to the `Widget`'s return values (see `BaseWidget` for reference.)
    pub fn get_optional_ref(&mut self) -> Option<&Texture> {
        Some(self.store.as_ref().unwrap())
    }

    /// This is used to create a new `Texture` object that can be drawn against.  If the `Widget` is
    /// ever redrawn, this function will automatically generate a new `Texture` to draw against, and
    /// destroy the previously stored `Texture`.  If any changes are observed when calling this
    /// function (ie. the width changes, height changes, or the store is lost), it is regenerated.
    pub fn create_or_resize_texture(&mut self, c: &mut Canvas<Window>, width: u32, height: u32) {
        if self.store.is_none() || self.width != width || self.height != height {
            self.width = width;
            self.height = height;

            self.store = Some(c.create_texture_target(None, width, height).unwrap());

            eprintln!("Created texture: size={}x{}", width, height);
        }
    }
}
