// Main Event Dispatcher
// Master of the Universe
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

use crate::core::point::*;
use crate::core::widget_store::*;

use piston_window::*;

/// This structure is returned when instantiating a new Pushrod main object.
/// It stores the OpenGL configuration that is desired for drawing, a list of references
/// to a managed set of `PushrodWindow` objects, registered `EventListener`s, and
/// `PushrodEvent` objects that are pending dispatch.
///
/// The objects contained within this structure are used by the `Pushrod` run loop, and
/// are not intended to be modified except through methods in the `Pushrod` impl.
pub struct Pushrod {
    window: PistonWindow,
    pub widget_store: WidgetStore,
//    event_listeners: RefCell<Vec<Box<EventListener>>>,
//    event_list: RefCell<Vec<PushrodEvent>>,
}

/// Pushrod implementation.  Create a `Pushrod::new( OpenGL )` object to create a new
/// main loop.  Only one of these should be set for the entire application runtime.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::main::*;
/// # fn main() {
///     // Create a PushrodWindow container to store the PistonWindow
///     let mut prod: Pushrod = Pushrod::new(
///         WindowSettings::new("Pushrod Window", [640, 480])
///             .opengl(OpenGL::V3_2)
///             .build()
///             .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)));
///
///     // Initiate the run loop.
///     prod.run();
/// # }
/// ```
impl Pushrod {
    /// Pushrod Object Constructor.  Takes in a single OpenGL configuration type.
    pub fn new(window: PistonWindow) -> Self {
        Self {
            window,
            widget_store: WidgetStore::new(),
//            event_listeners: RefCell::new(Vec::new()),
//            event_list: RefCell::new(Vec::new()),
        }
    }

    /// Adds an event listener to the stack.  This should be an implementation of the
    /// `PushrodEventListener` trait.
    ///
    /// Example:
    /// ```no_run
    /// # use piston_window::*;
    /// # use pushrod::core::main::*;
    /// # use pushrod::core::point::*;
    /// fn main() {
    ///     // Create a new Pushrod object
    ///     let mut prod: Pushrod = Pushrod::new(WindowSettings::new("Pushrod Window", [640, 480])
    ///             .opengl(OpenGL::V3_2)
    ///             .build()
    ///             .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)));
    ///
    ///     // Initiate the run loop.
    ///     prod.run();
    /// }
    /// ```
//    pub fn add_event_listener_for_window(&self, _listener: Box<EventListener>) {
//        //        self.event_listeners.borrow_mut().push(listener);
//    }
//
//    /*
//     * By handling events internally, we bypass the risk of the user having to interpret each
//     * event, and having to figure out how to dispatch those events to any widgets that might be
//     * in the display area.  Events will eventually be dispatched using a "dispatch all" method,
//     * which will be done at the end of the event loop.  Any draw routines will be done within
//     * the render_args() area, and a separate event will be sent out for that, as drawing
//     * should be done at the end of all event processing, within the rendering loop, not the
//     * updating loop (UPS vs. FPS)
//     */
//
//    fn internal_handle_mouse_move(&self, _point: Point) {
//        //        // Send the point movement to the widget event handler.
//        //
//        //        self.event_list
//        //            .borrow_mut()
//        //            .push(PushrodEvent::MouseEvent { point });
//    }
//
//    fn internal_handle_mouse_button(&self, _button: ButtonArgs) {
//        //        // Send the button click to the widget event handler.
//        //
//        //        if button.state == ButtonState::Press {
//        //            match button.button {
//        //                Button::Mouse(button) => {
//        //                    self.event_list
//        //                        .borrow_mut()
//        //                        .push(PushrodEvent::MouseDownEvent { button });
//        //                }
//        //                _ => (),
//        //            }
//        //        } else if button.state == ButtonState::Release {
//        //            match button.button {
//        //                Button::Mouse(button) => {
//        //                    self.event_list
//        //                        .borrow_mut()
//        //                        .push(PushrodEvent::MouseUpEvent { button });
//        //                }
//        //                _ => (),
//        //            }
//        //        }
//    }
//
//    fn internal_handle_mouse_scroll(&self, _point: Point) {
//        //        // Send the mouse scroll to the widget event handler.
//        //
//        //        self.event_list
//        //            .borrow_mut()
//        //            .push(PushrodEvent::MouseScrollEvent { point });
//    }
//
//    fn internal_dispatch_events(&self) {
//        //        for event in self.event_list.borrow_mut().iter() {
//        //            for listener in self.event_listeners.borrow_mut().iter() {
//        //                let event_mask = self.internal_derive_event_mask(event);
//        //
//        //                if listener.event_mask() & event_mask == event_mask {
//        //                    listener.handle_event(event);
//        //                }
//        //            }
//        //        }
//        //
//        //        self.event_list.borrow_mut().clear();
//    }
//
//    fn internal_derive_event_mask(&self, _event: &PushrodEvent) -> EventMask {
//        //        match event {
//        //            PushrodEvent::MouseEvent { point: _ } => MASK_EVENT_MOUSE_MOVED,
//        //            PushrodEvent::MouseDownEvent { button: _ } => MASK_EVENT_MOUSE_DOWN,
//        //            PushrodEvent::MouseUpEvent { button: _ } => MASK_EVENT_MOUSE_UP,
//        //            PushrodEvent::MouseScrollEvent { point: _ } => MASK_EVENT_MOUSE_SCROLL,
//        //        }
//        0
//    }

