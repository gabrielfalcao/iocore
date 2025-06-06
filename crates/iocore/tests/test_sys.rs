use iocore::{User, guess_unix_home, unix_user_info_home, var as env_var, *};
use iocore_test::path_to_test_file;

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
    let stdout = format!(
        "uid=501(name) gid=20(group) groups=20(group),101(access_bpf),12(everyone),61(localaccounts),79(_appserverusr),80(admin),81(_appserveradm),98(_lpadmin),701(com.apple.sharepoint.group.1),702(com.apple.sharepoint.group.2),33(_appstore),100(_lpoperator),204(_developer),250(_analyticsusers),395(com.apple.access_ftp),398(com.apple.access_screensharing)"
    );
    let user = User::from_id_cmd_string(stdout)?;

    assert_eq!(user.gid, Some(20));
    assert_eq!(user.uid, 501);
    assert_eq!(user.name, "name");
    assert_eq!(user.group, Some("group".to_string()));
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

#[test]
fn test_unix_user_info_home() -> Result<()> {
    let passwd = path_to_test_file!("passwd.osx").write(
        [
            "root:*:0:0:System Administrator:/var/root:/bin/sh",
            "daemon:*:1:1:System Services:/var/root:/usr/bin/false",
            "_cvmsroot:*:212:212:CVMS Root:/var/empty:/usr/bin/false",
        ]
        .join("\n")
        .as_bytes(),
    )?;
    assert_eq!(unix_user_info_home(passwd.to_string().as_str(), "root", 0)?, "/var/root");
    Ok(())
}
