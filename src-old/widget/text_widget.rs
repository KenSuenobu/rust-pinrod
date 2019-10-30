// Text Widget
// Draws text in a specified bounding area.
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

//extern crate graphics;

use graphics;
use graphics::character::CharacterCache;
use graphics::draw_state::DrawState;
use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::input::*;

use crate::core::callbacks::*;
use crate::core::widget_store::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// Enumeration identifying the justification of the text to be drawn, as long as the bounds
/// of the object allow for it.
pub enum TextJustify {
    /// Left-justified text.
    Left,

    /// Center-justified text: `(total width - text width) / 2`
    Center,

    /// Right-justified text: `(total width - text width)`
    Right,
}

/// Draws a block of text.
pub struct TextWidget {
    config: Configurable,
    font_cache: GlyphCache<'static>, // YUCK - I do not like this!
    font_size: u32,
    justify: TextJustify,
    pub desired_width: i32,
    need_text_resize: bool,
    widget_id: i32,
    callbacks: DefaultWidgetCallbacks,
}

impl TextWidget {
    /// Constructor.  Requires the name of the font, the text to display, the size of the font,
    /// and the font justification when rendered.
    pub fn new(font_name: String, text: String, font_size: u32, justify: TextJustify) -> Self {
        let mut configurable = Configurable::new();
        let cache = GlyphCache::new(font_name.clone(), (), TextureSettings::new()).unwrap();

        configurable.set(CONFIG_DISPLAY_TEXT, Config::Text(text.clone()));

        Self {
            config: configurable,
            font_cache: cache,
            font_size,
            justify,
            desired_width: 0 as i32,
            need_text_resize: true,
            widget_id: 0,
            callbacks: DefaultWidgetCallbacks::new(),
        }
    }

    fn recalculate_desired_size(&mut self) {
        let text = self.config().get_text(CONFIG_DISPLAY_TEXT).clone();
        let mut width = 0.0;

        for ch in text.chars() {
            let character = self.font_cache.character(self.font_size, ch).unwrap();
            width += character.advance_width();
        }

        self.desired_width = width as i32;
        self.need_text_resize = false;
    }

    fn draw_text(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size: crate::core::point::Size = self.config().get_size(CONFIG_BODY_SIZE);

        // Modify transform here based on the width of the text being drawn, which is element 0 of
        // self.desired_size
        let start_x = match self.justify {
            TextJustify::Left => 0.0,
            TextJustify::Center => ((size.w - self.desired_width) / 2) as f64,
            TextJustify::Right => (size.w - self.desired_width) as f64,
        };

        // Vertically justify the text as default.  This isn't a very elegant formula, but it works.
        let start_y = (self.font_size - 2 + size.h as u32) / 2 - 1;

        // And draw the remaining text based on the starting point adjusted by the text justification.
        //
        // IMPORTANT NOTE:
        // The provided transform from the run loop must be modified, as Piston's text drawing
        // routines treats the top "y" value specified as the _baseline_ for the image drawing
        // start point.  We want to treat the _inside_ of the box as the baseline, so we simply
        // add the size of the font (in pixels), which adjusts the baseline to the desired area.
        text::Text::new_color(self.config().get_color(CONFIG_TEXT_COLOR), self.font_size)
            .draw(
                self.config().get_text(CONFIG_DISPLAY_TEXT).as_str(),
                &mut self.font_cache,
                clip,
                c.transform.trans(start_x, start_y as f64),
                g,
            )
            .unwrap();
    }

    inject_event_handler!();
}

impl Drawable for TextWidget {
    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        if self.need_text_resize {
            self.recalculate_desired_size();
        }

        let size = self.config().get_size(CONFIG_BODY_SIZE);

        // Clear the drawing backing
        g.rectangle(
            &Rectangle::new(self.config().get_color(CONFIG_MAIN_COLOR)),
            [0.0f64, 0.0f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
        );

        // Draw the text.
        self.draw_text(c, g, &clip);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}

impl InjectableSystemEvents for TextWidget {}

impl InjectableCustomEvents for TextWidget {}

impl Widget for TextWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());
        self.invalidate();
    }

    fn set_text(&mut self, config: u8, text: String) {
        self.set_config(config, Config::Text(text.clone()));
        self.need_text_resize = true;
        self.invalidate();
    }

    fn set_widget_id(&mut self, widget_id: i32) {
        self.widget_id = widget_id;
    }

    fn get_widget_id(&mut self) -> i32 {
        self.widget_id
    }

    fn handle_event(
        &mut self,
        injected: bool,
        _event: CallbackEvent,
        _widget_store: Option<&Vec<WidgetContainer>>,
    ) -> Option<CallbackEvent> {
        if !injected {
            self.handle_event_callbacks(_event, _widget_store);
        }

        None
    }

    fn handles_events(&mut self) -> bool {
        true
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
