use conrod::backend::glium::glium;
use conrod;

pub struct UiCore {
    pub events_loop: glium::glutin::EventsLoop,
    pub display: glium::Display,
    pub ui: conrod::Ui
}

impl UiCore {

    pub fn new(title: String, init_width: u32, init_height: u32) -> UiCore {

        // which will handle interaction with the UI
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(init_width, init_height);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        // The display is the home for the UI, and is an OpenGL context provided by glium.
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let ui = conrod::UiBuilder::new([init_width as f64, init_height as f64]).build();

        UiCore {
            events_loop,
            display,
            ui
        }
    }
}