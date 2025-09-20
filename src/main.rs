use cinder::core::application::CinderApplication;

mod core;

fn main() {
    let app = CinderApplication {
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
    };

    app.create();
}
