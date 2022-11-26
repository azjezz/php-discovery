use std::fmt::Display;
use std::hash::Hash;
use std::path::Path;
use std::path::PathBuf;

use crate::error::InstallationError;
use crate::utils::exec;

#[cfg(target_family = "windows")]
/// Represents a PHP build architecture.
///
/// Note: this is only available on windows.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Architecture {
    X86,
    X64,
    AArch64,
}

#[cfg(target_family = "windows")]
/// Try to parse `Architecture` from a string.
impl TryFrom<&str> for Architecture {
    type Error = InstallationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x86" => Ok(Self::X86),
            "x64" => Ok(Self::X64),
            "arm64" => Ok(Self::AArch64),
            _ => Err(InstallationError::FailedToRetrieveArch),
        }
    }
}

#[cfg(target_family = "windows")]
/// Display `Architecture`.
impl Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Architecture::X86 => "x86",
            Architecture::X64 => "x64",
            Architecture::AArch64 => "arm64",
        };

        write!(f, "{}", name)
    }
}

/// Represents a PHP version.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub release: u32,
    pub extra: Option<String>,
}

/// Display `Version`.
///
/// Example:
///
/// ```
/// use php_discovery::build::Version;
///
/// let v = Version {
///     major: 8,
///     minor: 2,
///     release: 0,
///     extra: Some("RC6")
/// };
///
/// assert_eq!("8.2.0RC6", v.to_string());
///
/// let v = Version {
///     major: 7,
///     minor: 4,
///     release: 11,
///     extra: None
/// };
///
/// assert_eq!("7.4.11", v.to_string());
/// ```
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}{}",
            self.major,
            self.minor,
            self.release,
            self.extra.as_ref().unwrap_or(&"".to_string())
        )
    }
}

/// Represents a PHP build.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Build {
    pub version: Version,
    pub binary: PathBuf,
    pub directory: PathBuf,
    pub is_debug: bool,
    pub is_thread_safety_enabled: bool,
    pub php_api: u32,
    pub zend_api: u32,
    #[cfg(target_family = "windows")]
    pub architecture: Architecture,
}

impl Build {
    pub fn from_binary<P: AsRef<Path>>(binary: P) -> Result<Self, InstallationError> {
        let binary = binary.as_ref().to_path_buf();
        if !is_executable::is_executable(&binary) {
            return Err(InstallationError::BinaryIsNotExecutable(binary));
        }

        let directory = binary.parent().unwrap().to_path_buf();
        let version_string = exec(&binary, &["-r", VERSION_CODE])?;
        let parts = version_string.split('.').collect::<Vec<&str>>();
        let version = Version {
            major: parts[0].parse().unwrap(),
            minor: parts[1].parse().unwrap(),
            release: parts[2].parse().unwrap(),
            extra: {
                let extra = parts[3].to_string();

                if extra.is_empty() {
                    None
                } else {
                    Some(extra)
                }
            },
        };

        let information = exec(&binary, &["-i"])?;

        let mut is_debug = false;
        let mut is_thread_safety_enabled = false;
        let mut php_api = None;
        let mut zend_api = None;
        #[cfg(target_family = "windows")]
        let mut architecture = Err(InstallationError::FailedToRetrieveArch);

        for line in information.lines() {
            if line.contains("Thread Safety =>") {
                is_thread_safety_enabled = !line.contains("disabled");
            } else if line.contains("Debug Build =>") {
                is_debug = !line.contains("no");
            } else if line.contains("Zend Extension =>") {
                zend_api = line.get(18..).and_then(|s| s.parse::<u32>().ok());
            } else if line.contains("PHP Extension =>") {
                php_api = line.get(17..).and_then(|s| s.parse::<u32>().ok());
            } else {
                #[cfg(target_family = "windows")]
                if line.contains("Architecture =>") {
                    architecture = line
                        .get(16..)
                        .ok_or(InstallationError::FailedToRetrieveArch)
                        .and_then(|s| TryInto::<Architecture>::try_into(s));
                }
            }
        }

        Ok(Build {
            version,
            binary,
            directory,
            is_debug,
            is_thread_safety_enabled,
            php_api: php_api.ok_or(InstallationError::FailedToRetrieveAPIVersion)?,
            zend_api: zend_api.ok_or(InstallationError::FailedToRetrieveAPIVersion)?,
            #[cfg(target_family = "windows")]
            architecture: architecture?,
        })
    }

    /// Retrieve the path to `php-config`, if available.
    pub fn config(&self) -> Option<PathBuf> {
        self.bin("php-config")
    }

    /// Retrieve the path to `phpdbg`, if available.
    pub fn cgi(&self) -> Option<PathBuf> {
        self.bin("php-cgi")
    }

    /// Retrieve the path to `phpize` binary, if available.
    pub fn phpize(&self) -> Option<PathBuf> {
        self.bin("phpize")
    }

    /// Retrieve the path to `phpdbg`, if available.
    pub fn phpdbg(&self) -> Option<PathBuf> {
        self.bin("phpdbg")
    }

    fn bin(&self, name: &str) -> Option<PathBuf> {
        let filename = self
            .binary
            .file_name()?
            .to_string_lossy()
            .replace("php", name);

        let config = self.directory.join(filename);
        if config.exists() {
            Some(config)
        } else {
            None
        }
    }
}

impl AsRef<Path> for Build {
    fn as_ref(&self) -> &Path {
        self.binary.as_path()
    }
}

const VERSION_CODE: &str =
    "echo PHP_MAJOR_VERSION.'.'.PHP_MINOR_VERSION.'.'.PHP_RELEASE_VERSION.'.'.PHP_EXTRA_VERSION;";
