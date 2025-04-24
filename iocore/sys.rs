use std::process::{Command, Stdio};
use std::str::FromStr;

use regex::Regex;
use sanitation::{from_hex, SBoolean, SString};

use crate::{env_var, Error};

pub const DEFAULT_UID: u32 = if cfg!(target_os = "macos") { 501 } else { 1001 };

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    pub gid: u32,
    pub name: String,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    pub uid: u32,
    pub gid: Option<u32>,
    pub name: String,
    pub group: Option<String>,
    pub groups: Vec<Group>,
}

impl User {
    pub fn id() -> Result<User, Error> {
        let stdout = get_stdout_string("/usr/bin/id")?;
        Ok(User::from_id_cmd_string(stdout)?)
    }

    pub fn from_env() -> User {
        let uid = env_var_uid().unwrap_or_else(|_| DEFAULT_UID);
        let name = env_var_user();
        User {
            uid,
            name,
            group: None,
            gid: None,
            groups: Vec::new(),
        }
    }

    pub fn from_id_cmd_string(id_stdout: impl std::fmt::Display) -> Result<User, Error> {
        let stdout = id_stdout.to_string();
        let uexpr = Regex::new(
            r"uid=(?<uid>\d+)([(](?<name>[^)]+)[)])\s*gid=(?<gid>\d+)[(](?<group>[^)]+)[)]",
        )
        .unwrap();
        let gexpr = Regex::new(r"(\d+)([(]([^)]+)[)])").unwrap();
        if let Some(captures) = uexpr.captures(&stdout) {
            let gid = parse_u32(captures.name("gid").map(|s| s.as_str()).unwrap(), "gid").unwrap();
            let uid = parse_u32(captures.name("uid").map(|s| s.as_str()).unwrap(), "uid").unwrap();
            let name = captures.name("name").map(|s| s.as_str().to_string()).unwrap();
            let group = captures.name("group").map(|s| s.as_str().to_string()).unwrap();
            let groups = gexpr
                .captures_iter(&stdout)
                .map(|c| c.extract::<3>())
                .map(|(_n_, m)| {
                    let g = m.iter().map(|n| n.to_string()).collect::<Vec<String>>();
                    let gid = parse_u32(&g[0], "gid").unwrap();
                    let name = g[2].to_string();
                    Group { gid, name }
                })
                .collect::<Vec<Group>>();
            Ok(User {
                uid,
                gid: Some(gid),
                name,
                group: Some(group),
                groups,
            })
        } else {
            Err(Error::SystemError(format!(
                "could not secure user information from /usr/bin/id"
            )))
        }
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> Option<u32> {
        self.gid.clone()
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn home(&self) -> Result<String, Error> {
        let user = self.name();
        let uid = self.uid();
        Ok(unix_user_info_home("/etc/passwd", &user, uid)
            .map_err(|e| {
                Error::SystemError(format!(
                    "User::home():{} failed call to unix_user_info_home: {:#?}",
                    line!(),
                    e
                ))
            })
            .or_else(|_| {
                env_var_home(&user, uid, None).map_err(|e| {
                    Error::SystemError(format!(
                        "User::home():{} failed call to env_var_home: {:#?}",
                        line!(),
                        e
                    ))
                })
            })
            .or_else(|_| {
                best_guess_home(&user).map_err(|e| {
                    Error::SystemError(format!(
                        "User::home():{} failed call to best_guess_home: {:#?}",
                        line!(),
                        e
                    ))
                })
            })?
            .to_string())
    }
}

pub fn parse_u32(s: impl Into<String>, short_description: &str) -> Result<u32, Error> {
    let s = s.into();
    Ok(u32::from_str(&s).map_err(|e| {
        Error::SafetyError(format!("{} in converting {:#?} {:#?} to u32", e, s, short_description))
    })?)
}

pub fn get_subprocess_output(name: &str) -> Result<std::process::Output, Error> {
    Ok(Command::new(name)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| Error::SubprocessError(format!("failed to execute {:#?}: {}", name, e)))?)
}
pub fn get_stdout_string(executable: &str) -> Result<String, Error> {
    let (exit_code, stdout, stderr) =
        crate::sh::shell_command_string_output(executable, crate::fs::Path::cwd())?;
    if exit_code != 0 {
        return Err(Error::ShellCommandError(format!(
            "{:#?} failed with exit code {}{}",
            executable,
            exit_code,
            if stderr.len() > 0 { format!(": {:#?}", stderr) } else { String::new() }
        )));
    }
    safe_string(stdout.as_bytes(), &format!("stdout of {:#?}", executable))
}

pub fn safe_string(bytes: &[u8], short_description: &str) -> Result<String, Error> {
    Ok(SString::new(bytes)
        .safe()
        .map_err(|e| Error::SafetyError(format!("{} in converting {:#?}", e, short_description)))?)
}

pub fn unix_user_info_home(path: &str, name: &str, uid: u32) -> Result<String, Error> {
    for (n, line) in crate::Path::from(path).read_lines()?.iter().enumerate() {
        let location = format!("{}:{}", path, n + 1);
        if !line.starts_with(&format!("{}:", &name)) {
            continue;
        }

        let fields = line.split(':').into_iter().collect::<Vec<_>>();
        if fields[2] != uid.to_string() {
            return Err(Error::SystemError(format!(
                "unexpected uid in {:#?}: {} != {}",
                location, fields[2], uid
            )));
        }

        return Ok(path_owned_expectedly(
            crate::Path::raw(match fields.len() {
                7 => fields[5],
                10 => fields[7],
                e =>
                    return Err(Error::SystemError(format!(
                        "unexpected number of fields in {:#?} {}",
                        location, e
                    ))),
            }),
            name,
            uid,
        )?
        .to_string());
    }
    Err(Error::SystemError(format!(
        "home not found in {} for uid {} ({})",
        path, uid, name
    )))
}

fn env_var_home(user: &str, uid: u32, key: Option<String>) -> Result<String, Error> {
    let key = key.unwrap_or("HOME".to_string());
    Ok(path_owned_expectedly(
        crate::Path::directory(crate::env::var(&key)?).map_err(|e| {
            Error::SystemError(format!(
                "fetching home directory from environment variable {:#?}: {}",
                key, e
            ))
        })?,
        user,
        uid,
    )?
    .to_string())
}
fn env_var_uid() -> Result<u32, Error> {
    Ok(parse_u32(env_var!("UID"), "UID environment variable")?)
}
fn env_var_user() -> String {
    env_var!("USER")
}
fn path_owned_expectedly(path: crate::Path, user: &str, uid: u32) -> Result<crate::Path, Error> {
    if path.uid() == uid {
        Ok(path)
    } else {
        Err(Error::SystemError(format!(
            "{:#?} ain't owned by uid {} ({:#?})",
            path, uid, user
        )))
    }
}

pub fn guess_unix_home(user: impl Into<String>) -> Result<String, Error> {
    let user = user.into();
    use crate::fs::Path;

    let path = if cfg!(target_os = "macos") {
        format!("/Users/{}", &user)
    } else if cfg!(unix) {
        format!("/home/{}", &user)
    } else {
        return Err(Error::SystemError(format!(
            "windows, wasm and other non-unix platforms not supported"
        )));
    };

    if Path::raw(&path).is_dir() {
        Ok(path)
    } else {
        Err(Error::HomePathError(format!(
            "guessed unix user home {:#?} is not a folder",
            &path
        )))
    }
}

pub fn best_guess_home(user: impl Into<String>) -> Result<String, Error> {
    let user = user.into();
    use crate::fs::Path;
    Ok(if let Ok(home) = crate::env::var("HOME") {
        Path::directory(home.trim().to_string()).map(|p| p.to_string()).map_err(|e| {
            Error::SafetyError(format!(
                "environment variable HOME points to a non-accessible path {:#?}: {}",
                home, e
            ))
        })?
    } else {
        guess_unix_home(&user)?
    })
}
impl Default for User {
    fn default() -> User {
        User::from_env()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XPC {
    pub null_bootstrap: Option<bool>,
    pub flags: Option<Vec<u8>>,
    pub service_name: Option<String>,
}
impl XPC {
    pub fn from_env() -> XPC {
        let null_bootstrap = match crate::env::var("XPC_NULL_BOOTSTRAP") {
            Ok(nb) => match u8::from_str_radix(&nb, 10) {
                Ok(nb) => {
                    eprintln!("[warning] XPC_NULL_BOOTSTRAP environment variable is set");
                    Some(SBoolean::new(nb).value())
                },
                Err(_) => None,
            },
            Err(_) => None,
        };
        let flags = match crate::env::var("XPC_FLAGS") {
            Ok(flags) => match from_hex(&flags) {
                Ok(flags) => {
                    eprintln!("[warning] XPC_FLAGS environment variable is set");
                    Some(flags)
                },
                Err(_) => None,
            },
            Err(_) => None,
        };
        let service_name = match crate::env::var("XPC_SERVICE_NAME") {
            Ok(service_name) => Some(service_name),
            Err(_) => None,
        };
        XPC {
            null_bootstrap,
            flags,
            service_name,
        }
    }
}
