// Simple Windowed Example
// Super simplistic test to show off the use of the library in its current state
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate glfw_window;
extern crate pushrod;

use std::cell::RefCell;

use glfw_window::GlfwWindow;
use piston::input::*;
use piston::window::*;

use pushrod::core::callbacks::*;
use pushrod::core::horizontal_layout_manager::*;
use pushrod::core::layout_manager::LayoutManagerPadding;
use pushrod::core::main::*;
use pushrod::core::point::{make_origin_point, Point, Size};
use pushrod::core::vertical_layout_manager::*;
use pushrod::core::widget_store::*;
use pushrod::widget::box_widget::*;
use pushrod::widget::checkbox_widget::*;
use pushrod::widget::config::*;
use pushrod::widget::image_button_widget::*;
use pushrod::widget::progress_widget::*;
use pushrod::widget::push_button_widget::*;
use pushrod::widget::text_widget::*;
use pushrod::widget::timer_widget::*;
use pushrod::widget::toggle_button_widget::*;
use pushrod::widget::widget::*;

pub struct SimpleWindow {
    pushrod: RefCell<Pushrod>,
}

pub struct SimpleWindowEventHandler {
    animated: bool,
    progress: u16,
    red_value: i16,
    green_value: i16,
    blue_value: i16,
    color_direction: i16,
    manipulated_color: i16,
}

impl PushrodCallbackEvents for SimpleWindowEventHandler {
    fn timer_triggered(&mut self, widget_id: i32, widget_store: &mut WidgetStore) {
        match widget_store.get_name_for_widget_id(widget_id) {
            "HelloWorldTimer" => {
                if self.manipulated_color == 1 {
                    if self.color_direction == 1 {
                        if self.red_value == 255 {
                            self.color_direction = -1;
                        }
                    } else {
                        if self.red_value == 0 {
                            self.color_direction = 1;
                            self.manipulated_color = 2;
                        }
                    }

                    self.red_value += self.color_direction;
                } else if self.manipulated_color == 2 {
                    if self.color_direction == 1 {
                        if self.green_value == 255 {
                            self.color_direction = -1;
                        }
                    } else {
                        if self.green_value == 0 {
                            self.color_direction = 1;
                            self.manipulated_color = 3;
                        }
                    }

                    self.green_value += self.color_direction;
                } else if self.manipulated_color == 3 {
                    if self.color_direction == 1 {
                        if self.blue_value == 255 {
                            self.color_direction = -1;
                        }
                    } else {
                        if self.blue_value == 0 {
                            self.color_direction = 1;
                            self.manipulated_color = 1;
                        }
                    }

                    self.blue_value += self.color_direction;
                }

                widget_store
                    .get_widget_for_name("TextWidget")
                    .borrow_mut()
                    .set_color(
                        CONFIG_TEXT_COLOR,
                        [
                            (self.red_value as f32 / 255.0),
                            (self.green_value as f32 / 255.0),
                            (self.blue_value as f32 / 255.0),
                            1.0,
                        ],
                    );
            }

            "TimerWidget1" => {
                if self.animated {
                    self.progress += 1;

                    if self.progress > 100 {
                        self.progress = 0;
                    }

                    widget_store
                        .get_widget_for_name("ProgressWidget")
                        .borrow_mut()
                        .set_config(CONFIG_PROGRESS, Config::Numeric(self.progress as u64));

                    widget_store
                        .get_widget_for_name("ProgressText1")
                        .borrow_mut()
                        .set_text(CONFIG_DISPLAY_TEXT, format!("{} %", self.progress));
                }
            }

            _ => {}
        };
    }

