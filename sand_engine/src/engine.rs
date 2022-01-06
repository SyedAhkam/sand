use log::{info, debug};
use ndk::{
    looper::{ThreadLooper, Poll},
    event::{InputEvent, Keycode},
};

use std::os::unix::prelude::RawFd;

// First free number after ndk_glue::NDK_GLUE_LOOPER_INPUT_QUEUE_IDENT. This might be fragile.
const CUSTOM_EVENT_IDENT: i32 = ndk_glue::NDK_GLUE_LOOPER_INPUT_QUEUE_IDENT + 1;

pub struct Engine { 
    main_looper: ThreadLooper,
    is_running: bool
} 

impl Engine {
    pub fn new() -> Self {
        Self {
            main_looper: ThreadLooper::for_thread().expect("main looper not attached"),
            is_running: true
        }
    }

    fn handle_event_pipe(&self) {
        info!(
            "Event pipe yields: {:?}",
            ndk_glue::poll_events()
                .expect("Looper says event-pipe has data available!")
        )

    }
    fn handle_input(&mut self) {
        let input_queue = ndk_glue::input_queue();
        let input_queue = input_queue.as_ref().expect("Input queue not attached");

        assert!(input_queue.has_events().unwrap());

        while let Some(event) = input_queue.get_event() {
            if let Some(event) = input_queue.pre_dispatch(event) {
                info!("Input event: {:?}", event);

                let mut event_did_handle = false;
                match &event {
                    InputEvent::KeyEvent(key_event) => {
                        debug!("KeyEvent: {:?}", key_event);
                    },
                    InputEvent::MotionEvent(motion_event) => {
                        debug!("MotionEvent: {:?}", motion_event);
                    }
                };
                input_queue.finish_event(event, event_did_handle);
            }
        }
    }

    fn dispatch_event(&mut self, ident: i32, fd: RawFd) {
        match ident {
            ndk_glue::NDK_GLUE_LOOPER_EVENT_PIPE_IDENT => self.handle_event_pipe(),
            ndk_glue::NDK_GLUE_LOOPER_INPUT_QUEUE_IDENT => self.handle_input(),
            _ => {}
        }
    }

    fn setup(&self) {
        debug!("Setup function");
    }

    fn run(&mut self) {
        while self.is_running {
            match self.main_looper.poll_all().unwrap() {
                Poll::Wake => {},
                Poll::Callback => { unreachable!() },
                Poll::Timeout => { unreachable!() },
                Poll::Event{
                    ident,
                    fd,
                    events: _,
                    data: _, // might need later
                } => self.dispatch_event(ident, fd)
            }
        };
    }

    pub fn start(&mut self) {
        self.setup();
        self.run()
    }
}
