extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::widgets::image_button_widget::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render image button demo", 400, 180)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 180, 60);
    let widget1 = ImageButtonWidget::new(
        20,
        20,
        360,
        40,
        String::from(" Rust Logo"),
        24,
        String::from("assets/rust-48x48.jpg"),
    );
    let widget2 = ImageButtonWidget::new(
        20,
        70,
        360,
        40,
        String::from(" Unselected Radio Button"),
        24,
        String::from("assets/radio_unselected.png"),
    );
    let widget3 = ImageButtonWidget::new(
        20,
        120,
        360,
        40,
        String::from(" Unchecked Button"),
        24,
        String::from("assets/checkbox_unselected.png"),
    );

    engine.add_widget(Box::new(widget1), String::from("widget1"));
    engine.add_widget(Box::new(widget2), String::from("widget2"));
    engine.add_widget(Box::new(widget3), String::from("widget3"));

    engine.run(sdl_context, window);
}
