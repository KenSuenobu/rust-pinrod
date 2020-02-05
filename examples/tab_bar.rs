extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::{
    CONFIG_BORDER_WIDTH, CONFIG_COLOR_BASE, CONFIG_COLOR_BORDER, CONFIG_COLOR_HOVER,
    CONFIG_COLOR_SECONDARY,
};
use pushrod::render::{make_points, make_size};
use pushrod::widgets::list_widget::*;
use pushrod::widgets::tab_bar_widget::TabBarWidget;
use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render tab bar demo", 400, 300)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 300, 60);
    let mut widget1 = TabBarWidget::new(
        make_points(20, 20),
        make_size(360, 30),
        vec![
            String::from("First"),
            String::from("Second"),
            String::from("Third"),
        ],
    );

    widget1.set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
    widget1.set_color(CONFIG_COLOR_HOVER, Color::RGB(192, 192, 255));
    widget1.on_tab_selected(|_, _, _, selected_tab| {
        eprintln!("Selected tab: {}", selected_tab);
    });

    engine.add_widget(Box::new(widget1), String::from("widget1"));

    engine.run(sdl_context, window);
}
