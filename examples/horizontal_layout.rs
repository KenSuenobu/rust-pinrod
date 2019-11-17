extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::{Widget, BaseWidget};
use pushrod::render::widget_config::{CONFIG_COLOR_SECONDARY, PaddingConstraint, CONFIG_COLOR_BORDER, CONFIG_BORDER_WIDTH, CONFIG_COLOR_TEXT, CONFIG_COLOR_BASE};
use pushrod::widgets::progress_widget::*;
use sdl2::pixels::Color;
use pushrod::layouts::horizontal_layout::HorizontalLayout;
use pushrod::render::layout::{Layout, LayoutPosition};
use pushrod::widgets::text_widget::{TextWidget, TextJustify};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render horizontal layout demo", 400, 240)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 240);
    let mut layout = HorizontalLayout::new(20, 20, 360, 80,
    PaddingConstraint::new(0, 0, 0, 0, 1));

    let mut widget1 = BaseWidget::new(0, 0, 0, 0);

    widget1
        .get_config()
        .set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    widget1
        .get_config()
        .set_numeric(CONFIG_BORDER_WIDTH, 2);

    let mut widget2 = BaseWidget::new(0, 0, 0, 0);

    widget2
        .get_config()
        .set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    widget2
        .get_config()
        .set_numeric(CONFIG_BORDER_WIDTH, 2);

    let widget1_id = engine.add_widget(Box::new(widget1), String::from("widget1"));
    let widget2_id = engine.add_widget(Box::new(widget2), String::from("widget2"));

    layout.add_widget(widget1_id, LayoutPosition::new(0, 0));
    layout.add_widget(widget2_id, LayoutPosition::new(1, 0));
    engine.add_layout(Box::new(layout));

    let mut widget1 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Right,
        String::from("Spacing:"),
        20,
        106,
        80,
        22,
    );

    widget1
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    engine.add_widget(Box::new(widget1), String::from("text_widget1"));

    engine.run(sdl_context, window);
}
