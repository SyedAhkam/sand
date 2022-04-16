use log::{info, debug};
use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput},
    window::Window,
};

use sand_renderer::SkiaRenderer;

use crate::input::InputHandler;

pub fn quit_activity() {
    ndk_glue::native_activity().finish();
}

pub struct Engine {
    event_loop: EventLoop<()>,
    window: Window,
    renderer: SkiaRenderer,
    input_handler: InputHandler
}

impl Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder::new()
    }

    pub fn run(self) {
        // Run the event loop
        self.event_loop.run(move |event, _target, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {input, ..} => self.input_handler.handle_event(input),
                    _ => debug!("{:#?}", event),
                },
                Event::RedrawRequested(_) => {
                    //println!("\nredrawing!\n");
                }
                _ => (),
            }
        });
    }
    
    pub fn start(self) {
        self.run();
    }
}

pub struct EngineBuilder { 
    renderer: Option<SkiaRenderer>,
    input_handler: Option<InputHandler>
} 

impl EngineBuilder {
    pub fn new() -> Self {
        Self { renderer: None, input_handler: None }
    }

    pub fn with_renderer(mut self, renderer: SkiaRenderer) -> Self {
        self.renderer = Some(renderer);
        self
    }

    pub fn with_input_handler(mut self, input_handler: InputHandler) -> Self {
        self.input_handler = Some(input_handler);
        self
    }

    pub fn build(self) -> Engine {
        // Create the event loop and window
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).expect("failed to create window");

        Engine {
            event_loop,
            window,
            renderer: self.renderer.expect("renderer not attached"),
            input_handler: self.input_handler.expect("input handler not attached")
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine::builder()
            .with_renderer(SkiaRenderer::new())
            .with_input_handler(InputHandler::new())
            .build()
    }
}
