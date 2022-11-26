use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::PathBuf;
use std::process::Command;

use crate::error::InstallationError;

pub(crate) fn exec<S: AsRef<OsStr> + Debug>(
    binary: &PathBuf,
    argv: &[S],
) -> Result<String, InstallationError> {
    let output = Command::new(binary)
        .args(argv)
        .output()?
        .stdout;

    Ok(String::from_utf8_lossy(&output).trim().to_string())
}

#[macro_export]
macro_rules! implement_from_for_enum {
    ($type:ty, $error:ty, $field:ident) => {
        impl From<$type> for $error {
            fn from(value: $type) -> Self {
                <$error>::$field(value)
            }
        }
    };
}
