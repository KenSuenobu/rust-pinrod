extern crate pushrod;
extern crate sdl2;

use pushrod::layouts::horizontal_layout::HorizontalLayout;
use pushrod::layouts::vertical_layout::VerticalLayout;
use pushrod::render::callbacks::widget_id_for_name;
use pushrod::render::engine::Engine;
use pushrod::render::layout::{Layout, LayoutPosition};
use pushrod::render::widget::{BaseWidget, Widget};
use pushrod::render::widget_cache::WidgetContainer;
use pushrod::render::widget_config::{
    PaddingConstraint, CONFIG_BORDER_WIDTH, CONFIG_COLOR_BORDER, CONFIG_COLOR_TEXT,
};
use pushrod::widgets::push_button_widget::PushButtonWidget;
use pushrod::widgets::text_widget::{TextJustify, TextWidget};
use sdl2::pixels::Color;

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
        .window("pushrod-render horizontal layout demo", 400, 300)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(400, 300, 60);
    let mut layout = HorizontalLayout::new(20, 20, 360, 80, PaddingConstraint::new(0, 0, 0, 0, 1));
    let mut layout2 =
        VerticalLayout::new(250, 120, 130, 160, PaddingConstraint::new(0, 0, 0, 0, 1));
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

    let mut widget3 = BaseWidget::new(0, 0, 0, 0);

    widget3
        .get_config()
        .set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    widget3.get_config().set_numeric(CONFIG_BORDER_WIDTH, 2);

    let mut widget4 = BaseWidget::new(0, 0, 0, 0);

    widget4
        .get_config()
        .set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    widget4.get_config().set_numeric(CONFIG_BORDER_WIDTH, 2);

    let widget1_id = engine.add_widget(Box::new(widget1), String::from("widget1"));
    let widget2_id = engine.add_widget(Box::new(widget2), String::from("widget2"));
    let widget3_id = engine.add_widget(Box::new(widget3), String::from("widget3"));
    let widget4_id = engine.add_widget(Box::new(widget4), String::from("widget4"));

    layout.add_widget(widget1_id, LayoutPosition::new(0, 0));
    layout.add_widget(widget2_id, LayoutPosition::new(1, 0));
    layout2.add_widget(widget3_id, LayoutPosition::new(0, 0));
    layout2.add_widget(widget4_id, LayoutPosition::new(0, 1));
    engine.add_layout(Box::new(layout));
    engine.add_layout(Box::new(layout2));

    let mut text_widget1 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Right,
        String::from("Spacing:"),
        20,
        116,
        70,
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
        100,
        116,
        40,
        22,
    );

    text_widget2
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 255));

    let mut button1 = PushButtonWidget::new(130, 112, 50, 30, String::from("<"), 20);

    button1.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button1.on_click(|_, _widgets, _layouts| {
        let mut spacing = _layouts[0].layout.borrow_mut().get_padding().spacing - 1;
        let top = _layouts[0].layout.borrow_mut().get_padding().top;
        let bottom = _layouts[0].layout.borrow_mut().get_padding().bottom;
        let left = _layouts[0].layout.borrow_mut().get_padding().left;
        let right = _layouts[0].layout.borrow_mut().get_padding().right;
        let text_widget2_id = widget_id_for_name(_widgets, String::from("text_widget2"));

        if spacing <= 0 {
            spacing = 0;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget2_id, TextWidget).set_text(format!("{}", spacing));
    });

    let mut button2 = PushButtonWidget::new(180, 112, 50, 30, String::from(">"), 20);

    button2.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button2.on_click(|_, _widgets, _layouts| {
        let mut spacing = _layouts[0].layout.borrow_mut().get_padding().spacing + 1;
        let top = _layouts[0].layout.borrow_mut().get_padding().top;
        let bottom = _layouts[0].layout.borrow_mut().get_padding().bottom;
        let left = _layouts[0].layout.borrow_mut().get_padding().left;
        let right = _layouts[0].layout.borrow_mut().get_padding().right;
        let text_widget2_id = widget_id_for_name(_widgets, String::from("text_widget2"));

        if spacing >= MAX_SPACING {
            spacing = MAX_SPACING;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget2_id, TextWidget).set_text(format!("{}", spacing));
    });

    let mut text_widget3 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Right,
        String::from("Top:"),
        20,
        146,
        70,
        22,
    );

    text_widget3
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    let mut text_widget4 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Left,
        String::from("0"),
        100,
        146,
        40,
        22,
    );

    text_widget4
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 255));

    let mut button3 = PushButtonWidget::new(130, 142, 50, 30, String::from("<"), 20);

    button3.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button3.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button3.on_click(|_, _widgets, _layouts| {
        let spacing = _layouts[0].layout.borrow_mut().get_padding().spacing;
        let mut top = _layouts[0].layout.borrow_mut().get_padding().top - 1;
        let bottom = _layouts[0].layout.borrow_mut().get_padding().bottom;
        let left = _layouts[0].layout.borrow_mut().get_padding().left;
        let right = _layouts[0].layout.borrow_mut().get_padding().right;
        let text_widget4_id = widget_id_for_name(_widgets, String::from("text_widget4"));

        if top <= 0 {
            top = 0;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget4_id, TextWidget).set_text(format!("{}", top));
    });

    let mut button4 = PushButtonWidget::new(180, 142, 50, 30, String::from(">"), 20);

    button4.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button4.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button4.on_click(|_, _widgets, _layouts| {
        let spacing = _layouts[0].layout.borrow_mut().get_padding().spacing;
        let mut top = _layouts[0].layout.borrow_mut().get_padding().top + 1;
        let bottom = _layouts[0].layout.borrow_mut().get_padding().bottom;
        let left = _layouts[0].layout.borrow_mut().get_padding().left;
        let right = _layouts[0].layout.borrow_mut().get_padding().right;
        let text_widget4_id = widget_id_for_name(_widgets, String::from("text_widget4"));

        if top >= MAX_SPACING {
            top = MAX_SPACING;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget4_id, TextWidget).set_text(format!("{}", top));
    });

    let mut text_widget5 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Right,
        String::from("Bottom:"),
        20,
        176,
        70,
        22,
    );

    text_widget5
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    let mut text_widget6 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Left,
        String::from("0"),
        100,
        176,
        40,
        22,
    );

    text_widget6
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 255));

    let mut button5 = PushButtonWidget::new(130, 172, 50, 30, String::from("<"), 20);

    button5.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button5.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button5.on_click(|_, _widgets, _layouts| {
        let spacing = _layouts[0].layout.borrow_mut().get_padding().spacing;
        let top = _layouts[0].layout.borrow_mut().get_padding().top;
        let mut bottom = _layouts[0].layout.borrow_mut().get_padding().bottom - 1;
        let left = _layouts[0].layout.borrow_mut().get_padding().left;
        let right = _layouts[0].layout.borrow_mut().get_padding().right;
        let text_widget6_id = widget_id_for_name(_widgets, String::from("text_widget6"));

        if bottom <= 0 {
            bottom = 0;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget6_id, TextWidget).set_text(format!("{}", bottom));
    });

    let mut button6 = PushButtonWidget::new(180, 172, 50, 30, String::from(">"), 20);

    button6.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button6.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button6.on_click(|_, _widgets, _layouts| {
        let spacing = _layouts[0].layout.borrow_mut().get_padding().spacing;
        let top = _layouts[0].layout.borrow_mut().get_padding().top;
        let mut bottom = _layouts[0].layout.borrow_mut().get_padding().bottom + 1;
        let left = _layouts[0].layout.borrow_mut().get_padding().left;
        let right = _layouts[0].layout.borrow_mut().get_padding().right;
        let text_widget6_id = widget_id_for_name(_widgets, String::from("text_widget6"));

        if bottom >= MAX_SPACING {
            bottom = MAX_SPACING;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget6_id, TextWidget).set_text(format!("{}", bottom));
    });

    let mut text_widget7 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Right,
        String::from("Left:"),
        20,
        206,
        70,
        22,
    );

    text_widget7
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    let mut text_widget8 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Left,
        String::from("0"),
        100,
        206,
        40,
        22,
    );

    text_widget8
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 255));

    let mut button7 = PushButtonWidget::new(130, 202, 50, 30, String::from("<"), 20);

    button7.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button7.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button7.on_click(|_, _widgets, _layouts| {
        let spacing = _layouts[0].layout.borrow_mut().get_padding().spacing;
        let top = _layouts[0].layout.borrow_mut().get_padding().top;
        let bottom = _layouts[0].layout.borrow_mut().get_padding().bottom;
        let mut left = _layouts[0].layout.borrow_mut().get_padding().left - 1;
        let right = _layouts[0].layout.borrow_mut().get_padding().right;
        let text_widget8_id = widget_id_for_name(_widgets, String::from("text_widget8"));

        if left <= 0 {
            left = 0;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget8_id, TextWidget).set_text(format!("{}", left));
    });

    let mut button8 = PushButtonWidget::new(180, 202, 50, 30, String::from(">"), 20);

    button8.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button8.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button8.on_click(|_, _widgets, _layouts| {
        let spacing = _layouts[0].layout.borrow_mut().get_padding().spacing;
        let top = _layouts[0].layout.borrow_mut().get_padding().top;
        let bottom = _layouts[0].layout.borrow_mut().get_padding().bottom;
        let mut left = _layouts[0].layout.borrow_mut().get_padding().left + 1;
        let right = _layouts[0].layout.borrow_mut().get_padding().right;
        let text_widget8_id = widget_id_for_name(_widgets, String::from("text_widget8"));

        if left >= MAX_SPACING {
            left = MAX_SPACING;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget8_id, TextWidget).set_text(format!("{}", left));
    });

    let mut text_widget9 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Right,
        String::from("Right:"),
        20,
        236,
        70,
        22,
    );

    text_widget9
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 0));

    let mut text_widget10 = TextWidget::new(
        String::from("assets/OpenSans-Regular.ttf"),
        sdl2::ttf::FontStyle::NORMAL,
        16,
        TextJustify::Left,
        String::from("0"),
        100,
        236,
        40,
        22,
    );

    text_widget10
        .get_config()
        .set_color(CONFIG_COLOR_TEXT, Color::RGB(0, 0, 255));

    let mut button9 = PushButtonWidget::new(130, 232, 50, 30, String::from("<"), 20);

    button9.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button9.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button9.on_click(|_, _widgets, _layouts| {
        let spacing = _layouts[0].layout.borrow_mut().get_padding().spacing;
        let top = _layouts[0].layout.borrow_mut().get_padding().top;
        let bottom = _layouts[0].layout.borrow_mut().get_padding().bottom;
        let left = _layouts[0].layout.borrow_mut().get_padding().left;
        let mut right = _layouts[0].layout.borrow_mut().get_padding().right - 1;
        let text_widget10_id = widget_id_for_name(_widgets, String::from("text_widget10"));

        if right <= 0 {
            right = 0;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget10_id, TextWidget).set_text(format!("{}", right));
    });

    let mut button10 = PushButtonWidget::new(180, 232, 50, 30, String::from(">"), 20);

    button10.set_color(CONFIG_COLOR_BORDER, Color::RGB(0, 0, 0));
    button10.set_numeric(CONFIG_BORDER_WIDTH, 2);
    button10.on_click(|_, _widgets, _layouts| {
        let spacing = _layouts[0].layout.borrow_mut().get_padding().spacing;
        let top = _layouts[0].layout.borrow_mut().get_padding().top;
        let bottom = _layouts[0].layout.borrow_mut().get_padding().bottom;
        let left = _layouts[0].layout.borrow_mut().get_padding().left;
        let mut right = _layouts[0].layout.borrow_mut().get_padding().right + 1;
        let text_widget10_id = widget_id_for_name(_widgets, String::from("text_widget10"));

        if right >= MAX_SPACING {
            right = MAX_SPACING;
        }

        let spacing_new = PaddingConstraint::new(top, bottom, left, right, spacing);

        _layouts[0]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());
        _layouts[1]
            .layout
            .borrow_mut()
            .set_padding(spacing_new.clone());

        refresh_widgets(_widgets);

        cast!(_widgets, text_widget10_id, TextWidget).set_text(format!("{}", right));
    });

    engine.add_widget(Box::new(text_widget1), String::from("text_widget1"));
    engine.add_widget(Box::new(text_widget2), String::from("text_widget2"));
    engine.add_widget(Box::new(button1), String::from("button1"));
    engine.add_widget(Box::new(button2), String::from("button2"));
    engine.add_widget(Box::new(text_widget3), String::from("text_widget3"));
    engine.add_widget(Box::new(text_widget4), String::from("text_widget4"));
    engine.add_widget(Box::new(button3), String::from("button3"));
    engine.add_widget(Box::new(button4), String::from("button4"));
    engine.add_widget(Box::new(text_widget5), String::from("text_widget5"));
    engine.add_widget(Box::new(text_widget6), String::from("text_widget6"));
    engine.add_widget(Box::new(button5), String::from("button5"));
    engine.add_widget(Box::new(button6), String::from("button6"));
    engine.add_widget(Box::new(text_widget7), String::from("text_widget7"));
    engine.add_widget(Box::new(text_widget8), String::from("text_widget8"));
    engine.add_widget(Box::new(button7), String::from("button7"));
    engine.add_widget(Box::new(button8), String::from("button8"));
    engine.add_widget(Box::new(text_widget9), String::from("text_widget9"));
    engine.add_widget(Box::new(text_widget10), String::from("text_widget10"));
    engine.add_widget(Box::new(button9), String::from("button9"));
    engine.add_widget(Box::new(button10), String::from("button10"));

    engine.run(sdl_context, window);
}

fn refresh_widgets(_widgets: &[WidgetContainer]) {
    _widgets[0]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[5]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[6]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[9]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[10]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[13]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[14]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[17]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[18]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[21]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
    _widgets[22]
        .widget
        .borrow_mut()
        .get_config()
        .set_invalidated(true);
}