    fn mouse_entered(&mut self, widget_id: i32, widget_store: &mut WidgetStore) {
        // When a mouse enters a widget, the ID will get modified; modify the debug widget
        // with the ID that was specified.
        let widget_name = String::from(widget_store.get_name_for_widget_id(widget_id));
        let widget_point = widget_store
            .get_widget_for_id(widget_id)
            .borrow_mut()
            .config()
            .get_point(CONFIG_ORIGIN);
        let widget_size = widget_store
            .get_widget_for_id(widget_id)
            .borrow_mut()
            .config()
            .get_size(CONFIG_BODY_SIZE);

        widget_store
            .get_widget_for_name("DebugText1")
            .borrow_mut()
            .set_config(
                CONFIG_DISPLAY_TEXT,
                Config::Text(format!("Current Widget: {} ({})", widget_id, widget_name)).clone(),
            );

        widget_store
            .get_widget_for_name("DebugText2")
            .borrow_mut()
            .set_config(
                CONFIG_DISPLAY_TEXT,
                Config::Text(format!(
                    "Dimensions: x={} y={} w={} h={}",
                    widget_point[0], widget_point[1], widget_size.w, widget_size.h
                ))
                .clone(),
            );
    }

    fn widget_selected(
        &mut self,
        widget_id: i32,
        _button: Button,
        selected: bool,
        widget_store: &mut WidgetStore,
    ) {
        match widget_store.get_name_for_widget_id(widget_id) {
            "AnimateButton1" => {
                self.animated = selected;
            }

            "DebugCheck1" => {
                widget_store
                    .get_widget_for_name("DebugText1")
                    .borrow_mut()
                    .set_toggle(CONFIG_WIDGET_HIDDEN, !selected);
                widget_store
                    .get_widget_for_name("DebugText2")
                    .borrow_mut()
                    .set_toggle(CONFIG_WIDGET_HIDDEN, !selected);
            }
            _ => (),
        }
    }

    fn widget_moved(&mut self, widget_id: i32, point: Point, widget_store: &mut WidgetStore) {
        match widget_store.get_name_for_widget_id(widget_id) {
            "BoxInLayoutWidget3" => {
                eprintln!("Reposition text inside BoxInLayoutWidget1");

                widget_store
                    .get_widget_for_name("LeftJustifiedText")
                    .borrow_mut()
                    .set_point(CONFIG_ORIGIN, point[0] + 16, point[1] + 10);

                widget_store
                    .get_widget_for_name("CenterJustifiedText")
                    .borrow_mut()
                    .set_point(CONFIG_ORIGIN, point[0] + 16, point[1] + 100 - 24);

                widget_store
                    .get_widget_for_name("RightJustifiedText")
                    .borrow_mut()
                    .set_point(CONFIG_ORIGIN, point[0] + 16, point[1] + 200 - 54);

                let layout_size2 = widget_store
                    .get_widget_for_name("BoxInLayoutWidget2")
                    .borrow_mut()
                    .config()
                    .get_size(CONFIG_BODY_SIZE);

                eprintln!("Reposition text inside BoxInLayoutWidget2");

                let box2_point = widget_store
                    .get_widget_for_name("BoxInLayoutWidget2")
                    .borrow_mut()
                    .config()
                    .get_point(CONFIG_ORIGIN);

                widget_store
                    .get_widget_for_name("MiniBox1")
                    .borrow_mut()
                    .set_point(CONFIG_ORIGIN, box2_point[0] + 10, box2_point[1] + 10);

                widget_store
                    .get_widget_for_name("MiniBox2")
                    .borrow_mut()
                    .set_point(
                        CONFIG_ORIGIN,
                        box2_point[0] + (layout_size2.w / 2) + 4,
                        box2_point[1] + 10,
                    );

                widget_store
                    .get_widget_for_name("MiniBox3")
                    .borrow_mut()
                    .set_point(
                        CONFIG_ORIGIN,
                        box2_point[0] + 10,
                        box2_point[1] + (layout_size2.h / 2) + 4,
                    );

                widget_store
                    .get_widget_for_name("MiniBox4")
                    .borrow_mut()
                    .set_point(
                        CONFIG_ORIGIN,
                        box2_point[0] + (layout_size2.w / 2) + 4,
                        box2_point[1] + (layout_size2.h / 2) + 4,
                    );
            }
            _ => (),
        }
    }

