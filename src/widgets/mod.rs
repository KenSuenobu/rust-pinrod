// Pushrod Widgets Library
// Core Widgets
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

#[macro_use]

/// This is a `TextWidget`, which draws text in a clipped area.
pub mod text_widget;

/// This is an `ImageWidget`, which draws an Image in a clipped area.
pub mod image_widget;

/// This is a `ProgressWidget`, which draws a progress bar.
pub mod progress_widget;

/// This is a `TimerWidget`, which times out after a specified duration, triggering a callback
/// after the timeout is exceeded.
pub mod timer_widget;
