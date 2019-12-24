// Pushrod Rendering Library
// Canvas Helper Trait
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

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;
use crate::render::widget::Widget;

/// This trait is used in conjunction with `Widget`s or anything else that draws to a `Canvas` object.
/// It provides convenience methods to provide drawing functions common to `Widget`s.  All points and
/// dimensions are relative to the position of the `Widget`, so no translation is necessary.
pub trait CanvasHelper: Widget {

    /// Draws a point in the `Canvas`.
    fn draw_point(&mut self, c: &mut Canvas<Window>, x: i32, y: i32) {
        let point = Point::new(self.get_config().to_x(x), self.get_config().to_y(y));

        c.draw_point(point).unwrap();
    }

}