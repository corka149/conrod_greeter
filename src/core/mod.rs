use conrod::backend::glium::glium;
use conrod::backend::glium::{glium::Surface, Renderer};
use conrod::{self, image};

pub struct UiCore {
    pub events_loop: glium::glutin::EventsLoop,
    pub display: glium::Display,
    pub ui: conrod::Ui,
    pub image_map: image::Map<glium::texture::Texture2d>,
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
        let mut ui = conrod::UiBuilder::new([init_width as f64, init_height as f64]).build();

        // Load fonts
        const FONT_PATH: &'static str = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/NotoSans/NotoSans-Regular.ttf"
        );
        ui.fonts.insert_from_file(FONT_PATH).unwrap();

        // Create empty graphics map because it is a mandatory parameter
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        UiCore {
            events_loop,
            display,
            ui,
            image_map,
        }
    }

    pub fn draw_if_changed(&self, renderer: &mut Renderer) {
        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = self.ui.draw_if_changed() {
            renderer.fill(&self.display, primitives, &self.image_map);

            // starts drawing on the backbuffer
            let mut target = self.display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
