use log::{info, debug};
use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
};

pub struct Engine { 
    event_loop: EventLoop<()>,
    window: Window,
} 

fn quit_activity() {
    ndk_glue::native_activity().finish();
}

impl Engine {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();

        Self {
            event_loop,
            window,
        }
    }

    fn run(self) {
        let Self {
            mut event_loop,
            window,
            ..
        } = self;
        

        // Run the event loop
        event_loop.run(move |event, _target, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        debug!("The close button was pressed; stopping");

                        *control_flow = ControlFlow::Exit;
                    },
                    WindowEvent::KeyboardInput{ 
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(vkeycode),
                                state: winit::event::ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        if vkeycode == VirtualKeyCode::Back {
                            quit_activity();
                        }
                    },
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
