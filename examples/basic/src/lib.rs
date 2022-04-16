use sand::{run_app, Widget, StatelessWidget, BuildContext};
use log::*;

#[derive(Debug)]
struct MyApp;

impl Widget for MyApp {
    fn build(&self, ctx: BuildContext) -> Box<dyn Widget> {
        unimplemented!()
    }
}

impl StatelessWidget for MyApp {}

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level="debug", tag="Basic"))
)]
fn main() {
    sand::run_app(MyApp);
}