    fn handle_draw(&mut self, event: &Event) {
        let widgets = &mut self.widget_store;

        self.window.draw_2d(event, |c, g| widgets.draw(0, c, g));
    }

    /// This is the main run loop that is called to process all UI events.  This loop is responsible
    /// for handling events from the OS, converting them to workable objects, and passing them off
    /// to quick callback dispatchers.
    ///
    /// The run loop handles events in the following order:
    ///
    /// - Mouse events
    ///   - Movement events
    ///   - Button events
    ///   - Scroll button events
    /// - Custom events are then dispatched to any registered event listeners
    /// - Draw loop
    ///   - Draw only widgets whose states have become invalidated
    ///   - Swap display buffers if required
    ///
    /// This event is handled window-by-window.  Once a window has processed all of its pending
    /// events, the next window is then processed.  No particular window takes precidence - any
    /// window that has events to process gets handled in order.
    pub fn run(&mut self) {
        let mut last_widget_id = -1;
        let mut previous_mouse_position: Point = make_origin_point();
//        let draw_size = self.window.draw_size();

        while let Some(ref event) = &self.window.next() {
            event.mouse_cursor(|x, y| {
                let mouse_point = make_point_f64(x, y);

                if mouse_point.x != previous_mouse_position.x
                    || mouse_point.y != previous_mouse_position.y
                {
                    previous_mouse_position = mouse_point.clone();

                    let current_widget_id = self
                        .widget_store
                        .get_widget_id_for_point(mouse_point.clone());
                    let current_parent_for_widget =
                        self.widget_store.get_parent_of(current_widget_id);

//                    self.internal_handle_mouse_move(mouse_point.clone());

                    // Handles the mouse move callback.
                    if current_widget_id != -1 {
                        self.widget_store
                            .mouse_moved_for_id(current_widget_id, mouse_point.clone());
                    }

                    if current_widget_id != last_widget_id {
                        if last_widget_id != -1 {
                            self.widget_store.mouse_exited_for_id(last_widget_id);
                        }

                        last_widget_id = current_widget_id;

                        if last_widget_id != -1 {
                            self.widget_store.mouse_entered_for_id(last_widget_id);
                        }

                        eprintln!(
                            "Widget IDs: current={} parent={} children={:?}",
                            current_widget_id,
                            current_parent_for_widget,
                            self.widget_store.get_children_of(current_widget_id)
                        );
                    }
                }
            });

//            event.button(|button| {
//                self.internal_handle_mouse_button(button);
//            });

            event.mouse_scroll(|x, y| {
                let mouse_point = make_point_f64(x, y);

//                self.internal_handle_mouse_scroll(mouse_point.clone());

                if last_widget_id != -1 {
                    self.widget_store
                        .mouse_scrolled_for_id(last_widget_id, mouse_point.clone());
                }
            });

            event.resize(|_, _| {
                self.widget_store.invalidate_all_widgets();
            });

            // Dispatch events here in the bus
//            self.internal_dispatch_events();

            // FPS loop handling

            event.render(|_| {
                self.handle_draw(&event);
                self.widget_store.invalidate_all_widgets();
            });
        }
    }
}
