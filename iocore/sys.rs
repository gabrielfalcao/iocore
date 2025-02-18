//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\

use std::process::{Command, Stdio};
use std::str::FromStr;

use regex::Regex;
use sanitation::SString;

use crate::Exception;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    pub gid: u32,
    pub name: String,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    pub gid: u32,
    pub uid: u32,
    pub name: String,
    pub group: String,
    pub groups: Vec<Group>,
}

impl User {
    pub fn id() -> Result<User, Exception> {
        let stdout = get_stdout_string("/usr/bin/id")?;
        Ok(User::from_id_cmd_string(stdout)?)
    }

    pub fn from_id_cmd_string(id_stdout: impl std::fmt::Display) -> Result<User, Exception> {
        let stdout = id_stdout.to_string();
        let uexpr = Regex::new(
            r"uid=(?<uid>\d+)([(](?<name>[^)]+)[)])\s*gid=(?<gid>\d+)[(](?<group>[^)]+)[)]",
        )
        .unwrap();
        let gexpr = Regex::new(r"(\d+)([(]([^)]+)[)])").unwrap();
        if let Some(captures) = uexpr.captures(&stdout) {
            let gid = parse_u32(captures.name("gid").map(|s| s.as_str()).unwrap(), "gid")?;
            let uid = parse_u32(captures.name("uid").map(|s| s.as_str()).unwrap(), "uid")?;
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
                .collect::<Vec<Group>>(); //(|captures|captures.extract()).collect::<Vec<_>>(),
            Ok(User {
                uid,
                gid,
                name,
                group,
                groups,
            })
        } else {
            Err(Exception::SystemError(format!(
                "could not secure user information from /usr/bin/id"
            )))
        }
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn home(&self) -> Result<String, Exception> {
        let user = self.name();
        let uid = self.uid();
        Ok(unix_user_info_home("/etc/passwd", &user, uid)
            .or(env_var_home(&user, uid, None))
            .or(best_guess_home(&user))?
            .to_string())
    }
}

pub fn parse_u32(s: impl Into<String>, short_description: &str) -> Result<u32, Exception> {
    let s = s.into();
    Ok(u32::from_str(&s).map_err(|e| {
        Exception::SafetyError(format!(
            "{} in converting {:#?} {:#?} to u32",
            e, s, short_description
        ))
    })?)
}

pub fn get_subprocess_output(name: &str) -> Result<std::process::Output, Exception> {
    Ok(Command::new(name)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| Exception::SubprocessError(format!("failed to execute {:#?}: {}", name, e)))?)
}
pub fn get_stdout_string(executable: &str) -> Result<String, Exception> {
    let output = get_subprocess_output(executable)?;
    safe_string(&output.stdout, &format!("stdout of {:#?}", executable))
}

pub fn env_var(key: impl Into<String>) -> Result<String, Exception> {
    let key = key.into();
    Ok(std::env::var(&key).map_err(|e| {
        Exception::EnvironmentVarError(format!("fetching environment variable {:#?}: {}", &key, e))
    })?)
}

pub fn safe_string(bytes: &[u8], short_description: &str) -> Result<String, Exception> {
    Ok(SString::new(bytes).safe().map_err(|e| {
        Exception::SafetyError(format!("{} in converting {:#?}", e, short_description))
    })?)
}

fn unix_user_info_home(path: &str, name: &str, uid: u32) -> Result<String, Exception> {
    for (n, line) in crate::Path::from(path).read_lines()?.iter().enumerate() {
        let location = format!("{}:{}", path, n + 1);
        if !line.starts_with(&format!("{}:", &name)) {
            continue;
        }

        let fields = line.split(':').into_iter().collect::<Vec<_>>();
        if fields[2] != uid.to_string() {
            return Err(Exception::SystemError(format!(
                "unexpected uid in {:#?}: {} != {}",
                location, fields[2], uid
            )));
        }

        return Ok(path_owned_expectedly(
            crate::Path::directory(match fields.len() {
                7 => fields[6],
                10 => fields[8],
                e =>
                    return Err(Exception::SystemError(format!(
                        "unexpected number of fields in {:#?} {}",
                        location, e
                    ))),
            })?,
            name,
            uid,
        )?
        .to_string());
    }
    Err(Exception::SystemError(format!(
        "home not found in {} for uid {} ({})",
        path, uid, name
    )))
}

fn env_var_home(user: &str, uid: u32, key: Option<String>) -> Result<String, Exception> {
    let key = key.unwrap_or("HOME".to_string());
    Ok(path_owned_expectedly(
        crate::Path::directory(env_var(&key)?).map_err(|e| {
            Exception::SystemError(format!(
                "fetching home directory from environment variable {:#?}: {}",
                key, e
            ))
        })?,
        user,
        uid,
    )?
    .to_string())
}

fn path_owned_expectedly(
    path: crate::Path,
    user: &str,
    uid: u32,
) -> Result<crate::Path, Exception> {
    if path.node().uid == uid {
        Ok(path)
    } else {
        Err(Exception::SystemError(format!(
            "{:#?} ain't owned by uid {} ({:#?})",
            path, uid, user
        )))
    }
}

