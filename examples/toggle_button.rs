extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::{CONFIG_BORDER_WIDTH, CONFIG_COLOR_BORDER};
use pushrod::render::{make_points, make_size};
use pushrod::widgets::toggle_button_widget::ToggleButtonWidget;
use sdl2::pixels::Color;

/*
 * This demo just tests the rendering functionality of the `BaseWidget`.  It only tests the
 * render portion of the library, nothing else.
 */

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render toggle button demo", 400, 100)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 100, 60);
    let mut button1 = ToggleButtonWidget::new(
        make_points(20, 20),
        make_size(170, 60),
        String::from("1"),
        40,
        false,
    );

    button1.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button1.on_toggle(|_, _widgets, _layouts, _state| {
        eprintln!("1 Toggled: {}", _state);
    });

    let mut button2 = ToggleButtonWidget::new(
        make_points(210, 20),
        make_size(170, 60),
        String::from("2"),
        40,
        true,
    );

    button2.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button2.on_toggle(|_, _widgets, _layouts, _state| {
        eprintln!("2 Toggled: {}", _state);
    });

    engine.add_widget(Box::new(button1), String::from("button1"));
    engine.add_widget(Box::new(button2), String::from("button2"));

    engine.run(sdl_context, window);
}
