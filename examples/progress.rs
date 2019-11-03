extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::COLOR_SECONDARY;
use pushrod::widgets::progress_widget::*;
use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render demo", 400, 180)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new();
    let mut widget1 = ProgressWidget::new(20, 20, 360, 40, 25);

    widget1
        .get_config()
        .colors
        .insert(COLOR_SECONDARY, Color::RGB(255, 0, 0));

    let mut widget2 = ProgressWidget::new(20, 70, 360, 40, 50);

    widget2
        .get_config()
        .colors
        .insert(COLOR_SECONDARY, Color::RGB(255, 0, 0));

    let mut widget3 = ProgressWidget::new(20, 120, 360, 40, 75);

    widget3
        .get_config()
        .colors
        .insert(COLOR_SECONDARY, Color::RGB(255, 0, 0));

    engine.setup(500, 180);

    engine.add_widget(Box::new(widget1), String::from("widget1"));
    engine.add_widget(Box::new(widget2), String::from("widget2"));
    engine.add_widget(Box::new(widget3), String::from("widget3"));

    engine.run(sdl_context, window);
}
