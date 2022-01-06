use crate::BuildContext;

pub trait Widget {
    fn build(&self, ctx: BuildContext) -> Box<dyn Widget>;
}
