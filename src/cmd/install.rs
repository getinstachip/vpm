use anyhow::{Context, Result};
use std::process::Command;
use std::path::Path;
use crate::cmd::{Execute, Install};

impl Execute for Install {
    fn execute(&self) -> Result<()> {
        if self.tool_name == "verilator" {
            println!("Including Verilator...");

            #[cfg(target_os = "macos")]

// Clone the Verilator repository
                let status = Command::new("git")
                    .arg("clone")
                    .arg("https://github.com/verilator/verilator.git")
                    .status()
                    .context("Failed to clone Verilator repository")?;

                if !status.success() {
                    println!("Failed to clone Verilator repository.");
                    return Ok(());
                }

            {
                println!("Running on macOS...");
                // Install necessary tools using Homebrew on macOS
                let status = Command::new("brew")
                    .arg("install")
                    .arg("autoconf")
                    .arg("automake")
                    .arg("libtool")
                    .status()
                    .context("Failed to install tools using Homebrew")?;

                if !status.success() {
                    println!("Failed to install necessary tools on macOS.");
                    return Ok(());
                }


                // Change into the Verilator directory
                let verilator_dir = Path::new("verilator");
                if !verilator_dir.exists() {
                    println!("Verilator directory does not exist.");
                    return Ok(());
                }

                // Run autoconf
                let status = Command::new("autoconf")
                    .current_dir(verilator_dir)
                    .status()
                    .context("Failed to run autoconf")?;

                if !status.success() {
                    println!("Failed to run autoconf.");
                    return Ok(());
                }

                // Run ./configure
                let status = Command::new("./configure")
                    .current_dir(verilator_dir)
                    .status()
                    .context("Failed to run ./configure")?;

                if status.success() {
                    println!("Verilator configured successfully on macOS.");
                } else {
                    println!("Failed to configure Verilator on macOS.");
                }
            }

            #[cfg(target_os = "linux")]
            {
                println!("Running on Linux...");
                // Install necessary tools using apt-get on Linux
                let status = Command::new("sudo")
                    .arg("apt-get")
                    .arg("install")
                    .arg("-y")
                    .arg("autoconf")
                    .arg("automake")
                    .arg("libtool")
                    .status()
                    .context("Failed to install tools using apt-get")?;

                if !status.success() {
                    println!("Failed to install necessary tools on Linux.");
                    return Ok(());
                }

                // Clone the Verilator repository
                let status = Command::new("git")
                    .arg("clone")
                    .arg("https://github.com/verilator/verilator.git")
                    .status()
                    .context("Failed to clone Verilator repository")?;

                if !status.success() {
                    println!("Failed to clone Verilator repository.");
                    return Ok(());
                }

                // Change into the Verilator directory
                let verilator_dir = Path::new("verilator");
                if !verilator_dir.exists() {
                    println!("Verilator directory does not exist.");
                    return Ok(());
                }

                // Run autoconf
                let status = Command::new("autoconf")
                    .current_dir(verilator_dir)
                    .status()
                    .context("Failed to run autoconf")?;

                if !status.success() {
                    println!("Failed to run autoconf.");
                    return Ok(());
                }

                // Run ./configure
                let status = Command::new("./configure")
                    .current_dir(verilator_dir)
                    .status()
                    .context("Failed to run ./configure")?;

                if status.success() {
                    println!("Verilator configured successfully on Linux.");
                } else {
                    println!("Failed to configure Verilator on Linux.");
                }
            }
        } else {
            println!("Tool '{}' is not recognized for inclusion.", self.tool_name);
        }

        Ok(())
    }
}