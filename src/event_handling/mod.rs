use conrod::backend::glium::glium;

pub enum EventResult {
  CONTINUE,
  BREAK
}

pub fn react_on_event(event: glium::glutin::Event) -> EventResult {
  match event {
    glium::glutin::Event::WindowEvent { event, .. } => match event {
      glium::glutin::WindowEvent::Closed
      | glium::glutin::WindowEvent::KeyboardInput {
        input:
        glium::glutin::KeyboardInput {
          virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
          ..
        },
        ..
      } => return EventResult::BREAK,
      _ => (),
    },
    _ => (),
  }
  EventResult::CONTINUE
}