extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::{
    CONFIG_BORDER_WIDTH, CONFIG_COLOR_BASE, CONFIG_COLOR_BORDER, CONFIG_COLOR_HOVER,
    CONFIG_COLOR_SECONDARY,
};
use pushrod::widgets::list_widget::*;
use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render list demo", 400, 300)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 300, 60);
    let mut widget1 = ListWidget::new(20, 20, 200, 260);

    widget1.set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
    widget1.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    widget1.set_color(CONFIG_COLOR_HOVER, Color::RGB(0x90, 0x90, 0xFF));
    widget1.set_numeric(CONFIG_BORDER_WIDTH, 1);

    widget1.add_item(String::from("Item 1"));
    widget1.add_item(String::from("Item 2"));
    widget1.add_item(String::from("Item 3"));
    widget1.add_item(String::from("Item 4"));
    widget1.add_item(String::from("Item 5"));

    widget1.on_selected(|x, _widgets, _layout, selected_item| {
        eprintln!("Selected: {}", selected_item);
    });

    engine.add_widget(Box::new(widget1), String::from("widget1"));

    engine.run(sdl_context, window);
}
