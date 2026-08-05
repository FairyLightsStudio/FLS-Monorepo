//! Dependency installer for various packages and tools

use tera_common::error::{Error, Result};
use tokio::process::Command;

/// Supported package managers
#[derive(Debug, Clone)]
pub enum PackageManager {
    Apt,     // Debian/Ubuntu
    Yum,     // RHEL/CentOS
    Pacman,  // Arch Linux
    Dnf,     // Fedora
    Brew,    // macOS
}

/// Dependency installer
pub struct DepsInstaller;

impl DepsInstaller {
    /// Detect the system's package manager
    pub fn detect_package_manager() -> Result<PackageManager> {
        // TODO: Implement package manager detection
        // - Check for apt (Debian/Ubuntu)
        // - Check for yum/dnf (RHEL/Fedora)
        // - Check for pacman (Arch)
        // - Check for brew (macOS)

        // Placeholder: assume APT
        Ok(PackageManager::Apt)
    }

    /// Install a package
    pub async fn install_package(&self, package: &str) -> Result<()> {
        let pkg_manager = Self::detect_package_manager()?;

        match pkg_manager {
            PackageManager::Apt => {
                self.install_via_apt(package).await
            }
            PackageManager::Yum => {
                self.install_via_yum(package).await
            }
            PackageManager::Pacman => {
                self.install_via_pacman(package).await
            }
            PackageManager::Dnf => {
                self.install_via_dnf(package).await
            }
            PackageManager::Brew => {
                self.install_via_brew(package).await
            }
        }
    }

    /// Install package via apt
    async fn install_via_apt(&self, package: &str) -> Result<()> {
        let output = Command::new("sudo")
            .args(&["apt", "update"])
            .output()
            .await
            .map_err(|e| Error::Io(e))?;

        if !output.status.success() {
            return Err(Error::Service(format!(
                "Failed to update apt: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let output = Command::new("sudo")
            .args(&["apt", "install", "-y", package])
            .output()
            .await
            .map_err(|e| Error::Io(e))?;

        if !output.status.success() {
            return Err(Error::Service(format!(
                "Failed to install package: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Install package via yum
    async fn install_via_yum(&self, package: &str) -> Result<()> {
        let output = Command::new("sudo")
            .args(&["yum", "install", "-y", package])
            .output()
            .await
            .map_err(|e| Error::Io(e))?;

        if !output.status.success() {
            return Err(Error::Service(format!(
                "Failed to install package: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Install package via pacman
    async fn install_via_pacman(&self, package: &str) -> Result<()> {
        let output = Command::new("sudo")
            .args(&["pacman", "-S", "--noconfirm", package])
            .output()
            .await
            .map_err(|e| Error::Io(e))?;

        if !output.status.success() {
            return Err(Error::Service(format!(
                "Failed to install package: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Install package via dnf
    async fn install_via_dnf(&self, package: &str) -> Result<()> {
        let output = Command::new("sudo")
            .args(&["dnf", "install", "-y", package])
            .output()
            .await
            .map_err(|e| Error::Io(e))?;

        if !output.status.success() {
            return Err(Error::Service(format!(
                "Failed to install package: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Install package via brew
    async fn install_via_brew(&self, package: &str) -> Result<()> {
        let output = Command::new("brew")
            .args(&["install", package])
            .output()
            .await
            .map_err(|e| Error::Io(e))?;

        if !output.status.success() {
            return Err(Error::Service(format!(
                "Failed to install package: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }
}
