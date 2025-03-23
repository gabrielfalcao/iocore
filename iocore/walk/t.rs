use crate::{Error, Path};
pub type Matcher = fn(&Path) -> bool;
pub type ErrorHandler = fn(&Path, Error) -> Option<Error>;
pub type MaxDepth = usize;
pub type Depth = usize;

pub trait WalkProgressHandler: Send + Sync + 'static + Clone {
    fn path_matching(
        &mut self,
        path: &Path,
    ) -> std::result::Result<bool, Error>;
    fn error(&mut self, _path_: &Path, _exception_: Error) -> Option<Error> {
        None
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct NoopProgressHandler;
pub type WalkDirDepth = usize;
impl WalkProgressHandler for NoopProgressHandler {
    fn path_matching(
        &mut self,
        _path_: &Path,
    ) -> std::result::Result<bool, Error> {
        Ok(true)
    }
}
