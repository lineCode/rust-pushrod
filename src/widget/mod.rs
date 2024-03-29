// Widget Module
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

/// Base component and UI Components (Widget) library.  These components are used for on-screen interactions
/// between the user and the application.
pub mod widget;

/// Box component: draws a box on the screen with adjustable border color and width.
pub mod box_widget;

/// Timer component: triggers a callback after a certain amount of time.
pub mod timer_widget;

/// Text component: draws text on the screen with an adjustable text, font size, color, and fpnt name.
pub mod text_widget;

/// `Configurable` definition, used by `Widget` objects to store configuration settings.
pub mod config;
