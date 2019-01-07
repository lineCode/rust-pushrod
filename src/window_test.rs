// Window Test
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

use pushrod::core::main::*;
use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;

    println!("Starting Test.");

    let prod: Pushrod = Pushrod::new(opengl);

    let mut window: PistonWindow = WindowSettings::new(
        "Pushrod Window",
        [640, 480]
    )
        .opengl(opengl)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

    window.set_max_fps(60);
    window.set_ups(60);

    // Adds a window to the stack of watched events
    prod.add_window(window);

    // Runs the main event loop
    prod.run();
}