extern crate pushrod;
extern crate sdl2;

use pushrod::layouts::horizontal_layout::HorizontalLayout;
use pushrod::render::callbacks::widget_id_for_name;
use pushrod::render::engine::Engine;
use pushrod::render::layout::{Layout, LayoutPosition};
use pushrod::render::widget::{BaseWidget, Widget};
use pushrod::render::widget_config::{
    PaddingConstraint, CONFIG_BORDER_WIDTH, CONFIG_COLOR_BASE, CONFIG_COLOR_BORDER,
    CONFIG_COLOR_SECONDARY, CONFIG_COLOR_TEXT,
};
use pushrod::widgets::progress_widget::*;
use pushrod::widgets::push_button_widget::PushButtonWidget;
use pushrod::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;
use std::fmt::Display;

#[macro_export]
macro_rules! cast {
    ($a:expr, $b:expr, $c:ident) => {
        $a[$b]
            .widget
            .borrow_mut()
            .as_any()
            .downcast_mut::<$c>()
            .unwrap()
    };
}

pub const MAX_SPACING: i32 = 20;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render horizontal layout demo", 400, 160)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 160);
    let mut layout = HorizontalLayout::new(20, 20, 360, 80, PaddingConstraint::new(0, 0, 0, 0, 1));

    let mut widget1 = BaseWidget::new(0, 0, 0, 0);

    widget1
        .get_config()
        .set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    widget1.get_config().set_numeric(CONFIG_BORDER_WIDTH, 2);

    let mut widget2 = BaseWidget::new(0, 0, 0, 0);

    widget2
        .get_config()
        .set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    widget2.get_config().set_numeric(CONFIG_BORDER_WIDTH, 2);

    let widget1_id = engine.add_widget(Box::new(widget1), String::from("widget1"));
    let widget2_id = engine.add_widget(Box::new(widget2), String::from("widget2"));

    layout.add_widget(widget1_id, LayoutPosition::new(0, 0));
    layout.add_widget(widget2_id, LayoutPosition::new(1, 0));
    engine.add_layout(Box::new(layout));

    let mut text_widget1 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Right,
        String::from("Spacing:"),
        20,
        116,
        80,
        22,
    );

    text_widget1
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    let mut text_widget2 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Left,
        String::from("1"),
        110,
        116,
        40,
        22,
    );

    text_widget2
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 255));

    let mut button1 = PushButtonWidget::new(150, 112, 50, 30, String::from("<"), 20);

    button1.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button1.on_click(|x, _widgets, _layouts| {
        let mut spacing = _layouts[0].layout.borrow_mut().get_padding().spacing - 1;
        let text_widget2_id = widget_id_for_name(_widgets, String::from("text_widget2"));

        if spacing <= 0 {
            spacing = 0;
        }

        let spacing_new = PaddingConstraint::new(0, 0, 0, 0, spacing);

        _layouts[0].layout.borrow_mut().set_padding(spacing_new);
        _widgets[0]
            .widget
            .borrow_mut()
            .get_config()
            .set_invalidate(true);
        _widgets[3]
            .widget
            .borrow_mut()
            .get_config()
            .set_invalidate(true);
        _widgets[4]
            .widget
            .borrow_mut()
            .get_config()
            .set_invalidate(true);

        cast!(_widgets, text_widget2_id, TextWidget).set_text(format!("{}", spacing));
    });

    let mut button2 = PushButtonWidget::new(200, 112, 50, 30, String::from(">"), 20);

    button2.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button2.on_click(|x, _widgets, _layouts| {
        let mut spacing = _layouts[0].layout.borrow_mut().get_padding().spacing + 1;
        let text_widget2_id = widget_id_for_name(_widgets, String::from("text_widget2"));

        if spacing >= MAX_SPACING {
            spacing = MAX_SPACING;
        }

        let spacing_new = PaddingConstraint::new(0, 0, 0, 0, spacing);

        _layouts[0].layout.borrow_mut().set_padding(spacing_new);
        _widgets[0]
            .widget
            .borrow_mut()
            .get_config()
            .set_invalidate(true);
        _widgets[3]
            .widget
            .borrow_mut()
            .get_config()
            .set_invalidate(true);
        _widgets[4]
            .widget
            .borrow_mut()
            .get_config()
            .set_invalidate(true);

        cast!(_widgets, text_widget2_id, TextWidget).set_text(format!("{}", spacing));
    });

    engine.add_widget(Box::new(text_widget1), String::from("text_widget1"));
    engine.add_widget(Box::new(text_widget2), String::from("text_widget2"));
    engine.add_widget(Box::new(button1), String::from("button1"));
    engine.add_widget(Box::new(button2), String::from("button2"));

    engine.run(sdl_context, window);
}
