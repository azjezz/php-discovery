use std::io::Error;
use std::path::PathBuf;

use crate::implement_from_for_enum;

#[derive(Debug)]
pub enum InstallationError {
    BinaryIsNotExecutable(PathBuf),
    CommandError(Error),
    FailedToRetrieveAPIVersion,
    #[cfg(target_family = "windows")]
    FailedToRetrieveArch,
}

#[derive(Debug)]
pub enum DiscoveryError {
    FailedToReadDirectory(Error),
    InstallationError(InstallationError),
    #[cfg(windows)]
    FailedToLocateSystemDrive,
}

implement_from_for_enum!(Error, InstallationError, CommandError);
implement_from_for_enum!(Error, DiscoveryError, FailedToReadDirectory);
implement_from_for_enum!(InstallationError, DiscoveryError, InstallationError);
