
use crate::{Error, Node, Path};
pub type Matcher = fn(&Path, &Node) -> bool;
pub type ErrorHandler = fn(&Path, Error) -> Option<Error>;
pub type MaxDepth = usize;
pub type Depth = usize;

pub trait WalkProgressHandler: Send + Sync + 'static {
    fn path_matching(&mut self, path: &Path, node: &Node) -> bool;
    fn error(&mut self, _path_: &Path, _exception_: Error) -> Option<Error> {
        None
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct NoopProgressHandler;

impl WalkProgressHandler for NoopProgressHandler {
    fn path_matching(&mut self, _p: &Path, _n: &Node) -> bool {
        true
    }
}
