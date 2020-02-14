extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::CONFIG_COLOR_TEXT;
use pushrod::render::{make_points, make_size};
use pushrod::widgets::text_widget::*;
use sdl2::pixels::Color;
use pushrod::widgets::viewport_widget::ViewportWidget;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod viewport demo", 500, 500)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(500, 500, 20);
    let mut viewport = ViewportWidget::new(make_points(20, 20),
    make_size(460, 460));

    engine.add_widget(Box::new(viewport), String::from("viewport"));

    engine.run(sdl_context, window);
}
