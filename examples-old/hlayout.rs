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
use pushrod::core::point::make_origin_point;
use pushrod::core::widget_store::*;
use pushrod::widget::box_widget::*;
use pushrod::widget::config::*;
use pushrod::widget::push_button_widget::*;
use pushrod::widget::text_widget::*;
use pushrod::widget::widget::*;

pub struct SimpleWindow {
    pushrod: RefCell<Pushrod>,
    layout_id: i32,
}

pub struct SimpleWindowEventHandler {
    top_padding: i32,
    left_padding: i32,
    right_padding: i32,
    bottom_padding: i32,
    spacing: i32,
    layout_id: i32,
}

impl PushrodCallbackEvents for SimpleWindowEventHandler {
    fn widget_clicked(&mut self, widget_id: i32, button: Button, widget_store: &mut WidgetStore) {
        match button {
            Button::Mouse(mouse_button) => {
                if mouse_button != MouseButton::Left {
                    return;
                }
            }
            _ => (),
        }

        match widget_store.get_name_for_widget_id(widget_id) {
            "TopButtonPlus" => {
                self.top_padding += 1;
                if self.top_padding > 10 {
                    self.top_padding = 10;
                }
                self.refresh_layout(widget_store);
            }

            "TopButtonMinus" => {
                self.top_padding -= 1;
                if self.top_padding <= 0 {
                    self.top_padding = 0;
                }
                self.refresh_layout(widget_store);
            }

            "LeftButtonPlus" => {
                self.left_padding += 1;
                if self.left_padding > 20 {
                    self.left_padding = 20;
                }
                self.refresh_layout(widget_store);
            }

            "LeftButtonMinus" => {
                self.left_padding -= 1;
                if self.left_padding <= 0 {
                    self.left_padding = 0;
                }
                self.refresh_layout(widget_store);
            }

            "RightButtonPlus" => {
                self.right_padding += 1;
                if self.right_padding > 20 {
                    self.right_padding = 20;
                }
                self.refresh_layout(widget_store);
            }

            "RightButtonMinus" => {
                self.right_padding -= 1;
                if self.right_padding <= 0 {
                    self.right_padding = 0;
                }
                self.refresh_layout(widget_store);
            }

            "BottomButtonPlus" => {
                self.bottom_padding += 1;
                if self.bottom_padding > 10 {
                    self.bottom_padding = 10;
                }
                self.refresh_layout(widget_store);
            }

            "BottomButtonMinus" => {
                self.bottom_padding -= 1;
                if self.bottom_padding <= 0 {
                    self.bottom_padding = 0;
                }
                self.refresh_layout(widget_store);
            }

            "SpacingButtonPlus" => {
                self.spacing += 1;
                if self.spacing > 12 {
                    self.spacing = 12;
                }
                self.refresh_layout(widget_store);
            }

            "SpacingButtonMinus" => {
                self.spacing -= 1;
                if self.spacing <= 0 {
                    self.spacing = 0;
                }
                self.refresh_layout(widget_store);
            }

            _ => (),
        }
    }
}

impl SimpleWindowEventHandler {
    fn new() -> Self {
        SimpleWindowEventHandler {
            top_padding: 1,
            left_padding: 1,
            right_padding: 1,
            bottom_padding: 1,
            spacing: 1,
            layout_id: 0,
        }
    }

    fn refresh_layout(&mut self, widget_store: &mut WidgetStore) {
        widget_store.adjust_layout_manager(
            self.layout_id,
            LayoutManagerPadding {
                top: self.top_padding,
                left: self.left_padding,
                right: self.right_padding,
                bottom: self.bottom_padding,
                spacing: self.spacing,
            },
        );

        widget_store
            .get_widget_for_name("TopButtonText")
            .borrow_mut()
            .set_config(
                CONFIG_DISPLAY_TEXT,
                Config::Text(format!("{}", self.top_padding)).clone(),
            );

        widget_store
            .get_widget_for_name("LeftButtonText")
            .borrow_mut()
            .set_config(
                CONFIG_DISPLAY_TEXT,
                Config::Text(format!("{}", self.left_padding)).clone(),
            );

        widget_store
            .get_widget_for_name("RightButtonText")
            .borrow_mut()
            .set_config(
                CONFIG_DISPLAY_TEXT,
                Config::Text(format!("{}", self.right_padding)).clone(),
            );

        widget_store
            .get_widget_for_name("BottomButtonText")
            .borrow_mut()
            .set_config(
                CONFIG_DISPLAY_TEXT,
                Config::Text(format!("{}", self.bottom_padding)).clone(),
            );

        widget_store
            .get_widget_for_name("SpacingButtonText")
            .borrow_mut()
            .set_config(
                CONFIG_DISPLAY_TEXT,
                Config::Text(format!("{}", self.spacing)).clone(),
            );

        widget_store.invalidate_all_widgets();
    }

    pub fn set_layout_id(&mut self, layout_id: i32) {
        self.layout_id = layout_id;
    }
}