pub fn guess_unix_home(user: impl Into<String>) -> Result<String, Exception> {
    let user = user.into();
    use crate::fs::Path;
    let candidates = vec![format!("/home/{}", &user), format!("/Users/{}", &user)];

    for path in candidates.iter().map(|p| Path::directory(p).ok()) {
        if path.is_some() {
            return Ok(path.unwrap().to_string());
        }
    }
    return Err(Exception::HomePathError(format!(
        "neither paths seem to be home of user {:#?}",
        &user
    )));
}

pub fn best_guess_home(user: impl Into<String>) -> Result<String, Exception> {
    let user = user.into();
    use crate::fs::Path;
    Ok(if let Ok(home) = env_var("HOME") {
        Path::directory(home.trim().to_string()).map(|p| p.to_string()).map_err(|e| {
            Exception::SafetyError(format!(
                "environment variable HOME points to a non-accessible path {:#?}: {}",
                home, e
            ))
        })?
    } else {
        guess_unix_home(&user)?
    })
}
pub fn home() -> Result<String, Exception> {
    let user = if let Ok(user) = env_var("USER") {
        user.trim().to_string()
    } else {
        get_stdout_string("/usr/bin/whoami")?.trim().to_string()
    };

    let uid = if let Ok(uid_s) = env_var("UID") {
        u32::from_str(&uid_s).map_err(|_| {
            Exception::SafetyError(format!(
                "environment variable UID holds a non-numeric value: {:#?}",
                uid_s
            ))
        })?
    } else if let Ok(best_guess_path) = best_guess_home(user) {
        return Ok(best_guess_path);
    } else {
        return Err(Exception::HomePathError(format!("could not secure home path from neither HOME environment variable nor guess based on UID environment variable")));
    };

    unix_user_info_home("/etc/passwd", &user, uid)
        .or(env_var_home(&user, uid, None))
        .or(best_guess_home(&user))
}

#[cfg(test)]
mod test {
    use crate::exceptions::*;
    use crate::sys::*;

    #[test]
    fn test_home() {
        assert!(home().unwrap().contains(&env_var("USER").unwrap()));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_guess_unix_home_macosx() {
        let user = env_var("USER").unwrap();
        assert_eq!(guess_unix_home(&user).unwrap(), format!("/Users/{}", &user))
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_guess_unix_home_linux() {
        let user = env_var("USER").unwrap();
        assert_eq!(guess_unix_home(&user).unwrap(), format!("/home/{}", &user))
    }

    #[test]
    fn test_user_from_id_cmd_string() -> Result<()> {
        let stdout = format!("uid=501(name) gid=20(group) groups=20(group),101(access_bpf),12(everyone),61(localaccounts),79(_appserverusr),80(admin),81(_appserveradm),98(_lpadmin),701(com.apple.sharepoint.group.1),702(com.apple.sharepoint.group.2),33(_appstore),100(_lpoperator),204(_developer),250(_analyticsusers),395(com.apple.access_ftp),398(com.apple.access_screensharing)");
        let user = User::from_id_cmd_string(stdout)?;

        assert_eq!(user.gid, 20);
        assert_eq!(user.uid, 501);
        assert_eq!(user.name, "name");
        assert_eq!(user.group, "group");
        assert_eq!(
            user.groups,
            vec![
                Group {
                    gid: 501,
                    name: format!("name"),
                },
                Group {
                    gid: 20,
                    name: format!("group"),
                },
                Group {
                    gid: 20,
                    name: format!("group"),
                },
                Group {
                    gid: 101,
                    name: format!("access_bpf"),
                },
                Group {
                    gid: 12,
                    name: format!("everyone"),
                },
                Group {
                    gid: 61,
                    name: format!("localaccounts"),
                },
                Group {
                    gid: 79,
                    name: format!("_appserverusr"),
                },
                Group {
                    gid: 80,
                    name: format!("admin"),
                },
                Group {
                    gid: 81,
                    name: format!("_appserveradm"),
                },
                Group {
                    gid: 98,
                    name: format!("_lpadmin"),
                },
                Group {
                    gid: 701,
                    name: format!("com.apple.sharepoint.group.1"),
                },
                Group {
                    gid: 702,
                    name: format!("com.apple.sharepoint.group.2"),
                },
                Group {
                    gid: 33,
                    name: format!("_appstore"),
                },
                Group {
                    gid: 100,
                    name: format!("_lpoperator"),
                },
                Group {
                    gid: 204,
                    name: format!("_developer"),
                },
                Group {
                    gid: 250,
                    name: format!("_analyticsusers"),
                },
                Group {
                    gid: 395,
                    name: format!("com.apple.access_ftp"),
                },
                Group {
                    gid: 398,
                    name: format!("com.apple.access_screensharing"),
                }
            ]
        );
        Ok(())
    }
}
