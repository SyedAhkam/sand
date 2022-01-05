#![cfg(target_os = "android")]

// Re-exports
pub use sand_widgets::{
    Widget,
    StatelessWidget,
    BuildContext
};

pub use ndk_glue::main;

pub fn run_app(app: impl StatelessWidget + std::fmt::Debug) {
    println!("I run: {:?}", app);

}
