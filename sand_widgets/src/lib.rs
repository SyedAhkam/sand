#[derive(Debug)]
pub struct BuildContext;

pub trait Widget {
    fn build(&self, ctx: BuildContext) -> Box<dyn Widget>;
}

pub trait StatelessWidget: Widget {}

pub trait StatefulWidget: Widget {}
