extern crate pushrod;
extern crate sdl2;

use pushrod::render::callbacks::widget_id_for_name;
use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget::*;
use pushrod::render::widget_config::CONFIG_COLOR_SECONDARY;
use pushrod::widgets::progress_widget::*;
use pushrod::widgets::text_widget::TextWidget;
use pushrod::widgets::timer_widget::*;
use sdl2::pixels::Color;
use std::any::Any;

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
        .window("pushrod-render timer demo", 400, 180)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 180);
    let mut widget1 = ProgressWidget::new(20, 20, 360, 40, 25);

    widget1.set_color(CONFIG_COLOR_SECONDARY, Color::RGB(255, 0, 0));

    let mut widget2 = ProgressWidget::new(20, 70, 360, 40, 50);

    widget2.set_color(CONFIG_COLOR_SECONDARY, Color::RGB(255, 0, 0));

    let mut widget3 = ProgressWidget::new(20, 120, 360, 40, 75);

    widget3.set_color(CONFIG_COLOR_SECONDARY, Color::RGB(255, 0, 0));

    let mut timer = TimerWidget::new(100, true);
    timer.on_timeout(|x, _widgets| {
        let widget1_id = widget_id_for_name(_widgets, String::from("widget1"));
        let widget2_id = widget_id_for_name(_widgets, String::from("widget2"));
        let widget3_id = widget_id_for_name(_widgets, String::from("widget3"));
        let progress1_value: u8 =
            (cast!(_widgets, widget1_id, ProgressWidget).get_progress() + 1) % 100;
        let progress2_value: u8 =
            (cast!(_widgets, widget2_id, ProgressWidget).get_progress() + 1) % 100;
        let progress3_value: u8 =
            (cast!(_widgets, widget3_id, ProgressWidget).get_progress() + 1) % 100;

        cast!(_widgets, widget1_id, ProgressWidget).set_progress(progress1_value);
        cast!(_widgets, widget2_id, ProgressWidget).set_progress(progress2_value);
        cast!(_widgets, widget3_id, ProgressWidget).set_progress(progress3_value);
    });

    engine.add_widget(Box::new(widget1), String::from("widget1"));
    engine.add_widget(Box::new(widget2), String::from("widget2"));
    engine.add_widget(Box::new(widget3), String::from("widget3"));
    engine.add_widget(Box::new(timer), String::from("timer1"));

    engine.run(sdl_context, window);
}
