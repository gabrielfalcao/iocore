use crate::{Error, Path};
pub type Matcher = fn(&Path) -> bool;
pub type ErrorHandler = fn(&Path, Error) -> Option<Error>;
pub type MaxDepth = usize;
pub type Depth = usize;

/// `WalkProgressHandler` trait defines a protocol outlining the
/// behavior of [`iocore::walk::walk_dir`] in terms of whether to
/// aggregate paths in the final result, whether to scan a directory
/// and whether an error should cause [`iocore::walk::walk_dir`] to
/// fail.
pub trait WalkProgressHandler: Send + Sync + 'static + Clone {
    /// `path_matching` is called to determine whether
    /// [`iocore::walk::walk_dir`] should aggregate the given `path`
    /// argument in its final result.
    ///
    /// If the implementor returns [`std::result::Result::Err`] then
    /// the error is handled by [`WalkProgressHandler::error`] which
    /// cascades the error all the way up to the initial call if
    /// [`Some(error)`] is returned.
    ///
    /// If the implementor returns [`Ok(false)`] the given `path` will
    /// not be aggregated in the final result.
    fn path_matching(&mut self, path: &Path) -> std::result::Result<bool, Error>;

    /// `should_scan_directory` is only called when `path` argument is a directory.
    ///
    /// Implementors return [`Ok(false)`] to indicate that
    /// [`iocore::walk::walk_dir`] shall skip scanning directory.
    ///
    /// If the implementor returns [`std::result::Result::Err`] then
    /// the error is handled by [`WalkProgressHandler::error`] which
    /// cascades the error all the way up to the initial call if
    /// [`Some(error)`] is returned.
    ///
    /// Default implementation always returns [`Ok(true)`].
    ///
    ///
    /// > NOTE: [`iocore::walk::walk_dir`] spawns (i.e.:
    /// > [`thread_groups::ThreadGroup::spawn`]) a sub thread calling
    /// > [`iocore::walk::walk_dir`] (in assynchronously recursively
    /// > fashion) with the directory referenced in the `path` argument
    /// > which is then "joined" (via
    /// > [`thread_groups::ThreadGroup::results`]) at the end of each
    /// > `walk_dir` function.
    fn should_scan_directory(&mut self, path: &Path) -> std::result::Result<bool, Error> {
        Ok(path.is_directory())
    }
    /// `error` is called when [`Err(iocore::Error)`] arises anywhere
    /// within a [`iocore::walk::walk_dir`] call so that implementors
    /// can choose how to handle errors.
    ///
    /// Default implementation always returns [`Some(error)`].
    fn error(&mut self, _path_: &Path, error: Error) -> Option<Error> {
        Some(error)
    }
}

/// `NoopProgressHandler` is the builtin implementation of
/// [`iocore::WalkProgressHandler`] which aggregates results insofar
/// as the `path` given to `path_matching` exists at the moment the
/// calling thread calls it.
#[derive(Clone, Eq, PartialEq)]
pub struct NoopProgressHandler;
pub type WalkDirDepth = usize;
impl WalkProgressHandler for NoopProgressHandler {
    fn path_matching(&mut self, path: &Path) -> std::result::Result<bool, Error> {
        Ok(path.exists())
    }

    fn should_scan_directory(&mut self, path: &Path) -> std::result::Result<bool, Error> {
        Ok(path.is_directory())
    }
}
