use std::path::PathBuf;
use walkdir::WalkDir;

use crate::coreio::absolute_path;
use crate::exceptions::Exception;
use crate::plant::PathRelative;


pub fn rsvfilematch<M: FnMut(&PathBuf) -> bool>(
    filenames: Vec<String>,
    mut matcher: M,
) -> Result<Vec<PathBuf>, Exception> {
    let mut result = Vec::<PathBuf>::new();
    for filename in if filenames.len() == 0 {
        vec![format!("{}", std::env::current_dir()?.display())]
    } else {
        filenames.clone()
    }
    .iter()
    {
        let path = absolute_path(filename.as_str())?;
        if !path.try_exists()? {
            continue;
        }
        if path.is_dir() {
            for entry in WalkDir::new(path) {
                let entry = match entry {
                    Ok(entry) => entry.clone(),
                    Err(_) => continue,
                };
                let path = entry.path();
                if path.is_dir() {
                    continue;
                }
                if matcher(&path.to_path_buf()) {
                    result.push(path.to_path_buf().relative_wherewith(path));
                }
            }
        } else {
            if matcher(&path.to_path_buf()) {
                result.push(path.to_path_buf());
            }
        }
    }
    Ok(result)
}


#[cfg(test)]
mod functests {
    use super::*;
    use crate::coreio::{open_write, homedir};
    use std::io::Write;

    #[test]
    fn test_ow() -> Result<(), Exception> {
        let filez = [
            "foo/bar/123/45/6.bin",
            "foo/bar/123/44/6.bin",
            "foo/bar/123/43/6.bin",
            "foo/bar/123/42/6.bin",
            "foo/bar/111/30/6.bin",
            "foo/bar/111/222/333.bin",
            "foo/bar/111/333/444.bin",
            "foo/bar/111/444/555.bin",
            "foo/baz/123/45/6.bin",
            "foo/baz/123/44/6.bin",
            "foo/baz/123/43/6.bin",
            "foo/baz/123/42/6.bin",
            "foo/baz/111/30/6.bin",
            "foo/baz/111/222/333.bin",
            "foo/baz/111/333/444.bin",
            "foo/baz/111/444/555.bin",
        ]
        .iter()
        .map(|p| {
            match open_write(p) {
                Ok(mut f) => match f.write_all(b"test1234") {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("cannot write {}: {}", p, e);
                    }
                },
                Err(e) => {
                    eprintln!("cannot write {}: {}", p, e);
                }
            };
            p.to_string()
        })
        .collect::<Vec<String>>();

        assert_eq!(
            filez,
            [
                "foo/bar/123/45/6.bin",
                "foo/bar/123/44/6.bin",
                "foo/bar/123/43/6.bin",
                "foo/bar/123/42/6.bin",
                "foo/bar/111/30/6.bin",
                "foo/bar/111/222/333.bin",
                "foo/bar/111/333/444.bin",
                "foo/bar/111/444/555.bin",
                "foo/baz/123/45/6.bin",
                "foo/baz/123/44/6.bin",
                "foo/baz/123/43/6.bin",
                "foo/baz/123/42/6.bin",
                "foo/baz/111/30/6.bin",
                "foo/baz/111/222/333.bin",
                "foo/baz/111/333/444.bin",
                "foo/baz/111/444/555.bin"
            ]
            .to_vec()
        );
        let matches = rsvfilematch(vec![format!(".")], |path| {
            path.starts_with(&homedir().unwrap()) && path.ends_with(".bin")
        })?;
        assert_eq!(matches, Vec::<PathBuf>::new());
        Ok(())
    }
}
