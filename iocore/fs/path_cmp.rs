use std::cmp::Ordering;

use crate::Path;

pub(crate) fn partial_cmp_paths_by_parts(a: &Path, b: &Path) -> Option<Ordering> {
    let cmp = b.is_dir().partial_cmp(&a.is_dir()).partial_cmp(
        &b.to_string()
            .partial_cmp(&a.to_string())
            .partial_cmp(&a.split().len().partial_cmp(&b.split().len())),
    );
    match cmp {
        Some(Ordering::Less) | Some(Ordering::Greater) => cmp,
        _ => Some(fallback_cmp_paths_by_parts(a, b)),
    }
}
pub(crate) fn cmp_paths_by_parts(a: &Path, b: &Path) -> Ordering {
    let cmp = b
        .is_dir()
        .cmp(&a.is_dir())
        .cmp(&b.to_string().cmp(&a.to_string()).cmp(&a.split().len().cmp(&b.split().len())));
    if cmp == Ordering::Equal {
        fallback_cmp_paths_by_parts(a, b)
    } else {
        cmp
    }
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
    } else {
        b
    }
}

pub(crate) fn path_ord_to_string_min(a: Path, b: Path) -> Path {
    let a_parts = a.to_string();
    let b_parts = b.to_string();
    if a_parts.len() < b_parts.len() {
        a
    } else {
        b
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
        current
    }
}

pub(crate) fn fallback_cmp_paths_by_parts(a: &Path, b: &Path) -> Ordering {
    let ordering = if a.split().len() > b.split().len() {
        Ordering::Greater
    } else if a.split().len() < b.split().len() {
        Ordering::Less
    } else {
        if a.to_string().len() > b.to_string().len() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    };
    ordering
}
