#[macro_use]
extern crate conrod;

mod core;
mod util;

use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Colorable, Positionable, Widget};
use util::EventLoop;

pub fn bootstrap() {
  let mut ui_core = core::UiCore::new(String::from("Conrod Greeter"), 400, 300);

  // a macro provided by conrod to create widget ids. Conrod's primary data structure contains all
  // the widgets, and uses their ids to keep track of them. The widget_ids! macro just provides a
  // very quick and easy way of defining widget names (in this case just text) and giving them ids.
  widget_ids!(struct Ids { text });
  // Ids::new creates the the widget structure.
  let ids = Ids::new(ui_core.ui.widget_id_generator());

  // Conrod can use graphics. It stores these in a map. The system needs the map,
  // even though it doesn't contain anything at this time, so create it:
  let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

  // Add a `Font` to the `Ui`'s `font::Map` from file.
  const FONT_PATH: &'static str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/NotoSans/NotoSans-Regular.ttf"
  );
  ui_core.ui.fonts.insert_from_file(FONT_PATH).unwrap();

  // Finally, Conrod needs to render its UI. It uses a renderer to do this, so create one:
  let mut renderer = conrod::backend::glium::Renderer::new(&ui_core.display).unwrap();

  // As an Immediate Mode GUI, Conrod sits in the main loop of the program, drawing the UI every
  // time round the loop. Here's the main loop:
  'main: loop {
    let mut event_loop = EventLoop::new();
    for event in event_loop.next(&mut ui_core.events_loop) {
      match event.clone() {
        glium::glutin::Event::WindowEvent { event, .. } => match event {
          glium::glutin::WindowEvent::Closed
          | glium::glutin::WindowEvent::KeyboardInput {
            input:
            glium::glutin::KeyboardInput {
              virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
              ..
            },
            ..
          } => break 'main,
          _ => (),
        },
        _ => (),
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

      let ui = &mut ui_core.ui.set_widgets();
      // "Hello World!" in the middle of the screen.
      widget::Text::new("Hello World!")
        .middle_of(ui.window)
        .color(conrod::color::WHITE)
        .font_size(32)
        .set(ids.text, ui);
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
