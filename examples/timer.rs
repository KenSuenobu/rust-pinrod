extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::{CONFIG_COLOR_SECONDARY, CONFIG_PROGRESS};
use pushrod::widgets::progress_widget::*;
use pushrod::widgets::timer_widget::*;
use sdl2::pixels::Color;
use pushrod::render::callbacks::widget_id_for_name;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render timer demo", 400, 180)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new();
    let mut widget1 = ProgressWidget::new(20, 20, 360, 40);

    widget1.set_color(CONFIG_COLOR_SECONDARY, Color::RGB(255, 0, 0));
    widget1.set_numeric(CONFIG_PROGRESS, 25);

    let mut widget2 = ProgressWidget::new(20, 70, 360, 40);

    widget2.set_color(CONFIG_COLOR_SECONDARY, Color::RGB(255, 0, 0));
    widget2.set_numeric(CONFIG_PROGRESS, 50);

    let mut widget3 = ProgressWidget::new(20, 120, 360, 40);

    widget3.set_color(CONFIG_COLOR_SECONDARY, Color::RGB(255, 0, 0));
    widget3.set_numeric(CONFIG_PROGRESS, 75);

    let mut timer = TimerWidget::new(100, true);
    timer.on_timeout(|x, _widgets| {
        let widget1_id = widget_id_for_name(_widgets, String::from("widget1"));
        let widget2_id = widget_id_for_name(_widgets, String::from("widget2"));
        let widget3_id = widget_id_for_name(_widgets, String::from("widget3"));
        let timer1_pos = (_widgets[widget1_id].widget.borrow_mut().get_numeric(CONFIG_PROGRESS) + 1) % 100;
        let timer2_pos = (_widgets[widget2_id].widget.borrow_mut().get_numeric(CONFIG_PROGRESS) + 1) % 100;
        let timer3_pos = (_widgets[widget3_id].widget.borrow_mut().get_numeric(CONFIG_PROGRESS) + 1) % 100;

        _widgets[widget1_id].widget.borrow_mut().set_numeric(CONFIG_PROGRESS, timer1_pos);
        _widgets[widget2_id].widget.borrow_mut().set_numeric(CONFIG_PROGRESS, timer2_pos);
        _widgets[widget3_id].widget.borrow_mut().set_numeric(CONFIG_PROGRESS, timer3_pos);
        _widgets[widget1_id].widget.borrow_mut().get_config().set_invalidate(true);
        _widgets[widget2_id].widget.borrow_mut().get_config().set_invalidate(true);
        _widgets[widget3_id].widget.borrow_mut().get_config().set_invalidate(true);

    });

    engine.setup(500, 180);

    engine.add_widget(Box::new(widget1), String::from("widget1"));
    engine.add_widget(Box::new(widget2), String::from("widget2"));
    engine.add_widget(Box::new(widget3), String::from("widget3"));
    engine.add_widget(Box::new(timer), String::from("timer1"));

    engine.run(sdl_context, window);
}
