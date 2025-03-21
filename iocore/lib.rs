//!
//!
//!
//!
//!```c
//!⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠟⠛⠛⠛⠛⠻⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿                  ___ ___   ___
//!⣿⣿⣿⣿⣿⣿⡿⠟⣋⣡⣴⣶⣾⣿⡇⢸⣶⣄⠀⠀⠈⠙⠻⢿⣿⣿⣿⣿⣿⣿                 |_ _/ _ \ / __|___ _ _ ___
//!⣿⣿⣿⣿⡿⢋⣴⣾⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⣆⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿                  | | (_) | (__/ _ \ '_/ -_)
//!⣿⣿⣿⠏⣰⣿⣿⣿⣿⣿⣿⡿⠟⠛⠃⠘⢿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿                 |___\___/ \___\___/_| \___|
//!⣿⣿⠏⣸⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠈⢿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿
//!⣿⡏⢠⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿
//!⣿⠁⢸⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿     _/_/        _/    _/_/      _/    _/_/          _/_/
//!⣿⠀⢈⣉⣉⣉⣉⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿  _/    _/    _/_/  _/    _/  _/_/  _/    _/      _/    _/
//!⣿⡀⠀⠈⠛⠿⢿⣿⣷⣶⣤⣤⣤⣄⣀⣠⣤⣄⡉⠻⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿ _/    _/      _/  _/    _/    _/  _/    _/      _/    _/
//!⣿⣇⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠛⠛⠛⠛⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿_/    _/      _/  _/    _/    _/  _/    _/      _/    _/
//!⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿ _/_/    _/  _/    _/_/      _/    _/_/    _/    _/_/
//!⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿
//!⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿
//!⣿⣿⣿⣿⣿⣿⣷⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⣿⣿⣿⣿⣿⣿
//!⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣦⣤⣤⣤⣤⣴⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//!//!```
//!
//!
//![![Continuous Integration](https://github.com/gabrielfalcao/iocore/actions/workflows/ci.yml/badge.svg)](https://github.com/gabrielfalcao/iocore/actions/workflows/ci.yml)
#![cfg(target_family = "unix")]
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

pub use coreio::{
    absolute_path, absolutely_current_path, canonical_path, ensure_dir_exists, expand_path,
    get_or_create_parent_dir, open_append, open_read, read_file, read_file_bytes, resolved_path,
    write_file,
};
pub use env::{args, var};
pub use errors::{Error, Result};
pub use fs::errors::{FileSystemError, FileSystemException};
pub use fs::ls_node_type::LsNodeType;
pub use fs::node::Node;
pub use fs::opts::OpenOptions;
pub use fs::path_status::PathStatus;
pub use fs::path_timestamps::PathTimestamps;
pub use fs::path_type::PathType;
pub use fs::path_utils::{
    add_trailing_separator, expand_home_regex, path_str_to_relative_subpath, remove_absolute_path,
    remove_duplicate_separators, remove_end, remove_equal_prefix_from_path_strings,
    remove_redundant_current_path, remove_start, remove_trailing_slash, repl_beg, repl_end,
    split_str_into_relative_subpath_parts,
};
pub use fs::perms::PathPermissions;
pub use fs::size::{ByteUnit, Size};
pub use fs::timed::PathDateTime;
pub use fs::Path;
pub use io::buffer::FileBuffer;
pub use io::error::Code;
pub use sh::{shell_command, shell_command_string_output, shell_command_vec_output};
pub use sys::{
    best_guess_home, get_stdout_string, get_subprocess_output, guess_unix_home, parse_u32,
    safe_string, Group, User,
};
pub use walk::entry::Entry;
pub use walk::info::Info;
pub use walk::t::{NoopProgressHandler, WalkProgressHandler};
pub use walk::{glob, read_dir, read_dir_size, walk_dir, walk_nodes};

lazy_static! {
    pub static ref USER: User = User::id().unwrap_or_default();
    pub static ref TILDE: String = format!("{}/", USER.home().expect("current UNIX user HOME"));
}
