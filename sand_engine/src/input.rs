use log::{debug, warn};
use winit::event::{KeyboardInput, VirtualKeyCode, ElementState};

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn on_press(&self, vkey: Option<VirtualKeyCode>) {
        match vkey {
            //Some(VirtualKeyCode::Back) => {
                //debug!("bekk");
                //crate::engine::quit_activity();
            //}
            _ => { debug!("vkey: {:#?}", vkey) }
        }
    }

    pub fn on_release(&self, _vkey: Option<VirtualKeyCode>) {}

    pub fn handle_event(&self, ev: KeyboardInput) {
        debug!("{:#?}", ev);
        let KeyboardInput { virtual_keycode, state, .. } = ev;

        match state {
            ElementState::Pressed => self.on_press(virtual_keycode),
            ElementState::Released => self.on_release(virtual_keycode)
        }
    }
}
