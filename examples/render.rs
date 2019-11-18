extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::{BaseWidget, Widget};
use pushrod::render::widget_config::{CONFIG_BORDER_WIDTH, CONFIG_COLOR_BASE, CONFIG_COLOR_BORDER};
use sdl2::pixels::Color;

/*
 * This demo just tests the rendering functionality of the `BaseWidget`.  It only tests the
 * render portion of the library, nothing else.
 */

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(800, 600);
    let mut new_base_widget = BaseWidget::new(100, 100, 600, 400);

    new_base_widget
        .get_config()
        .set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    new_base_widget
        .get_config()
        .set_numeric(CONFIG_BORDER_WIDTH, 2);

    new_base_widget
        .get_callbacks()
        .on_mouse_entered(|x, _widgets, _layouts| {
            x.get_config()
                .set_color(CONFIG_COLOR_BASE, Color::RGB(255, 0, 0));
            x.get_config().set_invalidated(true);
            _widgets[0]
                .widget
                .borrow_mut()
                .get_config()
                .set_invalidated(true);
            eprintln!("Mouse Entered");
        });

    new_base_widget
        .get_callbacks()
        .on_mouse_exited(|x, _widgets, _layouts| {
            x.get_config()
                .set_color(CONFIG_COLOR_BASE, Color::RGB(255, 255, 255));
            x.get_config().set_invalidated(true);
            _widgets[0]
                .widget
                .borrow_mut()
                .get_config()
                .set_invalidated(true);
            eprintln!("Mouse Exited");
        });

    new_base_widget
        .get_callbacks()
        .on_mouse_moved(|_widget, _widgets, _layouts, points| {
            eprintln!("Mouse Moved: {:?}", points);
        });

    new_base_widget
        .get_callbacks()
        .on_mouse_scrolled(|_widget, _widgets, _layouts, points| {
            eprintln!("Mouse Scrolled: {:?}", points);
        });

    new_base_widget.get_callbacks().on_mouse_clicked(
        |_widget, _widgets, _layouts, button, clicks, state| {
            eprintln!(
                "Mouse Clicked: button={} clicks={} state={}",
                button, clicks, state
            );
        },
    );

    engine.add_widget(Box::new(new_base_widget), String::from("widget1"));

    engine.run(sdl_context, window);
}
