extern crate pushrod_render;
extern crate sdl2;

use pushrod_render::render::engine::Engine;
use pushrod_render::render::widget::{BaseWidget, Widget};
use pushrod_render::render::widget_config::{COLOR_BASE, COLOR_BORDER};
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
    let mut new_base_widget = BaseWidget::new(100, 100, 600, 400);

    new_base_widget
        .get_config()
        .colors
        .insert(COLOR_BORDER, Color::RGB(0, 0, 0));
    new_base_widget.get_config().border_width = 2;

    new_base_widget
        .get_callbacks()
        .on_mouse_entered(|x, _widgets| {
            x.get_config()
                .colors
                .insert(COLOR_BASE, Color::RGB(255, 0, 0));
            x.get_config().set_invalidate(true);
            _widgets[0]
                .widget
                .borrow_mut()
                .get_config()
                .set_invalidate(true);
            eprintln!("Mouse Entered");
        });

    new_base_widget
        .get_callbacks()
        .on_mouse_exited(|x, _widgets| {
            x.get_config()
                .colors
                .insert(COLOR_BASE, Color::RGB(255, 255, 255));
            x.get_config().set_invalidate(true);
            _widgets[0]
                .widget
                .borrow_mut()
                .get_config()
                .set_invalidate(true);
            eprintln!("Mouse Exited");
        });

    new_base_widget
        .get_callbacks()
        .on_mouse_moved(|_widget, _widgets, points| {
            eprintln!("Mouse Moved: {:?}", points);
        });

    new_base_widget
        .get_callbacks()
        .on_mouse_scrolled(|_widget, _widgets, points| {
            eprintln!("Mouse Scrolled: {:?}", points);
        });

    new_base_widget
        .get_callbacks()
        .on_mouse_clicked(|_widget, _widgets, button, clicks, state| {
            eprintln!(
                "Mouse Clicked: button={} clicks={} state={}",
                button, clicks, state
            );
        });

    engine.setup(800, 600);

    engine.add_widget(Box::new(new_base_widget), "widget1".to_string());

    engine.run(sdl_context, window);
}
