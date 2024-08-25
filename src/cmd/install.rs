use anyhow::{Context, Result};
use std::process::Command;
use std::path::Path;

use crate::cmd::{Execute, Install};

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        match self.tool_name.as_str() {
            "verilator" => {
                println!("Installing Verilator...");
                install_verilator()?;
            },
            "chipyard" => {
                println!("Installing Chipyard...");
                install_chipyard()?;
            },
            "openroad" => {
                println!("Installing OpenROAD...");
                install_openroad()?;
            },
            _ => {
                println!("Tool '{}' is not recognized for installation.", self.tool_name);
            }
        }

        Ok(())
    }
}

fn install_verilator() -> Result<()> {
    println!("Installing Verilator...");

    #[cfg(target_os = "macos")]
    {
        println!("Running on macOS...");
        // Install Verilator using Homebrew on macOS
        let status = Command::new("brew")
            .arg("install")
            .arg("verilator")
            .status()
            .context("Failed to install Verilator using Homebrew")?;

        if !status.success() {
            println!("Failed to install Verilator on macOS.");
            return Ok(());
        }
    }

    #[cfg(target_os = "linux")]
    {
        println!("Running on Linux...");
        // Install Verilator using apt-get on Linux
        let status = Command::new("sudo")
            .arg("apt-get")
            .arg("update")
            .status()
            .context("Failed to update package lists")?;

        if !status.success() {
            println!("Failed to update package lists on Linux.");
            return Ok(());
        }

        let status = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .arg("verilator")
            .status()
            .context("Failed to install Verilator using apt-get")?;

        if !status.success() {
            println!("Failed to install Verilator on Linux.");
            return Ok(());
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        println!("Unsupported operating system. Please install Verilator manually.");
        return Ok(());
    }

    println!("Verilator installed successfully.");
    Ok(())
}

fn install_chipyard() -> Result<()> {
    println!("Installing Chipyard...");

    // Define the installation directory
    let install_dir = Path::new("/usr/local/bin");

    // Download Chipyard binary
    let status = Command::new("curl")
        .args(&["-L", "https://github.com/ucb-bar/chipyard/releases/latest/download/chipyard", "-o", install_dir.join("chipyard").to_str().unwrap()])
        .status()
        .context("Failed to download Chipyard binary")?;

    if !status.success() {
        println!("Failed to download Chipyard binary.");
        return Ok(());
    }

    // Make the binary executable
    let status = Command::new("chmod")
        .args(&["+x", install_dir.join("chipyard").to_str().unwrap()])
        .status()
        .context("Failed to make Chipyard binary executable")?;

    if !status.success() {
        println!("Failed to make Chipyard binary executable.");
        return Ok(());
    }

    println!("Chipyard installed successfully.");
    Ok(())
}

fn install_openroad() -> Result<()> {
    println!("Installing OpenROAD...");

    #[cfg(target_os = "linux")]
    {
        println!("Running on Linux...");
        // Install OpenROAD using apt on Linux
        let status = Command::new("sudo")
            .arg("apt")
            .arg("update")
            .status()
            .context("Failed to update package lists")?;

        if !status.success() {
            println!("Failed to update package lists on Linux.");
            return Ok(());
        }

        let status = Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg("openroad")
            .status()
            .context("Failed to install OpenROAD using apt")?;

        if !status.success() {
            println!("Failed to install OpenROAD on Linux.");
            return Ok(());
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("Running on macOS...");
        // Install OpenROAD using Homebrew on macOS
        let status = Command::new("brew")
            .arg("install")
            .arg("openroad/openroad/openroad")
            .status()
            .context("Failed to install OpenROAD using Homebrew")?;

        if !status.success() {
            println!("Failed to install OpenROAD on macOS.");
            return Ok(());
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        println!("Unsupported operating system. Please install OpenROAD manually.");
        return Ok(());
    }

    println!("OpenROAD installed successfully.");
    Ok(())
}