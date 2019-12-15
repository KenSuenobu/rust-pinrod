extern crate pushrod;
extern crate sdl2;

use pushrod::render::callbacks::widget_id_for_name;
use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::CONFIG_COLOR_TEXT;
use pushrod::widgets::slider_widget::SliderOrientation::{SliderHorizontal, SliderVertical};
use pushrod::widgets::slider_widget::SliderWidget;
use pushrod::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;

/*
 * This demo just tests the rendering functionality of the `BaseWidget`.  It only tests the
 * render portion of the library, nothing else.
 */

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

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render toggle button demo", 400, 300)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 300, 60);
    let mut slider1 = SliderWidget::new(20, 20, 300, 20, 0, 100, 20, SliderHorizontal);

    slider1.on_value_changed(|_slider, _widgets, _layouts, pos| {
        let text1_id = widget_id_for_name(_widgets, String::from("text1"));

        cast!(_widgets, text1_id, TextWidget).set_text(format!("{}", pos));
    });

    let mut text_widget1 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Left,
        String::from("20"),
        330,
        20,
        50,
        20,
    );

    text_widget1
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    let mut slider2 = SliderWidget::new(20, 50, 300, 20, 20, 80, 40, SliderHorizontal);

    slider2.on_value_changed(|_slider, _widgets, _layouts, pos| {
        let text2_id = widget_id_for_name(_widgets, String::from("text2"));

        cast!(_widgets, text2_id, TextWidget).set_text(format!("{}", pos));
    });

    let mut text_widget2 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Left,
        String::from("40"),
        330,
        50,
        50,
        20,
    );

    text_widget2
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    let mut slider3 = SliderWidget::new(30, 80, 20, 170, 0, 100, 0, SliderVertical);

    slider3.on_value_changed(|_slider, _widgets, _layouts, pos| {
        let text3_id = widget_id_for_name(_widgets, String::from("text3"));

        cast!(_widgets, text3_id, TextWidget).set_text(format!("{}", pos));
    });

    let mut text_widget3 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Center,
        String::from("0"),
        16,
        270,
        50,
        20,
    );

    text_widget3
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    let mut slider4 = SliderWidget::new(60, 80, 20, 170, 20, 80, 40, SliderVertical);

    slider4.on_value_changed(|_slider, _widgets, _layouts, pos| {
        let text4_id = widget_id_for_name(_widgets, String::from("text4"));

        cast!(_widgets, text4_id, TextWidget).set_text(format!("{}", pos));
    });

    let mut text_widget4 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Center,
        String::from("40"),
        56,
        270,
        50,
        20,
    );

    text_widget4
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    engine.add_widget(Box::new(slider1), String::from("slider1"));
    engine.add_widget(Box::new(text_widget1), String::from("text1"));
    engine.add_widget(Box::new(slider2), String::from("slider2"));
    engine.add_widget(Box::new(text_widget2), String::from("text2"));
    engine.add_widget(Box::new(slider3), String::from("slider3"));
    engine.add_widget(Box::new(text_widget3), String::from("text3"));
    engine.add_widget(Box::new(slider4), String::from("slider4"));
    engine.add_widget(Box::new(text_widget4), String::from("text4"));

    engine.run(sdl_context, window);
}
