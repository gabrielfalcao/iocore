use std::cmp::Ordering;

use crate::Path;

pub(crate) fn partial_cmp_paths_by_parts(a: &Path, b: &Path) -> Option<Ordering> {
    Some(cmp_paths_by_parts(a, b))
}
pub(crate) fn cmp_paths_by_parts(a: &Path, b: &Path) -> Ordering {
    let cmp = b
        .is_dir()
        .cmp(&a.is_dir())
        .cmp(&b.to_string().cmp(&a.to_string()).cmp(&a.split().len().cmp(&b.split().len())));
    let cmp = if cmp == Ordering::Equal { fallback_cmp_paths_by_parts(a, b) } else { cmp };
    cmp
}

pub(crate) fn path_ord_split_max(a: Path, b: Path) -> Path {
    let a_parts = a.split();
    let b_parts = b.split();
    if a_parts.len() > b_parts.len() {
        a
    } else if a_parts.len() < b_parts.len() {
        b
    } else {
        path_ord_to_string_max(a, b)
    }
}

pub(crate) fn path_ord_split_min(a: Path, b: Path) -> Path {
    let a_parts = a.split();
    let b_parts = b.split();
    if a_parts.len() < b_parts.len() {
        a
    } else if a_parts.len() > b_parts.len() {
        b
    } else {
        path_ord_to_string_min(a, b)
    }
}

pub(crate) fn path_ord_split_clamp(current: Path, min: Path, max: Path) -> Path {
    let current_parts = current.split();
    let min_parts = min.split();
    let max_parts = max.split();
    if current_parts.len() > max_parts.len() {
        max
    } else if current_parts.len() < min_parts.len() {
        min
    } else {
        path_ord_to_string_clamp(current, min, max)
    }
}

pub(crate) fn path_ord_to_string_max(a: Path, b: Path) -> Path {
    let a_parts = a.to_string();
    let b_parts = b.to_string();
    if a_parts.len() > b_parts.len() {
        a
    } else if a_parts.len() < b_parts.len() {
        b
    } else {
        Path::raw(a_parts.max(b_parts))
    }
}

pub(crate) fn path_ord_to_string_min(a: Path, b: Path) -> Path {
    let a_parts = a.to_string();
    let b_parts = b.to_string();
    if a_parts.len() < b_parts.len() {
        a
    } else if a_parts.len() > b_parts.len() {
        b
    } else {
        Path::raw(a_parts.min(b_parts))
    }
}

pub(crate) fn path_ord_to_string_clamp(current: Path, min: Path, max: Path) -> Path {
    let current_parts = current.to_string();
    let min_parts = min.to_string();
    let max_parts = max.to_string();
    if current_parts.len() > max_parts.len() {
        max
    } else if current_parts.len() < min_parts.len() {
        min
    } else {
        Path::raw(current_parts.clamp(min_parts, max_parts))
    }
}
/// `fallback_cmp_paths_by_parts` provides a way to further order two
/// paths when previous attempts at ordering them yields
/// [`std::cmp::Ordering::Equal`].
///
/// In other words, this function is a clear attempt at achieving "total order"
pub(crate) fn fallback_cmp_paths_by_parts(a: &Path, b: &Path) -> Ordering {
    if a.split().len() > b.split().len() {
        Ordering::Greater
    } else if a.split().len() < b.split().len() {
        Ordering::Less
    } else {
        if a.to_string().len() > b.to_string().len() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

#[cfg(test)]
mod test_very_specific_ordering {
    use crate::Path;
    #[test]
    fn test_paths_should_be_ordered_alphabetically() {
        let mut paths =
            vec![Path::raw("zzzzz"), Path::raw("mmmmm"), Path::raw("nnnnn"), Path::raw("aaaaa")];
        paths.sort();
        assert_eq!(
            paths,
            vec![Path::raw("aaaaa"), Path::raw("mmmmm"), Path::raw("nnnnn"), Path::raw("zzzzz"),]
        );
    }
    #[test]
    fn but_paths_should_be_ordered_by_length() {
        let mut paths =
            vec![Path::raw("mmmmm"), Path::raw("mmm"), Path::raw("aaa"), Path::raw("aaaa")];
        paths.sort();
        assert_eq!(
            paths,
            vec![Path::raw("aaa"), Path::raw("aaaa"), Path::raw("mmm"), Path::raw("mmmmm"),]
        );
    }
    #[test]
    fn but_paths_should_be_ordered_by_depth_of_folders() {
        let mut paths = vec![
            Path::raw("abcdefg"),
            Path::raw("nopqrst"),
            Path::raw("a/bcdefg"),
            Path::raw("no/pqrst"),
            Path::raw("u/v/w/x/y"),
            Path::raw("uv/wx/y"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("abcdefg"),
                Path::raw("nopqrst"),
                Path::raw("a/bcdefg"),
                Path::raw("no/pqrst"),
                Path::raw("uv/wx/y"),
                Path::raw("u/v/w/x/y"),
            ]
        );
    }
}
