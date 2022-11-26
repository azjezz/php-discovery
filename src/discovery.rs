use std::collections::HashSet;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;

use glob::glob;
use glob::GlobError;
use regex::Regex;

use crate::build::Build;
use crate::error::DiscoveryError;

pub(crate) fn installations_from_env(builds: &mut HashSet<Build>) -> Result<(), DiscoveryError> {
    #[cfg(target_family = "windows")]
    let key = "Path";
    #[cfg(not(target_family = "windows"))]
    let key = "PATH";

    if let Some(paths) = env::var_os(key) {
        for path in env::split_paths(&paths) {
            builds_from_dir(builds, path)?;
        }
    }

    Ok(())
}

#[cfg(not(target_family = "windows"))]
pub(crate) fn scan_dir_for_builds(
    builds: &mut HashSet<Build>,
    path: PathBuf,
) -> Result<(), DiscoveryError> {
    if path.is_dir() {
        builds_from_dir(builds, path.clone())?;

        let directories = read_dir(path).unwrap();
        for directory in directories {
            let directory: PathBuf = directory.unwrap().path();

            builds_from_dir(builds, directory)?;
        }
    }

    Ok(())
}

pub(crate) fn builds_from_dir(
    builds: &mut HashSet<Build>,
    path: PathBuf,
) -> Result<(), DiscoveryError> {
    if !path.is_dir() || path.is_symlink() {
        return Ok(());
    }

    let mut binaries_paths: Vec<String> = Vec::new();

    let mut path = path.display().to_string();
    if path.ends_with('/') {
        path.pop();
    }

    if path.ends_with('\\') {
        path.pop();
    }

    let entries: Vec<Result<PathBuf, GlobError>> = {
        let glob_path = format!("{}/php*", path);
        let glob_bin_path = format!("{}/bin/php*", path);

        glob(glob_path.as_str())
            .unwrap()
            .chain(glob(glob_bin_path.as_str()).unwrap())
            .collect()
    };

    let mut directories = vec![];
    for entry in entries {
        let binary: PathBuf = entry.unwrap();
        if binary.is_dir() {
            // This means that we have a "php"-like dir.
            // store it for later.
            directories.push(binary);

            continue;
        }

        if !PHP_BINARY_REGEX.is_match(binary.to_str().unwrap()) {
            continue;
        }

        binaries_paths.push(binary.to_str().unwrap().parse().unwrap());
    }

    let binaries_paths: HashSet<String> = binaries_paths.iter().cloned().collect();

    let mut found = false;
    for path in binaries_paths.iter() {
        builds.insert(Build::from_binary(path)?);
        found = true;
    }

    if !found && !directories.is_empty() {
        // we didn't find a binary, and we have some directories named "php*"
        for directory in directories {
            builds_from_dir(builds, directory)?;
        }
    }

    Ok(())
}

#[cfg(target_family = "windows")]
lazy_static::lazy_static! {
    // This matches executables like "php", "php74", "php7.4", "php-fpm", "php7.4-cgi" or "php-fpm74"
    static ref PHP_BINARY_REGEX: Regex = Regex::new(r"php([_-])?(\d+(\.\d+)*)?\.(exe|bat|cmd)$").unwrap();
}

#[cfg(not(target_family = "windows"))]
lazy_static::lazy_static! {
    static ref PHP_BINARY_REGEX: Regex = Regex::new(r"php([_-])?(\d+(\.\d+)*)?$").unwrap();
}
