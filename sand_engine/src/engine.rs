use log::{info, debug};
use ndk::{
    looper::{ThreadLooper, Poll, FdEvent},
    event::{InputEvent, Keycode},
};

use std::mem::MaybeUninit;
use std::os::unix::prelude::RawFd;

// First free number after ndk_glue::NDK_GLUE_LOOPER_INPUT_QUEUE_IDENT. This might be fragile.
const CUSTOM_EVENT_IDENT: i32 = ndk_glue::NDK_GLUE_LOOPER_INPUT_QUEUE_IDENT + 1;
const U32_SIZE: usize = std::mem::size_of::<u32>();

fn create_fd_pipe() -> [RawFd; 2] {
    let mut ends = MaybeUninit::<[RawFd; 2]>::uninit();

    assert_eq!(unsafe { libc::pipe(ends.as_mut_ptr().cast()) }, 0);
    unsafe { ends.assume_init() }
}

pub struct Engine { 
    main_looper: ThreadLooper,
    is_running: bool,
    custom_event_pipe: Option<[RawFd; 2]>,
    custom_callback_pipe: Option<[RawFd; 2]>
} 

impl Engine {
    pub fn new() -> Self {
        Self {
            main_looper: ThreadLooper::for_thread().expect("main looper not attached"),
            is_running: true,
            custom_event_pipe: None,
            custom_callback_pipe: None
        }
    }

    fn handle_custom_event(&self, fd: RawFd) {
        let mut recv = !0u32;
        assert_eq!(
            unsafe { libc::read(fd, &mut recv as *mut _ as *mut _, U32_SIZE) } as usize,
            U32_SIZE
        );

        info!("Read custom event from pipe: {}", recv);
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
            CUSTOM_EVENT_IDENT => self.handle_custom_event(fd),
            i => panic!("Unexpected event identifier: {}", i)
        }
    }

    fn setup(&mut self) {
        // Setup file descriptors
        self.custom_event_pipe = Some(create_fd_pipe());
        self.custom_callback_pipe = Some(create_fd_pipe());

        self.main_looper
            .as_foreign()
            .add_fd(
                self.custom_event_pipe.unwrap()[0],
                CUSTOM_EVENT_IDENT,
                FdEvent::INPUT,
                std::ptr::null_mut(),
            )
            .expect("Failed to add file descriptor to Looper");

        self.main_looper
            .as_foreign()
            .add_fd_with_callback(self.custom_callback_pipe.unwrap()[0], FdEvent::INPUT, |fd| {
                let mut recv = !0u32;

                assert_eq!(
                    unsafe { libc::read(fd, &mut recv as *mut _ as *mut _, U32_SIZE) } as usize,
                    U32_SIZE
                );

                info!("Read custom event from pipe, in callback: {}", recv);

                true
            })
            .expect("Failed to add file descriptor to Looper");
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
        self.run();
    }
}
