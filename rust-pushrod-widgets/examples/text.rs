extern crate pushrod_render;
extern crate sdl2;

use pushrod_render::render::engine::Engine;
use pushrod_render::render::widget::Widget;
use pushrod_render::render::widget_config::COLOR_TEXT;
use pushrod_widgets::widgets::text_widget::*;
use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new();
    let mut new_base_widget = TextWidget::new(
        "assets/OpenSans-Regular.ttf".to_string(),
        sdl2::ttf::FontStyle::NORMAL,
        28,
        TextJustify::Left,
        "Welcome to Pushrod!".to_string(),
        20,
        16,
        400,
        32,
    );

    new_base_widget
        .get_config()
        .colors
        .insert(COLOR_TEXT, Color::RGB(0, 0, 0));

    engine.setup(800, 600);

    engine.add_widget(Box::new(new_base_widget), "widget1".to_string());

    engine.run(sdl_context, window);
}
