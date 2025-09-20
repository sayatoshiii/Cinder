use std::num::NonZeroU32;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};

use crate::core::application::gui::windows::types::WinitAppBuilder;
use crate::core::application::gui::windows::winit::{make_window, run_app};
use crate::core::graphics::colour::types::Colour;

mod core;

fn main() {
    entry(EventLoop::new().unwrap())
}

pub(crate) fn entry(event_loop: EventLoop<()>) {
    let context = softbuffer::Context::new(event_loop.owned_display_handle()).unwrap();

    let app = WinitAppBuilder::with_init(
        |elwt| make_window(elwt, |w| w),
        move |_elwt, window| softbuffer::Surface::new(&context, window.clone()).unwrap(),
    )
    .with_event_handler(|window, surface, event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(size),
            } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("Resized fired before Resumed or after Suspended");
                    return;
                };

                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    surface.resize(width, height).unwrap();
                }
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let size = window.inner_size();
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    let mut buffer = surface.buffer_mut().unwrap();

                    let colour = Colour::from_hex("#1fd1b6");

                    let size = 100;
                    let pos_x = (width.get() / 2).saturating_sub(size / 2);
                    let pos_y = (height.get() / 2).saturating_sub(size / 2);

                    for y in pos_y..(pos_y + size) {
                        if y >= height.get() {
                            break;
                        }
                        for x in pos_x..(pos_x + size) {
                            if x >= width.get() {
                                break;
                            }
                            let idx = (y * width.get() + x) as usize;
                            buffer[idx] = colour.bitwise();
                        }
                    }

                    buffer.present().unwrap();
                }
            }
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    },
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    });

    run_app(event_loop, app);
}
