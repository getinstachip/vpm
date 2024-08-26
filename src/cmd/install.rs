use anyhow::{Context, Result};
use std::process::Command;
use std::path::Path;
use std::fs;

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
            "edalize" => {
                println!("Installing Edalize...");
                install_edalize()?;
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

fn install_edalize() -> Result<()> {
    println!("Installing Edalize...");

    let (python_cmd, pip_cmd) = if check_command("python3") {
        ("python3", "pip3")
    } else if check_command("python") {
        ("python", "pip")
    } else {
        println!("Neither Python 3 nor Python 2 is installed. Please install Python before proceeding.");
        return Ok(());
    };

    if !check_command(pip_cmd) {
        println!("{} is not installed. Please install pip before proceeding.", pip_cmd);
        return Ok(());
    }

    // Install Edalize
    let status = Command::new(pip_cmd)
        .arg("install")
        .arg("--user")
        .arg("edalize")
        .status()
        .context("Failed to install Edalize using pip")?;

    if !status.success() {
        println!("Failed to install Edalize.");
        return Ok(());
    }

    // Install FuseSoC
    let status = Command::new(pip_cmd)
        .arg("install")
        .arg("--user")
        .arg("fusesoc")
        .status()
        .context("Failed to install FuseSoC using pip")?;

    if !status.success() {
        println!("Failed to install FuseSoC.");
        return Ok(());
    }

    println!("Edalize installed successfully.");
    Ok(())
}

fn check_command(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .is_ok()
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