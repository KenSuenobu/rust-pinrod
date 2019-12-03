extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::widgets::slider_widget::SliderOrientation::SliderHorizontal;
use pushrod::widgets::slider_widget::{SliderOrientation, SliderWidget};

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
    let mut engine = Engine::new(400, 100);
    let mut slider1 = SliderWidget::new(20, 20, 360, 20, 0, 100, 20, SliderHorizontal);

    slider1.on_value_changed(|slider, _widgets, _layouts, pos| {
        eprintln!("Slider moved: {}", pos);
    });

    engine.add_widget(Box::new(slider1), String::from("slider1"));

    engine.run(sdl_context, window);
}
