//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\

use crate::errors::Error;
use crate::fs::{Node, Path};

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

#[derive(Clone)]
pub struct NoopProgressHandler;

impl WalkProgressHandler for NoopProgressHandler {
    fn path_matching(&mut self, _p: &Path, _n: &Node) -> bool {
        true
    }
}

// I1tkZXJpdmUoQ2xvbmUpXQpwdWIgc3RydWN0IEZuTXV0UHJvZ3Jlc3NIYW5kbGVyPEY6IFNlbmQgKyBTeW5jICsgJ3N0YXRpYyArIEZuTXV0KCZQYXRoLCAmTm9kZSkgLT4gYm9vbD4gewogICAgZm5tYXRjaDogRiwKfQppbXBsPEY+IEZuTXV0UHJvZ3Jlc3NIYW5kbGVyPEY6IFNlbmQgKyBTeW5jICsgJ3N0YXRpYyArIEZuTXV0KCZQYXRoLCAmTm9kZSkgLT4gYm9vbD4gewogICAgcHViIGZuIG5ldyhmbm1hdGNoOiBGKSAtPiBGbk11dFByb2dyZXNzSGFuZGxlciB7CiAgICAgICAgRm5NdXRQcm9ncmVzc0hhbmRsZXIgeyBmbm1hdGNoIH0KICAgIH0KfQppbXBsPEY+IFdhbGtQcm9ncmVzc0hhbmRsZXIgZm9yIEZuTXV0UHJvZ3Jlc3NIYW5kbGVyPEY+CndoZXJlCiAgICBGOiBGbk11dCgmUGF0aCwgJk5vZGUpIC0+IGJvb2wsCnsKICAgIGZuIHBhdGhfbWF0Y2hpbmcoJm11dCBzZWxmLCBwYXRoOiAmUGF0aCwgbm9kZTogJk5vZGUpIC0+IGJvb2wgewogICAgICAgIChzZWxmLmZubWF0Y2gpKHBhdGgsIG5vZGUpCiAgICB9Cn0K
