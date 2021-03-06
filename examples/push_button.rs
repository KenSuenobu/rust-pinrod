extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::{CONFIG_BORDER_WIDTH, CONFIG_COLOR_BORDER};
use pushrod::render::{make_points, make_size};
use pushrod::widgets::push_button_widget::PushButtonWidget;
use sdl2::pixels::Color;

/*
 * This demo just tests the rendering functionality of the `BaseWidget`.  It only tests the
 * render portion of the library, nothing else.
 */

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render push button demo", 400, 100)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 100, 30);
    let mut button1 = PushButtonWidget::new(
        make_points(20, 20),
        make_size(360, 60),
        String::from("Click me!"),
        40,
    );

    button1.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button1.on_click(|_x, _widgets, _layouts| {
        eprintln!("Click me clicked!");
    });

    engine.add_widget(Box::new(button1), String::from("button1"));

    engine.run(sdl_context, window);
}
