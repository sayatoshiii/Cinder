use std::num::NonZeroU32;
use std::rc::Rc;

use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
};

use crate::core::application::gui::windows::{
    types::WinitAppBuilder,
    winit::{make_window, run_app},
};

pub mod gui;

pub struct CinderApplication {
    pub render: Option<
        Box<
            dyn Fn(
                &mut Rc<winit::window::Window>,
                &mut softbuffer::Surface<
                    winit::event_loop::OwnedDisplayHandle,
                    Rc<winit::window::Window>,
                >,
            ),
        >,
    >,
}

impl CinderApplication {
    #[allow(dead_code)]
    pub fn create(self: Self) {
        let event_loop = EventLoop::new().unwrap();

        let context = softbuffer::Context::new(event_loop.owned_display_handle()).unwrap();

        let app = WinitAppBuilder::with_init(
            |elwt| make_window(elwt, |w| w),
            move |_elwt, window| softbuffer::Surface::new(&context, window.clone()).unwrap(),
        )
        .with_event_handler({
            let window_render = self.render;
            move |window, surface, event, elwt| {
                elwt.set_control_flow(ControlFlow::Poll);

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

                        if let Some(window_render) = &window_render {
                            window_render(window, surface)
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
            }
        });

        run_app(event_loop, app);
    }
}