    fn widget_resized(&mut self, widget_id: i32, _size: Size, widget_store: &mut WidgetStore) {
        match widget_store.get_name_for_widget_id(widget_id) {
            "BoxInLayoutWidget3" => {
                let layout_size = widget_store
                    .get_widget_for_name("BoxInLayoutWidget1")
                    .borrow_mut()
                    .config()
                    .get_size(CONFIG_BODY_SIZE);

                eprintln!("Resize text inside BoxInLayoutWidget1");

                widget_store
                    .get_widget_for_name("LeftJustifiedText")
                    .borrow_mut()
                    .set_size(CONFIG_BODY_SIZE, layout_size.w - 32, 32);

                widget_store
                    .get_widget_for_name("CenterJustifiedText")
                    .borrow_mut()
                    .set_size(CONFIG_BODY_SIZE, layout_size.w - 32, 32);

                widget_store
                    .get_widget_for_name("RightJustifiedText")
                    .borrow_mut()
                    .set_size(CONFIG_BODY_SIZE, layout_size.w - 32, 32);

                let layout_size2 = widget_store
                    .get_widget_for_name("BoxInLayoutWidget2")
                    .borrow_mut()
                    .config()
                    .get_size(CONFIG_BODY_SIZE);

                eprintln!("Resize boxes inside BoxInLayoutWidget2: {:?}", layout_size2);

                widget_store
                    .get_widget_for_name("MiniBox1")
                    .borrow_mut()
                    .set_size(
                        CONFIG_BODY_SIZE,
                        (layout_size2.w / 2) - 12,
                        (layout_size2.h / 2) - 12,
                    );

                widget_store
                    .get_widget_for_name("MiniBox2")
                    .borrow_mut()
                    .set_size(
                        CONFIG_BODY_SIZE,
                        (layout_size2.w / 2) - 12,
                        (layout_size2.h / 2) - 12,
                    );

                widget_store
                    .get_widget_for_name("MiniBox3")
                    .borrow_mut()
                    .set_size(
                        CONFIG_BODY_SIZE,
                        (layout_size2.w / 2) - 12,
                        (layout_size2.h / 2) - 12,
                    );

                widget_store
                    .get_widget_for_name("MiniBox4")
                    .borrow_mut()
                    .set_size(
                        CONFIG_BODY_SIZE,
                        (layout_size2.w / 2) - 12,
                        (layout_size2.h / 2) - 12,
                    );
            }
            _ => (),
        }
    }
}

impl SimpleWindowEventHandler {
    fn new() -> Self {
        SimpleWindowEventHandler {
            animated: true,
            progress: 50,
            red_value: 0,
            green_value: 0,
            blue_value: 0,
            color_direction: 1,
            manipulated_color: 1,
        }
    }
}

impl SimpleWindow {
    fn new(prod: Pushrod) -> Self {
        Self {
            pushrod: RefCell::new(prod),
        }
    }

    fn add_hello_world(&mut self) {
        let mut text_widget = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Welcome to Pushrod".to_string(),
            36,
            TextJustify::Left,
        );

