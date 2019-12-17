extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::CONFIG_COLOR_SECONDARY;
use pushrod::render::{make_points, make_size};
use pushrod::widgets::progress_widget::*;
use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render progress demo", 400, 180)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 180, 60);
    let mut widget1 = ProgressWidget::new(make_points(20, 20), make_size(360, 40), 25);

    widget1.set_color(CONFIG_COLOR_SECONDARY, Color::RGB(255, 0, 0));

    let mut widget2 = ProgressWidget::new(make_points(20, 70), make_size(360, 40), 50);

    widget2.set_color(CONFIG_COLOR_SECONDARY, Color::RGB(255, 0, 0));

    let mut widget3 = ProgressWidget::new(make_points(20, 120), make_size(360, 40), 75);

    widget3.set_color(CONFIG_COLOR_SECONDARY, Color::RGB(255, 0, 0));

    engine.add_widget(Box::new(widget1), String::from("widget1"));
    engine.add_widget(Box::new(widget2), String::from("widget2"));
    engine.add_widget(Box::new(widget3), String::from("widget3"));

    engine.run(sdl_context, window);
}
