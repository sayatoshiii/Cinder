use cinder::core::application::{CinderApplication, CinderApplicationOptions};
use winit::window::Window;

mod core;

fn main() {
    let attributes = Window::default_attributes().with_title("test");

    let app = CinderApplication {
        options: CinderApplicationOptions {
            window: attributes,
            icon: None,
        },
        render: Some(Box::new(|window, surface| {
            let size = window.inner_size();
            let width = size.width;
            let height = size.height;

            println!("Window:\n  Width: {},\n  Height: {}", width, height);

            let mut buffer = surface.buffer_mut().unwrap();
            for pixel in buffer.iter_mut() {
                *pixel = 0x1fd1b6;
            }
            buffer.present().unwrap();
        })),
        input: None,
    };

    app.create();
}
