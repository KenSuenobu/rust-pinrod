extern crate pushrod;
extern crate sdl2;

use pushrod::render::callbacks::widget_id_for_name;
use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::{CONFIG_BORDER_WIDTH, CONFIG_COLOR_BORDER, CONFIG_COLOR_TEXT};
use pushrod::widgets::grid_widget::GridWidget;
use pushrod::widgets::push_button_widget::PushButtonWidget;
use pushrod::widgets::slider_widget::SliderOrientation::SliderHorizontal;
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
        .window("pushrod-render grid demo", 400, 340)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 340);
    let mut grid1 = GridWidget::new(20, 20, 360, 280, 10);

    grid1
        .get_config()
        .set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    grid1.get_config().set_numeric(CONFIG_BORDER_WIDTH, 1);

    let mut slider1 = SliderWidget::new(20, 310, 320, 20, 0, 30, 10, SliderHorizontal);

    slider1.on_value_changed(|_slider, _widgets, _layouts, pos| {
        let text1_id = widget_id_for_name(_widgets, String::from("text1"));
        let grid1_id = widget_id_for_name(_widgets, String::from("grid1"));

        cast!(_widgets, text1_id, TextWidget).set_text(format!("{}", pos));
        cast!(_widgets, grid1_id, GridWidget).set_grid_size(pos);
    });

    let mut text_widget1 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Left,
        String::from("10"),
        360,
        310,
        40,
        20,
    );

    text_widget1
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    engine.add_widget(Box::new(grid1), String::from("grid1"));
    engine.add_widget(Box::new(slider1), String::from("slider1"));
    engine.add_widget(Box::new(text_widget1), String::from("text1"));

    engine.run(sdl_context, window);
}
