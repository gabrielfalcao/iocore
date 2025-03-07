//!
//!
//!
//!
//! ```c
//!  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠟⠛⠛⠛⠛⠻⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿                    ___ ___   ___
//!  ⣿⣿⣿⣿⣿⣿⡿⠟⣋⣡⣴⣶⣾⣿⡇⢸⣶⣄⠀⠀⠈⠙⠻⢿⣿⣿⣿⣿⣿⣿                   |_ _/ _ \ / __|___ _ _ ___
//!  ⣿⣿⣿⣿⡿⢋⣴⣾⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣆⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿                    | | (_) | (__/ _ \ '_/ -_)
//!  ⣿⣿⣿⠏⣰⣿⣿⣿⣿⣿⣿⡿⠟⠛⠃⠘⢿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿                   |___\___/ \___\___/_| \___|
//!  ⣿⣿⠏⣸⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠈⢿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿
//!  ⣿⡏⢠⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿
//!  ⣿⠁⢸⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿                    _/_/        _/    _/_/      _/_/      _/_/        _/
//!  ⣿⠀⢈⣉⣉⣉⣉⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿     _/      _/  _/    _/    _/_/  _/    _/  _/    _/  _/    _/    _/_/
//!  ⣿⡀⠀⠈⠛⠿⢿⣿⣷⣶⣤⣤⣤⣄⣀⣠⣤⣄⡉⠻⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿    _/      _/  _/    _/      _/  _/    _/  _/    _/  _/    _/      _/
//!  ⣿⣇⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠛⠛⠛⠛⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿     _/  _/    _/    _/      _/  _/    _/  _/    _/  _/    _/      _/
//!  ⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿      _/        _/_/    _/  _/    _/_/      _/_/      _/_/    _/  _/
//!  ⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿
//!  ⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿
//!  ⣿⣿⣿⣿⣿⣿⣷⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⣿⣿⣿⣿⣿⣿
//!  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣦⣤⣤⣤⣤⣴⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//! ```
//!
//!
//!
//!
//! [![IOՑ𐔙](https://github.com/gabrielfalcao/iocore/actions/workflows/zap.yml/badge.svg)](https://github.com/gabrielfalcao/iocore/actions/workflows/zap.yml)
//!
//! #### IOՑ𐔙
//! ![iocore/IOCORE.png?raw=true](iocore/IOCORE.png?raw=true "&#x13ba;&#x551;&#x10519;").
//!
//!
//! ```bash
//! cargo add iocore
//! ```
//!
#![cfg(target_family = "unix")]
#![feature(io_error_more, thread_id_value, io_error_inprogress)]
#[macro_use]
extern crate lazy_static;
pub mod coreio;
pub mod env;
pub mod errors;
pub mod fs;
pub mod io;
pub mod sh;
pub mod sys;
pub mod walk;

pub use coreio::*;
pub use env::*;
pub use errors::*;
pub use fs::*;
pub use io::*;
pub use sh::*;
pub use sys::*;
pub use walk::*;

lazy_static! {
    static ref TILDE: String =
        format!("{}/", home().unwrap_or(String::from("~")).trim_end_matches('/'));
}
