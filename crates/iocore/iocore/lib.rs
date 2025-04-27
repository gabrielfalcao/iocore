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
//!⣿⠁⢸⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿                     _/_/_/          _/_/        _/
//!⣿⠀⢈⣉⣉⣉⣉⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿        _/      _/        _/      _/    _/    _/_/
//!⣿⡀⠀⠈⠛⠿⢿⣿⣷⣶⣤⣤⣤⣄⣀⣠⣤⣄⡉⠻⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿       _/      _/    _/_/        _/    _/      _/
//!⣿⣇⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠛⠛⠛⠛⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿        _/  _/          _/      _/    _/      _/
//!⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿         _/      _/_/_/    _/    _/_/    _/  _/
//!⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿
//!⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿
//!⣿⣿⣿⣿⣿⣿⣷⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⣿⣿⣿⣿⣿⣿
//!⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣦⣤⣤⣤⣤⣴⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
//!```
//!
//!
#![cfg(target_family = "unix")]
#[macro_use]
extern crate lazy_static;
pub mod env;
pub(crate) mod errors;
pub(crate) mod fs;
pub(crate) mod sh;
pub(crate) mod sys;
pub(crate) mod walk;

pub use env::{args, args_from_string, var};
pub use errors::{Error, Result};
pub use fs::ls_path_type::LsPathType;
pub use fs::opts::OpenOptions;
pub use fs::path_datetime::PathDateTime;
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
pub use fs::{Path, ROOT_PATH_STR, USERS_PATH};
pub use sh::{
    shell_command, shell_command_stdout, shell_command_string_output, shell_command_vec_output,
};
pub use sys::{
    Group, User, XPC, best_guess_home, get_stdout_string, get_subprocess_output, guess_unix_home,
    parse_u32, safe_string, unix_user_info_home,
};
pub use walk::{Depth, NoopProgressHandler, WalkProgressHandler, glob, walk_dir, walk_globs};

lazy_static! {
    pub static ref XPC_INFO: XPC = XPC::from_env();
    pub static ref USER: User = User::id().unwrap_or_default();
    pub static ref TILDE: String = format!("{}/", USER.home().expect("current UNIX user HOME"));
}
