use winit::{event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent, KeyboardInput, ElementState}};

use crate::Context;

pub struct App {
    event_loop: EventLoop<()>,
    pub context: Context
}
impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        Self {
            context: Context::new(&event_loop),
            event_loop
        }
    }
    pub fn run(self) {
        let c = self.context;
        c.setup();
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(key),
                            state: ElementState::Pressed, ..
                        }, ..
                    } => c.key_pressed(key),
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(key),
                            state: ElementState::Released, ..
                        }, ..
                    } => c.key_released(&key),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_size) => c.resize(new_size),
                    _ => {}
                },
                Event::MainEventsCleared => c.request_redraw(),
                Event::RedrawRequested(_) => {
                    if !c.is_running() { return *control_flow = ControlFlow::Exit }
                    c.update()
                },
                _ => {}
            }
        })
    }
}