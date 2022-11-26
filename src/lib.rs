mod discovery;
mod utils;

pub mod build;
pub mod error;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

/// Discover all available PHP builds.
pub fn discover() -> Result<Vec<build::Build>, error::DiscoveryError> {
    let mut builds: HashSet<build::Build> = HashSet::new();

    discovery::installations_from_env(&mut builds)?;

    #[cfg(target_family = "windows")]
    {
        let system_drive: PathBuf = env::var_os("SystemDrive")
            .map(|system_drive_name| format!("{}\\", system_drive_name.to_string_lossy()))
            .map(|system_drive| PathBuf::from(system_drive))
            .ok_or(error::DiscoveryError::FailedToLocateSystemDrive)?;

        // XAMPP
        discovery::builds_from_dir(&mut builds, system_drive.join("xampp").join("php"))?;
        // Cygwin
        discovery::builds_from_dir(&mut builds, system_drive.join("cygwin64").join("bin"))?;
        discovery::builds_from_dir(&mut builds, system_drive.join("cygwin").join("bin"))?;
        // Chocolatey
        discovery::builds_from_dir(&mut builds, system_drive.join("tools"))?;
        // WAMP
        discovery::builds_from_dir(
            &mut builds,
            system_drive.join("wamp64").join("bin").join("php"),
        )?;
        discovery::builds_from_dir(
            &mut builds,
            system_drive.join("wamp").join("bin").join("php"),
        )?;
        // MAMP
        discovery::builds_from_dir(
            &mut builds,
            system_drive.join("mamp").join("bin").join("php"),
        )?;
    }

    #[cfg(not(target_family = "windows"))]
    {
        if let Some(home) = env::var_os("HOME") {
            let home: PathBuf = home.into();

            // phpbrew
            discovery::scan_dir_for_builds(&mut builds, home.join(".phpbrew").join("php"))?;
            // .phpenv
            discovery::scan_dir_for_builds(&mut builds, home.join(".phpenv").join("versions"))?;
        }

        #[cfg(target_os = "macos")]
        {
            // MacPorts (/opt/local/sbin/php-fpm71, /opt/local/bin/php71)
            discovery::scan_dir_for_builds(&mut builds, PathBuf::from("/opt/local"))?;
            // MAMP
            discovery::scan_dir_for_builds(
                &mut builds,
                PathBuf::from("/Applications/MAMP/bin/php/"),
            )?;
        }

        #[cfg(target_os = "linux")]
        {
            // Ondrej PPA on Linux (bin/php7.2)
            discovery::scan_dir_for_builds(&mut builds, PathBuf::from("/usr"))?;
            // Remi's RPM repository
            discovery::scan_dir_for_builds(&mut builds, PathBuf::from("/opt/remi"))?;
        }
    }

    Ok(builds.into_iter().collect())
}
