#![cfg(target_os = "android")]

// Re-exports
pub use sand_widgets::{
    Widget,
    StatelessWidget,
    BuildContext
};

pub(crate) use log::*;

pub fn run_app(app: impl StatelessWidget + std::fmt::Debug) {
    info!("Starting app: {:?}. Built using the Sand UI framework.", app)
}
