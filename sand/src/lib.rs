#![cfg(target_os = "android")]

use log::info;

// Re-exports
pub use sand_widgets::{
    Widget,
    StatelessWidget,
    BuildContext
};

use sand_engine::Engine;

pub fn run_app(app: impl StatelessWidget + std::fmt::Debug) {
    info!("Starting app: {:?}. Built using the Sand UI framework.", app);

    // Let the engine take over
    Engine::new().start();

    // Finish activity when the main loop returns
    ndk_glue::native_activity().finish()
}
