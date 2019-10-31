extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::COLOR_BASE;
use pushrod::widgets::image_widget::*;
use sdl2::pixels::Color;

pub fn main() {
    const WIDTH: u32 = 500;
    const HEIGHT: u32 = 270;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render image demo", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new();
    let mut widget1 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::NW,
        20,
        16,
        60,
        60,
        false,
    );

    widget1
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget2 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::N,
        90,
        16,
        60,
        60,
        false,
    );

    widget2
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget3 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::NE,
        160,
        16,
        60,
        60,
        false,
    );

    widget3
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget4 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::W,
        20,
        86,
        60,
        60,
        false,
    );

    widget4
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget5 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::Center,
        90,
        86,
        60,
        60,
        false,
    );

    widget5
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget6 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::E,
        160,
        86,
        60,
        60,
        false,
    );

    widget6
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget7 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::SW,
        20,
        156,
        60,
        60,
        false,
    );

    widget7
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget8 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::S,
        90,
        156,
        60,
        60,
        false,
    );

    widget8
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget9 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::SE,
        160,
        156,
        60,
        60,
        false,
    );

    widget9
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget10 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::NW,
        230,
        16,
        80,
        80,
        true,
    );

    widget10
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget11 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::NW,
        260,
        46,
        120,
        120,
        true,
    );

    widget11
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    let mut widget12 = ImageWidget::new(
        "assets/rust-48x48.jpg".to_string(),
        ImagePosition::NW,
        320,
        86,
        160,
        160,
        true,
    );

    widget12
        .get_config()
        .colors
        .insert(COLOR_BASE, Color::RGB(0, 0, 0));

    engine.setup(WIDTH, HEIGHT);

    engine.add_widget(Box::new(widget1), "widget1".to_string());
    engine.add_widget(Box::new(widget2), "widget2".to_string());
    engine.add_widget(Box::new(widget3), "widget3".to_string());
    engine.add_widget(Box::new(widget4), "widget4".to_string());
    engine.add_widget(Box::new(widget5), "widget5".to_string());
    engine.add_widget(Box::new(widget6), "widget6".to_string());
    engine.add_widget(Box::new(widget7), "widget7".to_string());
    engine.add_widget(Box::new(widget8), "widget8".to_string());
    engine.add_widget(Box::new(widget9), "widget9".to_string());
    engine.add_widget(Box::new(widget10), "widget10".to_string());
    engine.add_widget(Box::new(widget11), "widget11".to_string());
    engine.add_widget(Box::new(widget12), "widget12".to_string());

    engine.run(sdl_context, window);
}
