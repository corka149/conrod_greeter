extern crate conrod;

mod core;
mod event_handling;
mod util;
mod widgets;

use event_handling::*;
use util::EventLoop;

pub fn bootstrap() {
    let mut ui_core = core::UiCore::new(String::from("Conrod Greeter"), 400, 300);

    let widgets_register = widgets::WidgetRegister::new(&mut ui_core.ui);

    // Create a UI renderer
    let mut renderer = conrod::backend::glium::Renderer::new(&ui_core.display).unwrap();

    // Drawing loop for the immediate mode GUI
    'main: loop {
        let mut event_loop = EventLoop::new();
        for event in event_loop.next(&mut ui_core.events_loop) {
            if let EventResult::BREAK = react_on_event(event.clone()) {
                break 'main;
            }

            // add event handling for Conrod. Putting the following at the beginning of the event
            // for loop will take care of UI events
            if let Some(event) = conrod::backend::winit::convert_event(event, &ui_core.display) {
                // ui.handle_event() is the business end of Conrod - it takes events off the queue,
                // works out which widget they apply to and looks after dispatch, etc.
                ui_core.ui.handle_event(event);
            }

            widgets_register.register(&mut ui_core.ui);
        }

        ui_core.draw_if_changed(&mut renderer);
    }
}