impl SimpleWindow {
    fn new(prod: Pushrod) -> Self {
        Self {
            pushrod: RefCell::new(prod),
            layout_id: 0,
        }
    }

    fn add_horizontal_layout(&mut self) {
        let mut base_widget: BoxWidget = BoxWidget::new();

        base_widget.set_point(CONFIG_ORIGIN, 20, 20);
        base_widget.set_size(CONFIG_BODY_SIZE, 760, 200);
        base_widget.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
        base_widget.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        base_widget.set_numeric(CONFIG_BORDER_WIDTH, 1);

        let base_widget_id = self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "HorizontalManagerWidget1",
            Box::new(base_widget),
        );

        self.layout_id =
            self.pushrod
                .borrow_mut()
                .add_layout_manager(Box::new(HorizontalLayoutManager::new(
                    base_widget_id,
                    LayoutManagerPadding {
                        top: 1,
                        left: 1,
                        right: 1,
                        bottom: 1,
                        spacing: 1,
                    },
                )));

        let mut box_widget = BoxWidget::new();

        box_widget.set_point(CONFIG_ORIGIN, 250, 80);
        box_widget.set_size(CONFIG_BODY_SIZE, 200, 200);
        box_widget.set_color(CONFIG_MAIN_COLOR, [0.0, 1.0, 0.0, 1.0]);
        box_widget.set_numeric(CONFIG_BORDER_WIDTH, 4);
        box_widget.set_color(CONFIG_BORDER_COLOR, [1.0, 0.0, 0.0, 1.0]);
        let _box_widget_id = self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "BoxInLayoutWidget1",
            Box::new(box_widget),
            self.layout_id,
            make_origin_point(),
        );

        let mut box_1 = BoxWidget::new();
        box_1.set_point(CONFIG_ORIGIN, 480, 80);
        box_1.set_size(CONFIG_BODY_SIZE, 200, 200);
        box_1.set_color(CONFIG_MAIN_COLOR, [0.5, 0.5, 1.0, 1.0]);
        box_1.set_numeric(CONFIG_BORDER_WIDTH, 2);
        box_1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 1.0, 1.0]);
        let _box_1_id = self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "BoxInLayoutWidget2",
            Box::new(box_1),
            self.layout_id,
            make_origin_point(),
        );

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
            self.layout_id,
            make_origin_point(),
        );
    }

    fn add_horizontal_layout_buttons(&mut self) {
        let mut text_widget1 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Top Padding:".to_string(),
            18,
            TextJustify::Right,
        );
        text_widget1.set_point(CONFIG_ORIGIN, 20, 230);
        text_widget1.set_size(CONFIG_BODY_SIZE, 170, 24);
        text_widget1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget1.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("TextWidget1", Box::new(text_widget1));

        let mut text_widget2 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Left Padding:".to_string(),
            18,
            TextJustify::Right,
        );
        text_widget2.set_point(CONFIG_ORIGIN, 20, 258);
        text_widget2.set_size(CONFIG_BODY_SIZE, 170, 20);
        text_widget2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget2.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("TextWidget2", Box::new(text_widget2));

        let mut text_widget3 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Right Padding:".to_string(),
            18,
            TextJustify::Right,
        );
        text_widget3.set_point(CONFIG_ORIGIN, 20, 284);
        text_widget3.set_size(CONFIG_BODY_SIZE, 170, 20);
        text_widget3.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget3.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("TextWidget3", Box::new(text_widget3));

        let mut text_widget4 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Bottom Padding:".to_string(),
            18,
            TextJustify::Right,
        );
        text_widget4.set_point(CONFIG_ORIGIN, 20, 310);
        text_widget4.set_size(CONFIG_BODY_SIZE, 170, 20);
        text_widget4.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget4.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("TextWidget4", Box::new(text_widget4));

        let mut text_widget5 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "Spacing:".to_string(),
            18,
            TextJustify::Right,
        );
        text_widget5.set_point(CONFIG_ORIGIN, 20, 336);
        text_widget5.set_size(CONFIG_BODY_SIZE, 170, 20);
        text_widget5.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget5.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("TextWidget5", Box::new(text_widget5));

        let mut button1 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "+".to_string(),
            18,
            TextJustify::Center,
        );
        button1.set_point(CONFIG_ORIGIN, 210, 232);
        button1.set_size(CONFIG_BODY_SIZE, 40, 22);
        button1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button1.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("TopButtonPlus", Box::new(button1));

        let mut button2 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "-".to_string(),
            18,
            TextJustify::Center,
        );
        button2.set_point(CONFIG_ORIGIN, 254, 232);
        button2.set_size(CONFIG_BODY_SIZE, 40, 22);
        button2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button2.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button2.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("TopButtonMinus", Box::new(button2));

        let mut text_widget1 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "1".to_string(),
            18,
            TextJustify::Left,
        );
        text_widget1.set_point(CONFIG_ORIGIN, 310, 230);
        text_widget1.set_size(CONFIG_BODY_SIZE, 40, 24);
        text_widget1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget1.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("TopButtonText", Box::new(text_widget1));

        let mut button3 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "+".to_string(),
            18,
            TextJustify::Center,
        );
        button3.set_point(CONFIG_ORIGIN, 210, 258);
        button3.set_size(CONFIG_BODY_SIZE, 40, 22);
        button3.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button3.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button3.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("LeftButtonPlus", Box::new(button3));

        let mut button4 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "-".to_string(),
            18,
            TextJustify::Center,
        );
        button4.set_point(CONFIG_ORIGIN, 254, 258);
        button4.set_size(CONFIG_BODY_SIZE, 40, 22);
        button4.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button4.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button4.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("LeftButtonMinus", Box::new(button4));

        let mut text_widget2 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "1".to_string(),
            18,
            TextJustify::Left,
        );
        text_widget2.set_point(CONFIG_ORIGIN, 310, 256);
        text_widget2.set_size(CONFIG_BODY_SIZE, 40, 24);
        text_widget2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget2.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("LeftButtonText", Box::new(text_widget2));

        let mut button5 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "+".to_string(),
            18,
            TextJustify::Center,
        );
        button5.set_point(CONFIG_ORIGIN, 210, 284);
        button5.set_size(CONFIG_BODY_SIZE, 40, 22);
        button5.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button5.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button5.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("RightButtonPlus", Box::new(button5));

        let mut button6 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "-".to_string(),
            18,
            TextJustify::Center,
        );
        button6.set_point(CONFIG_ORIGIN, 254, 284);
        button6.set_size(CONFIG_BODY_SIZE, 40, 22);
        button6.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button6.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button6.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("RightButtonMinus", Box::new(button6));

        let mut text_widget3 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "1".to_string(),
            18,
            TextJustify::Left,
        );
        text_widget3.set_point(CONFIG_ORIGIN, 310, 282);
        text_widget3.set_size(CONFIG_BODY_SIZE, 40, 24);
        text_widget3.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget3.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("RightButtonText", Box::new(text_widget3));

        let mut button7 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "+".to_string(),
            18,
            TextJustify::Center,
        );
        button7.set_size(CONFIG_BODY_SIZE, 40, 22);
        button7.set_point(CONFIG_ORIGIN, 210, 310);
        button7.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button7.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button7.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("BottomButtonPlus", Box::new(button7));

        let mut button8 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "-".to_string(),
            18,
            TextJustify::Center,
        );
        button8.set_size(CONFIG_BODY_SIZE, 40, 22);
        button8.set_point(CONFIG_ORIGIN, 254, 310);
        button8.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button8.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button8.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("BottomButtonMinus", Box::new(button8));

        let mut text_widget4 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "1".to_string(),
            18,
            TextJustify::Left,
        );
        text_widget4.set_point(CONFIG_ORIGIN, 310, 308);
        text_widget4.set_size(CONFIG_BODY_SIZE, 40, 24);
        text_widget4.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget4.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("BottomButtonText", Box::new(text_widget4));

        let mut button9 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "+".to_string(),
            18,
            TextJustify::Center,
        );
        button9.set_size(CONFIG_BODY_SIZE, 40, 22);
        button9.set_point(CONFIG_ORIGIN, 210, 336);
        button9.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button9.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button9.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("SpacingButtonPlus", Box::new(button9));

        let mut button10 = PushButtonWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "-".to_string(),
            18,
            TextJustify::Center,
        );
        button10.set_size(CONFIG_BODY_SIZE, 40, 22);
        button10.set_point(CONFIG_ORIGIN, 254, 336);
        button10.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button10.set_numeric(CONFIG_BORDER_WIDTH, 1);
        button10.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("SpacingButtonMinus", Box::new(button10));

        let mut text_widget5 = TextWidget::new(
            "assets/OpenSans-Regular.ttf".to_string(),
            "1".to_string(),
            18,
            TextJustify::Left,
        );
        text_widget5.set_point(CONFIG_ORIGIN, 310, 334);
        text_widget5.set_size(CONFIG_BODY_SIZE, 40, 24);
        text_widget5.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        text_widget5.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("SpacingButtonText", Box::new(text_widget5));
    }

    fn build(&mut self) {
        self.add_horizontal_layout();
        self.add_horizontal_layout_buttons();
    }

    fn get_pushrod(&mut self) -> &mut Pushrod {
        self.pushrod.get_mut()
    }

    pub fn run(&mut self) {
        let mut handler = SimpleWindowEventHandler::new();

        self.build();

        handler.set_layout_id(self.layout_id);

        self.get_pushrod().run(&mut handler);
    }
}

fn main() {
    let window: GlfwWindow = WindowSettings::new("Horizontal Layout Example", [800, 370])
        .resizable(false)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));
    let mut app_window = SimpleWindow::new(Pushrod::new(window));

    app_window.run();
}
