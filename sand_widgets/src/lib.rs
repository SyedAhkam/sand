mod widget;
mod stateless_widget;
mod stateful_widget;

// Re-exports
pub use widget::Widget;
pub use stateless_widget::StatelessWidget;
pub use stateful_widget::StatefulWidget;

#[derive(Debug)]
pub struct BuildContext;
