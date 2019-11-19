extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::{BaseWidget, Widget};
use pushrod::render::widget_config::{CONFIG_BORDER_WIDTH, CONFIG_COLOR_BASE, CONFIG_COLOR_BORDER};
use pushrod::widgets::tab_widget::TabWidget;
use sdl2::pixels::Color;

/*
 * This demo just tests the rendering functionality of the `BaseWidget`.  It only tests the
 * render portion of the library, nothing else.
 */

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render tab demo", 400, 140)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 140);
    let mut tab_widget = TabWidget::new(20, 20, 360, 50);

    //    tab_widget.add_tab(String::from("Tab 1"), 60);
    //    tab_widget.add_tab(String::from("Tab 2"), 60);
    //    tab_widget.add_tab(String::from("Tab 3"), 60);

    engine.add_widget(Box::new(tab_widget), String::from("tab_widget"));

    engine.run(sdl_context, window);
}
