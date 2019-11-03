// Pushrod Rendering Library
// Widget Configuration Store
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

use std::cell::RefCell;

use crate::widgets::image_widget::ImageWidget;
use crate::widgets::progress_widget::ProgressWidget;
use crate::widgets::text_widget::TextWidget;
use crate::widgets::timer_widget::TimerWidget;

pub enum WidgetStore {
    ImageWidget(RefCell<ImageWidget>),
    ProgressWidget(RefCell<ProgressWidget>),
    TextWidget(RefCell<TextWidget>),
    TimerWidget(RefCell<TimerWidget>),
}
