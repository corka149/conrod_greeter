extern crate conrod;

mod core;
mod util;
mod widgets;
mod event_handling;

use conrod::backend::glium::glium::{self, Surface};
use util::EventLoop;
use event_handling::*;


pub fn bootstrap() {
  let mut ui_core = core::UiCore::new(String::from("Conrod Greeter"), 400, 300);

  let  widgets_register = widgets::WidgetRegister::new(&mut ui_core.ui);

  // Create empty graphics map because it is a mandatory parameter
  let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

  // Load fonts
  const FONT_PATH: &'static str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/NotoSans/NotoSans-Regular.ttf"
  );
  ui_core.ui.fonts.insert_from_file(FONT_PATH).unwrap();

  // Create a UI renderer
  let mut renderer = conrod::backend::glium::Renderer::new(&ui_core.display).unwrap();

  // Drawing loop for the immediate mode GUI
  'main: loop {
    let mut event_loop = EventLoop::new();
    for event in event_loop.next(&mut ui_core.events_loop) {

      match react_on_event(event.clone()) {
        EventResult::BREAK => break 'main,
        EventResult::CONTINUE => ()
      }

      // add event handling for Conrod. Putting the following at the beginning of the event
      // for loop will take care of UI events
      if let Some(event) =
      conrod::backend::winit::convert_event(event, &ui_core.display)
        {
          // ui.handle_event() is the business end of Conrod - it takes events off the queue,
          // works out which widget they apply to and looks after dispatch, etc.
          ui_core.ui.handle_event(event);
        }

      widgets_register.register(&mut ui_core.ui);
    }

    // Render the `Ui` and then display it on the screen.
    if let Some(primitives) = ui_core.ui.draw_if_changed() {
      renderer.fill(&ui_core.display, primitives, &image_map);

      // starts drawing on the backbuffer
      let mut target = ui_core.display.draw();
      target.clear_color(0.0, 1.0, 0.0, 1.0);
      renderer
        .draw(&ui_core.display, &mut target, &image_map)
        .unwrap();
      target.finish().unwrap();
    }
  }
}
