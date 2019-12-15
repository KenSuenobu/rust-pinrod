extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::CONFIG_COLOR_TEXT;
use pushrod::widgets::text_widget::*;
use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod text widget demo", 500, 200)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(500, 200, 20);
    let mut widget1 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        28,
        TextJustify::Left,
        String::from("Left Justified"),
        20,
        16,
        460,
        40,
    );

    widget1
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(255, 0, 0));

    let mut widget2 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        28,
        TextJustify::Center,
        String::from("Center Justified"),
        20,
        80,
        460,
        40,
    );

    widget2
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 255, 0));

    let mut widget3 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        28,
        TextJustify::Right,
        String::from("Right Justified"),
        20,
        144,
        460,
        40,
    );

    widget3
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 255));

    engine.add_widget(Box::new(widget1), String::from("widget1"));
    engine.add_widget(Box::new(widget2), String::from("widget2"));
    engine.add_widget(Box::new(widget3), String::from("widget3"));

    engine.run(sdl_context, window);
}