        text_widget.set_point(CONFIG_ORIGIN, 20, 10);
        text_widget.set_size(CONFIG_BODY_SIZE, 400, 50);
        text_widget.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
        text_widget.set_color(CONFIG_TEXT_COLOR, [0.75, 0.25, 1.0, 1.0]);

        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "TextWidget",
            Box::new(text_widget),
        );

        let mut timer = TimerWidget::new();

        timer.set_numeric(CONFIG_TIMER_TIMEOUT, 10);
        timer.set_toggle(CONFIG_TIMER_ENABLED, true);
        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "HelloWorldTimer",
            Box::new(timer),
        );
    }

    fn add_horizontal_layout(&mut self) {
        let mut base_widget: CanvasWidget = CanvasWidget::new();

        base_widget.set_point(CONFIG_ORIGIN, 20, 70);
        base_widget.set_size(CONFIG_BODY_SIZE, 760, 200);
        base_widget.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);

        let base_widget_id = self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "HorizontalManagerWidget1",
            Box::new(base_widget),
        );

        let base_layout_id =
            self.pushrod
                .borrow_mut()
                .add_layout_manager(Box::new(HorizontalLayoutManager::new(
                    base_widget_id,
                    LayoutManagerPadding {
                        top: 4,
                        left: 4,
                        right: 4,
                        bottom: 4,
                        spacing: 4,
                    },
                )));

        let mut box_widget = BoxWidget::new();

        box_widget.set_point(CONFIG_ORIGIN, 250, 80);
        box_widget.set_size(CONFIG_BODY_SIZE, 200, 200);
        box_widget.set_color(CONFIG_MAIN_COLOR, [0.0, 1.0, 0.0, 1.0]);
        box_widget.set_numeric(CONFIG_BORDER_WIDTH, 4);
        box_widget.set_color(CONFIG_BORDER_COLOR, [1.0, 0.0, 0.0, 1.0]);
        let box_widget_id = self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "BoxInLayoutWidget1",
            Box::new(box_widget),
            base_layout_id,
            make_origin_point(),
        );

        let mut text_widget2 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Left".to_string(),
            24,
            TextJustify::Left,
        );
        text_widget2.set_point(CONFIG_ORIGIN, 265, 100);
        text_widget2.set_size(CONFIG_BODY_SIZE, 170, 32);
        text_widget2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget2.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 0.0]);
        self.pushrod.borrow_mut().add_widget_to_parent(
            "LeftJustifiedText",
            Box::new(text_widget2),
            box_widget_id,
        );

        let mut text_widget3 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Center".to_string(),
            24,
            TextJustify::Center,
        );
        text_widget3.set_point(CONFIG_ORIGIN, 265, 166);
        text_widget3.set_size(CONFIG_BODY_SIZE, 170, 32);
        text_widget3.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget3.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 0.0]);
        self.pushrod.borrow_mut().add_widget_to_parent(
            "CenterJustifiedText",
            Box::new(text_widget3),
            box_widget_id,
        );

        let mut text_widget4 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Right".to_string(),
            24,
            TextJustify::Right,
        );
        text_widget4.set_point(CONFIG_ORIGIN, 265, 230);
        text_widget4.set_size(CONFIG_BODY_SIZE, 170, 32);
        text_widget4.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget4.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 0.0]);
        self.pushrod.borrow_mut().add_widget_to_parent(
            "RightJustifiedText",
            Box::new(text_widget4),
            box_widget_id,
        );

        let mut box_1 = BoxWidget::new();
        box_1.set_point(CONFIG_ORIGIN, 480, 80);
        box_1.set_size(CONFIG_BODY_SIZE, 200, 200);
        box_1.set_color(CONFIG_MAIN_COLOR, [0.5, 0.5, 1.0, 1.0]);
        box_1.set_numeric(CONFIG_BORDER_WIDTH, 2);
        box_1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 1.0, 1.0]);
        let box_1_id = self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "BoxInLayoutWidget2",
            Box::new(box_1),
            base_layout_id,
            make_origin_point(),
        );

        let mut inner_box_1 = BoxWidget::new();
        inner_box_1.set_point(CONFIG_ORIGIN, 505, 105);
        inner_box_1.set_size(CONFIG_BODY_SIZE, 70, 60);
        inner_box_1.set_color(CONFIG_MAIN_COLOR, [0.75, 0.75, 1.0, 1.0]);
        inner_box_1.set_numeric(CONFIG_BORDER_WIDTH, 1);
        inner_box_1.set_color(CONFIG_BORDER_COLOR, [1.0, 0.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent("MiniBox1", Box::new(inner_box_1), box_1_id);

        let mut inner_box_2 = BoxWidget::new();
        inner_box_2.set_point(CONFIG_ORIGIN, 585, 105);
        inner_box_2.set_size(CONFIG_BODY_SIZE, 70, 60);
        inner_box_2.set_color(CONFIG_MAIN_COLOR, [0.75, 0.25, 1.0, 1.0]);
        inner_box_2.set_numeric(CONFIG_BORDER_WIDTH, 1);
        inner_box_2.set_color(CONFIG_BORDER_COLOR, [1.0, 1.0, 0.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent("MiniBox2", Box::new(inner_box_2), box_1_id);

        let mut inner_box_3 = BoxWidget::new();
        inner_box_3.set_point(CONFIG_ORIGIN, 505, 190);
        inner_box_3.set_size(CONFIG_BODY_SIZE, 70, 60);
        inner_box_3.set_color(CONFIG_MAIN_COLOR, [0.25, 0.50, 0.75, 1.0]);
        inner_box_3.set_numeric(CONFIG_BORDER_WIDTH, 1);
        inner_box_3.set_color(CONFIG_BORDER_COLOR, [1.0, 0.50, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent("MiniBox3", Box::new(inner_box_3), box_1_id);

        let mut inner_box_4 = BoxWidget::new();
        inner_box_4.set_point(CONFIG_ORIGIN, 585, 190);
        inner_box_4.set_size(CONFIG_BODY_SIZE, 70, 60);
        inner_box_4.set_color(CONFIG_MAIN_COLOR, [0.75, 0.50, 0.0, 1.0]);
        inner_box_4.set_numeric(CONFIG_BORDER_WIDTH, 1);
        inner_box_4.set_color(CONFIG_BORDER_COLOR, [0.50, 0.0, 0.25, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent("MiniBox4", Box::new(inner_box_4), box_1_id);

        let mut box_2 = BoxWidget::new();
        box_2.set_point(CONFIG_ORIGIN, 480, 80);
        box_2.set_size(CONFIG_BODY_SIZE, 200, 200);
        box_2.set_color(
            CONFIG_MAIN_COLOR,
            [
                (rand::random::<u8>() as f32 / 255.0),
                (rand::random::<u8>() as f32 / 255.0),
                (rand::random::<u8>() as f32 / 255.0),
                1.0,
            ],
        );
        box_2.set_numeric(CONFIG_BORDER_WIDTH, 1);
        box_2.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "BoxInLayoutWidget3",
            Box::new(box_2),
            base_layout_id,
            make_origin_point(),
        );
    }

    fn add_horizontal_layout_buttons(&mut self) {
        let mut base_widget: CanvasWidget = CanvasWidget::new();

        base_widget.set_point(CONFIG_ORIGIN, 20, 270);
        base_widget.set_size(CONFIG_BODY_SIZE, 760, 36);
        base_widget.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);

        let base_widget_id = self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "HorizontalManagerWidget2",
            Box::new(base_widget),
        );

        let base_layout_id =
            self.pushrod
                .borrow_mut()
                .add_layout_manager(Box::new(HorizontalLayoutManager::new(
                    base_widget_id,
                    LayoutManagerPadding {
                        top: 0,
                        left: 4,
                        right: 4,
                        bottom: 4,
                        spacing: 4,
                    },
                )));

        let mut button1 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Hide".to_string(),
            18,
            TextJustify::Center,
        );
        button1.set_point(CONFIG_ORIGIN, 30, 236);
        button1.set_size(CONFIG_BODY_SIZE, 180, 32);
        button1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button1.get_callbacks().on_click(|x, widgets| {
            let state = get_widget_by_name(widgets, "BoxInLayoutWidget1".to_string())
                .config()
                .get_toggle(CONFIG_WIDGET_HIDDEN);
            let button_text = if state == true {
                String::from("Hide")
            } else {
                String::from("Show")
            };

            x.set_config(CONFIG_DISPLAY_TEXT, Config::Text(button_text));

            get_widget_by_name(widgets, "BoxInLayoutWidget1".to_string())
                .set_toggle(CONFIG_WIDGET_HIDDEN, !state);

            invalidate_all_widgets_except(widgets, x.get_widget_id());
        });

        self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "BoxInLayoutWidgetButton1",
            Box::new(button1),
            base_layout_id,
            make_origin_point(),
        );

        let mut button3 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Disable".to_string(),
            18,
            TextJustify::Center,
        );
        button3.set_point(CONFIG_ORIGIN, 30, 236);
        button3.set_size(CONFIG_BODY_SIZE, 180, 32);
        button3.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button3.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button3.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button3.get_callbacks().on_click(|x, widgets| {
            let state = get_widget_by_name(widgets, "BoxInLayoutWidget2".to_string())
                .config()
                .get_toggle(CONFIG_WIDGET_DISABLED);
            let button_text = if state == true {
                String::from("Disable")
            } else {
                String::from("Enable")
            };

            x.set_config(CONFIG_DISPLAY_TEXT, Config::Text(button_text));

            get_widget_by_name(widgets, "BoxInLayoutWidget2".to_string())
                .set_toggle(CONFIG_WIDGET_DISABLED, !state);

            get_widget_by_name(widgets, "MiniBox1".to_string())
                .set_toggle(CONFIG_WIDGET_DISABLED, !state);

            get_widget_by_name(widgets, "MiniBox2".to_string())
                .set_toggle(CONFIG_WIDGET_DISABLED, !state);

            get_widget_by_name(widgets, "MiniBox3".to_string())
                .set_toggle(CONFIG_WIDGET_DISABLED, !state);

            get_widget_by_name(widgets, "MiniBox4".to_string())
                .set_toggle(CONFIG_WIDGET_DISABLED, !state);
        });

        self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "BoxInLayoutWidgetButton2",
            Box::new(button3),
            base_layout_id,
            make_origin_point(),
        );

        let mut button5 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Randomize".to_string(),
            18,
            TextJustify::Center,
        );
        button5.set_point(CONFIG_ORIGIN, 30, 236);
        button5.set_size(CONFIG_BODY_SIZE, 180, 32);
        button5.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button5.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button5.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button5.get_callbacks().on_click(|_x, widgets| {
            get_widget_by_name(widgets, "BoxInLayoutWidget3".to_string()).set_config(
                CONFIG_MAIN_COLOR,
                Config::Color([
                    (rand::random::<u8>() as f32 / 255.0),
                    (rand::random::<u8>() as f32 / 255.0),
                    (rand::random::<u8>() as f32 / 255.0),
                    1.0,
                ]),
            );
        });

        self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "BoxInLayoutWidgetButton3",
            Box::new(button5),
            base_layout_id,
            make_origin_point(),
        );
    }

    fn add_vertical_layout(&mut self) {
        let mut base_widget: CanvasWidget = CanvasWidget::new();

        base_widget.set_point(CONFIG_ORIGIN, 20, 330);
        base_widget.set_size(CONFIG_BODY_SIZE, 200, 180);
        base_widget.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);

        let base_widget_id = self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "VerticalManagerWidget1",
            Box::new(base_widget),
        );

        let base_layout_id =
            self.pushrod
                .borrow_mut()
                .add_layout_manager(Box::new(VerticalLayoutManager::new(
                    base_widget_id,
                    LayoutManagerPadding {
                        top: 4,
                        left: 4,
                        right: 4,
                        bottom: 4,
                        spacing: 2,
                    },
                )));

        let mut progress_widget = ProgressWidget::new();

        progress_widget.set_point(CONFIG_ORIGIN, 20, 360);
        progress_widget.set_size(CONFIG_BODY_SIZE, 230, 32);
        progress_widget.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        progress_widget.set_color(CONFIG_SECONDARY_COLOR, [0.5, 0.5, 0.5, 1.0]);
        progress_widget.set_numeric(CONFIG_PROGRESS, 50);
        self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "ProgressWidget",
            Box::new(progress_widget),
            base_layout_id,
            make_origin_point(),
        );

        let mut progress_text = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "50%".to_string(),
            18,
            TextJustify::Center,
        );

        progress_text.set_point(CONFIG_ORIGIN, 260, 360);
        progress_text.set_size(CONFIG_BODY_SIZE, 50, 32);
        progress_text.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "ProgressText1",
            Box::new(progress_text),
            base_layout_id,
            make_origin_point(),
        );

        let mut button1 = ToggleButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Animate".to_string(),
            18,
            TextJustify::Center,
        );

        button1.set_point(CONFIG_ORIGIN, 340, 360);
        button1.set_size(CONFIG_BODY_SIZE, 160, 32);
        button1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button1.set_toggle(CONFIG_SELECTED, true);

        self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "AnimateButton1",
            Box::new(button1),
            base_layout_id,
            make_origin_point(),
        );

        let mut button2 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Randomize".to_string(),
            18,
            TextJustify::Center,
        );

        button2.set_point(CONFIG_ORIGIN, 520, 360);
        button2.set_size(CONFIG_BODY_SIZE, 160, 32);
        button2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button2.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button2.get_callbacks().on_click(|_x, widgets| {
            get_widget_by_name(widgets, "ProgressWidget".to_string()).set_config(
                CONFIG_SECONDARY_COLOR,
                Config::Color([
                    (rand::random::<u8>() as f32 / 255.0),
                    (rand::random::<u8>() as f32 / 255.0),
                    (rand::random::<u8>() as f32 / 255.0),
                    1.0,
                ]),
            );
        });

        self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "RandomColorButton2",
            Box::new(button2),
            base_layout_id,
            make_origin_point(),
        );
    }

    fn add_timer(&mut self) {
        let mut timer = TimerWidget::new();

        timer.set_numeric(CONFIG_TIMER_TIMEOUT, 100);
        timer.set_toggle(CONFIG_TIMER_ENABLED, true);
        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "TimerWidget1",
            Box::new(timer),
        );
    }

    fn add_debugging(&mut self) {
        let mut check_widget = CheckboxWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Enable Debugging".to_string(),
            20,
            TextJustify::Left,
            true,
        );
        check_widget.set_point(CONFIG_ORIGIN, 20, 540);
        check_widget.set_size(CONFIG_BODY_SIZE, 500, 28);
        check_widget.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "DebugCheck1",
            Box::new(check_widget),
        );

        let mut text_widget1 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Current Widget: 0".to_string(),
            20,
            TextJustify::Left,
        );
        text_widget1.set_point(CONFIG_ORIGIN, 20, 570);
        text_widget1.set_size(CONFIG_BODY_SIZE, 500, 28);
        text_widget1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "DebugText1",
            Box::new(text_widget1),
        );

        let mut text_widget2 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "".to_string(),
            20,
            TextJustify::Left,
        );
        text_widget2.set_point(CONFIG_ORIGIN, 20, 600);
        text_widget2.set_size(CONFIG_BODY_SIZE, 500, 28);
        text_widget2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "DebugText2",
            Box::new(text_widget2),
        );
    }

    fn add_powered_by(&mut self) {
        let mut image_widget = ImageButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Powered By Rust!".to_string(),
            "rust-512x512.jpg".to_string(),
            18,
            TextJustify::Left,
        );

        image_widget.set_point(CONFIG_ORIGIN, 570, 580);
        image_widget.set_size(CONFIG_BODY_SIZE, 220, 48);
        image_widget.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        image_widget.set_numeric(CONFIG_BORDER_WIDTH, 1);
        image_widget.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "RustImageButton",
            Box::new(image_widget),
        );
    }

    fn build(&mut self) {
        self.add_hello_world();
        self.add_horizontal_layout();
        self.add_horizontal_layout_buttons();
        self.add_vertical_layout();
        self.add_timer();
        self.add_debugging();
        self.add_powered_by();
    }

    fn get_pushrod(&mut self) -> &mut Pushrod {
        self.pushrod.get_mut()
    }

    pub fn run(&mut self) {
        let mut handler = SimpleWindowEventHandler::new();

        self.build();
        self.get_pushrod().run(&mut handler);
    }
}

fn main() {
    let window: GlfwWindow = WindowSettings::new("Pushrod Window", [800, 640])
        .resizable(true)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));
    let mut app_window = SimpleWindow::new(Pushrod::new(window));

    app_window.run();
}
