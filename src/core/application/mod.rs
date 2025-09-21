use std::fs::read;
use std::num::NonZeroU32;
use std::rc::Rc;

use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes},
};

use crate::core::application::gui::windows::{
    types::WinitAppBuilder,
    winit::{make_window, run_app},
};

pub mod gui;

pub struct CinderApplication {
    pub options: CinderApplicationOptions,
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

#[derive(Debug, Clone)]
pub struct CinderApplicationOptions {
    pub window: WindowAttributes,
    pub icon: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CinderWindowAttributes {
    pub title: Option<String>,
}

impl CinderApplication {
    #[allow(dead_code)]
    pub fn setup_window_attributes(attributes: CinderWindowAttributes) -> WindowAttributes {
        Window::default_attributes()
            .with_title(attributes.title.unwrap_or("Cinder Application".to_string()))
    }

    #[allow(dead_code)]
    pub fn create(self: Self) {
        let event_loop = EventLoop::new().unwrap();
        let context = softbuffer::Context::new(event_loop.owned_display_handle()).unwrap();

        let attributes = self.options.window;
        let icon = self.options.icon;

        let app = WinitAppBuilder::with_init(
            move |elwt| {
                let window = make_window(elwt, |_| attributes.clone());

                if let Some(icon) = &icon {
                    let icon_buffer =
                        read(icon).expect(&format!("Could not find file @ '{}'", icon));
                    let icon_image = image::load_from_memory(&icon_buffer).unwrap().into_rgba8();

                    let (width, height) = icon_image.dimensions();
                    let icon_raw_rgba = icon_image.into_raw();

                    window.set_window_icon(Some(
                        winit::window::Icon::from_rgba(icon_raw_rgba, width, height).unwrap(),
                    ));
                };

                window
            },
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
