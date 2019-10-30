extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::COLOR_TEXT;
use pushrod::widgets::text_widget::*;
use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render demo", 500, 200)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new();
    let mut widget1 = TextWidget::new(
        "assets/OpenSans-Regular.ttf".to_string(),
        sdl2::ttf::FontStyle::NORMAL,
        28,
        TextJustify::Left, "Left Justified".to_string(),
        20,
        16,
        460,
        40,
    );

    widget1
        .get_config()
        .colors
        .insert(COLOR_TEXT, Color::RGB(255, 0, 0));

    let mut widget2  = TextWidget::new(
        "assets/OpenSans-Regular.ttf".to_string(),
        sdl2::ttf::FontStyle::NORMAL,
        28,
        TextJustify::Center, "Center Justified".to_string(),
        20,
        80,
        460,
        40,
    );

    widget2
        .get_config()
        .colors
        .insert(COLOR_TEXT, Color::RGB(0, 255, 0));

    let mut widget3  = TextWidget::new(
        "assets/OpenSans-Regular.ttf".to_string(),
        sdl2::ttf::FontStyle::NORMAL,
        28,
        TextJustify::Right, "Right Justified".to_string(),
        20,
        144,
        460,
        40,
    );

    widget3
        .get_config()
        .colors
        .insert(COLOR_TEXT, Color::RGB(0, 0, 255));

    engine.setup(500, 200);

    engine.add_widget(Box::new(widget1), "widget1".to_string());
    engine.add_widget(Box::new(widget2), "widget2".to_string());
    engine.add_widget(Box::new(widget3), "widget3".to_string());

    engine.run(sdl_context, window);
}
