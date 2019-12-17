extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::{CompassPosition, CONFIG_COLOR_BASE, CONFIG_IMAGE_POSITION};
use pushrod::render::{make_points, make_size};
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
    let mut engine = Engine::new(WIDTH, HEIGHT, 60);
    let mut widget1 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(20, 16),
        make_size(60, 60),
        false,
    );

    widget1.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget1.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::NW);

    let mut widget2 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(90, 16),
        make_size(60, 60),
        false,
    );

    widget2.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget2.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::N);

    let mut widget3 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(160, 16),
        make_size(60, 60),
        false,
    );

    widget3.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget3.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::NE);

    let mut widget4 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(20, 86),
        make_size(60, 60),
        false,
    );

    widget4.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget4.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::W);

    let mut widget5 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(90, 86),
        make_size(60, 60),
        false,
    );

    widget5.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget5.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::Center);

    let mut widget6 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(160, 86),
        make_size(60, 60),
        false,
    );

    widget6.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget6.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::E);

    let mut widget7 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(20, 156),
        make_size(60, 60),
        false,
    );

    widget7.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget7.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::SW);

    let mut widget8 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(90, 156),
        make_size(60, 60),
        false,
    );

    widget8.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget8.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::S);

    let mut widget9 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(160, 156),
        make_size(60, 60),
        false,
    );

    widget9.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget9.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::SE);

    let mut widget10 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(230, 16),
        make_size(80, 80),
        true,
    );

    widget10.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget10.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::NW);

    let mut widget11 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(260, 46),
        make_size(120, 120),
        true,
    );

    widget11.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget11.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::NW);

    let mut widget12 = ImageWidget::new(
        String::from("assets/rust-48x48.jpg"),
        make_points(320, 86),
        make_size(160, 160),
        true,
    );

    widget12.set_color(CONFIG_COLOR_BASE, Color::RGB(0, 0, 0));
    widget12.set_compass(CONFIG_IMAGE_POSITION, CompassPosition::NW);

    engine.add_widget(Box::new(widget1), String::from("widget1"));
    engine.add_widget(Box::new(widget2), String::from("widget2"));
    engine.add_widget(Box::new(widget3), String::from("widget3"));
    engine.add_widget(Box::new(widget4), String::from("widget4"));
    engine.add_widget(Box::new(widget5), String::from("widget5"));
    engine.add_widget(Box::new(widget6), String::from("widget6"));
    engine.add_widget(Box::new(widget7), String::from("widget7"));
    engine.add_widget(Box::new(widget8), String::from("widget8"));
    engine.add_widget(Box::new(widget9), String::from("widget9"));
    engine.add_widget(Box::new(widget10), String::from("widget10"));
    engine.add_widget(Box::new(widget11), String::from("widget11"));
    engine.add_widget(Box::new(widget12), String::from("widget12"));

    engine.run(sdl_context, window);
}
