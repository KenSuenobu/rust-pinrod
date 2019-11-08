extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::{CONFIG_COLOR_SECONDARY, CONFIG_PROGRESS};
use pushrod::widgets::checkbox_widget::*;
use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render image button demo", 400, 180)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new();
    let widget1 = CheckboxWidget::new(20, 20, 360, 30, String::from(" Checkbox Item 1"), 22, false);
    let widget2 = CheckboxWidget::new(20, 70, 360, 30, String::from(" Checked Checkbox"), 22, true);
    let widget3 = CheckboxWidget::new(
        20,
        120,
        360,
        30,
        String::from(" Unchecked Checkbox"),
        22,
        false,
    );

    engine.setup(500, 180);

    engine.add_widget(Box::new(widget1), String::from("widget1"));
    engine.add_widget(Box::new(widget2), String::from("widget2"));
    engine.add_widget(Box::new(widget3), String::from("widget3"));

    engine.run(sdl_context, window);
}
