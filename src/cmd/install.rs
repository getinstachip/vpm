use anyhow::{Context, Result};
use std::process::Command;
use std::path::Path;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use crate::cmd::{Execute, Install};

impl Execute for Install {
    async fn execute(&self) -> Result<()> {
        match self.tool_name.as_str() {
            "verilator" => {
                println!("Installing Verilator...");
                install_verilator()?;
            },
            "icarus-verilog" => {
                println!("Installing Icarus Verilog...");
                install_icarus_verilog()?;
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
            "yosys" => {
                println!("Installing Yosys...");
                install_yosys()?;
            },
            "riscv" => {
                println!("Installing RISC-V toolchain...");
                install_riscv()?;
            },
            "nextpnr" => {
                println!("Installing NextPNR...");
                install_nextpnr()?;
            },
            "project-xray" => {
                println!("Installing Project XRay...");
                install_xray()?;
            },
            _ => {
                println!("Tool '{}' is not recognized for installation.", self.tool_name);
            }
        }

        Ok(())
    }
}

fn has_sudo_access() -> bool {
    let output = Command::new("sudo")
        .arg("-n")
        .arg("true")
        .output()
        .expect("Failed to execute sudo command");
    output.status.success()
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
        
        if has_sudo_access() {
            // Install Verilator using package manager
            if !is_arch_distro() {
                // Install Verilator using apt-get on non-Arch Linux
                let status = Command::new("sudo")
                    .args(&["apt-get", "update"])
                    .status()
                    .context("Failed to update package lists")?;

                if !status.success() {
                    println!("Failed to update package lists on Linux.");
                    return Ok(());
                }

                let status = Command::new("sudo")
                    .args(&["apt-get", "install", "-y", "verilator"])
                    .status()
                    .context("Failed to install Verilator using apt-get")?;

                if !status.success() {
                    println!("Failed to install Verilator on Linux.");
                    return Ok(());
                }
            } else {
                // Install Verilator using pacman on Arch Linux
                let status = Command::new("sudo")
                    .args(&["pacman", "-Syu", "--noconfirm", "verilator"])
                    .status()
                    .context("Failed to install Verilator using pacman")?;

                if !status.success() {
                    println!("Failed to install Verilator on Arch Linux.");
                    return Ok(());
                }
            }
        } else {
            println!("No sudo access. Installing Verilator from source...");
            install_verilator_from_source()?;
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

fn install_verilator_from_source() -> Result<()> {
    // Create a directory for the installation
    let install_dir = Path::new(&std::env::var("HOME")?).join("verilator");
    std::fs::create_dir_all(&install_dir)?;

    // Clone the repository
    Command::new("git")
        .args(&["clone", "https://github.com/verilator/verilator"])
        .current_dir(&install_dir)
        .status()
        .context("Failed to clone Verilator repository")?;

    let source_dir = install_dir.join("verilator");

    // Configure with custom prefix
    Command::new("autoconf")
        .current_dir(&source_dir)
        .status()
        .context("Failed to run autoconf for Verilator")?;

    Command::new("./configure")
        .arg(format!("--prefix={}", install_dir.display()))
        .current_dir(&source_dir)
        .status()
        .context("Failed to configure Verilator")?;

    // Build
    Command::new("make")
        .current_dir(&source_dir)
        .status()
        .context("Failed to build Verilator")?;

    // Install
    Command::new("make")
        .arg("install")
        .current_dir(&source_dir)
        .status()
        .context("Failed to install Verilator")?;

    // Add installation directory to PATH
    println!("Verilator installed successfully in {}.", install_dir.display());
    println!("Please add the following line to your shell configuration file (e.g., .bashrc or .zshrc):");
    println!("export PATH=$PATH:{}/bin", install_dir.display());

    Ok(())
}

fn install_icarus_verilog() -> Result<()> {
    println!("Installing Icarus Verilog...");

    #[cfg(target_os = "macos")]
    {
        println!("Running on macOS...");
        // Install Icarus Verilog using Homebrew on macOS
        let status = Command::new("brew")
            .arg("install")
            .arg("icarus-verilog")
            .status()
            .context("Failed to install Icarus Verilog using Homebrew")?;

        if !status.success() {
            println!("Failed to install Icarus Verilog on macOS.");
            return Ok(());
        }
    }

    #[cfg(target_os = "linux")]
    {
        if !is_arch_distro() {
            println!("Running on Linux...");
            // Install Icarus Verilog using apt-get on Linux
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
                .arg("iverilog")
                .status()
                .context("Failed to install Icarus Verilog using apt-get")?;

            if !status.success() {
                println!("Failed to install Icarus Verilog on Linux using Apt-Get.");
                let install_dir = Path::new(&std::env::var("HOME")?).join("icarus_verilog");
    std::fs::create_dir_all(&install_dir)?;


    Command::new("git")
        .args(&["clone", "https://github.com/steveicarus/iverilog.git"])
        .current_dir(&install_dir)
        .status()
        .context("Failed to clone Icarus Verilog repository")?;

    let source_dir = install_dir.join("iverilog");


    Command::new("sh")
        .arg("autoconf.sh")
        .current_dir(&source_dir)
        .status()
        .context("Failed to generate configure script")?;


    Command::new("./configure")
        .arg(format!("--prefix={}", install_dir.display()))
        .current_dir(&source_dir)
        .status()
        .context("Failed to configure Icarus Verilog")?;


    Command::new("make")
        .current_dir(&source_dir)
        .status()
        .context("Failed to build Icarus Verilog")?;


    Command::new("make")
        .arg("install")
        .current_dir(&source_dir)
        .status()
        .context("Failed to install Icarus Verilog")?;


    println!("Icarus Verilog installed successfully.");
    println!("Please add the following line to your shell configuration file (e.g., .bashrc or .zshrc):");
    println!("export PATH=$PATH:{}/bin", install_dir.display());

                return Ok(());
            } else {
                
        } else {
            println!("Running on Arch Linux...");
            // Install Icarus Verilog using pacman on Arch Linux
            let status = Command::new("sudo")
                .arg("pacman")
                .arg("-Syu")
                .arg("--noconfirm")
                .arg("iverilog")
                .status()
                .context("Failed to install Icarus Verilog using pacman")?;

            if !status.success() {
                println!("Failed to install Icarus Verilog on Arch Linux.");
                return Ok(());
            }
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        println!("Unsupported operating system. Please install Icarus Verilog manually.");
        return Ok(());
    }

    println!("Icarus Verilog installed successfully.");
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

    let (_python_cmd, pip_cmd) = if check_command("python3") {
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

fn install_yosys() -> Result<()> {
    println!("Installing Yosys and ABC...");

    #[cfg(target_os = "macos")]
    {
        println!("Running on macOS...");
        // Install Yosys using Homebrew on macOS
        let status = Command::new("brew")
            .arg("install")
            .arg("yosys")
            .status()
            .context("Failed to install Yosys using Homebrew")?;

        if !status.success() {
            println!("Failed to install Yosys on macOS.");
            return Ok(());
        }

        // Install ABC by git cloning and making
        if !Path::new("/usr/local/bin/abc").exists() {
            println!("Installing ABC...");
            let status = Command::new("git")
                .args(&["clone", "https://github.com/berkeley-abc/abc.git"])
                .status()
                .context("Failed to clone ABC repository")?;

            if !status.success() {
                println!("Failed to clone ABC repository.");
                return Ok(());
            }

            let status = Command::new("make")
                .current_dir("abc")
                .status()
                .context("Failed to make ABC")?;

            if !status.success() {
                println!("Failed to make ABC.");
                return Ok(());
            }

            let status = Command::new("sudo")
                .args(&["mv", "abc/abc", "/usr/local/bin/"])
                .status()
                .context("Failed to move ABC to /usr/local/bin/")?;

            if !status.success() {
                println!("Failed to move ABC to /usr/local/bin/.");
                return Ok(());
            }

            println!("ABC installed successfully.");
        } else {
            println!("ABC is already installed.");
        }
    }
    println!("Yosys and ABC installed successfully.");
    Ok(())
}

fn install_riscv() -> Result<()> {
    println!("Installing RISC-V toolchain...");
    Command::new("git")
        .args(&["clone", "--recursive", "https://github.com/riscv/riscv-gnu-toolchain.git"])
        .status()?;

    // Change to the cloned directory
    env::set_current_dir("riscv-gnu-toolchain")?;

    // Step 2: Install prerequisites (for Ubuntu/Debian)
    Command::new("sudo")
        .args(&["apt-get", "install", "autoconf", "automake", "autotools-dev", "curl", "python3", "libmpc-dev", "libmpfr-dev", "libgmp-dev", "gawk", "build-essential", "bison", "flex", "texinfo", "gperf", "libtool", "patchutils", "bc", "zlib1g-dev", "libexpat-dev"])
        .status()?;

    // Step 3: Create install directory
    Command::new("sudo")
        .args(&["mkdir", "-p", "/opt/riscv"])
        .status()?;

    // Step 4: Configure and build the toolchain
    Command::new("./configure")
        .arg("--prefix=/opt/riscv")
        .status()?;

    Command::new("sudo")
        .arg("make")
        .status()?;

    // Step 5: Add the toolchain to PATH
    let home = env::var("HOME")?;
    let bashrc_path = Path::new(&home).join(".bashrc");
    let mut bashrc = OpenOptions::new()
        .append(true)
        .open(bashrc_path)?;

    writeln!(bashrc, "\nexport PATH=$PATH:/opt/riscv/bin")?;

    // Step 6: Verify installation
    Command::new("/opt/riscv/bin/riscv64-unknown-elf-gcc")
        .arg("--version")
        .status()?;

    println!("RISC-V GNU toolchain installed successfully!");
    println!("Please restart your terminal or run 'source ~/.bashrc' to update your PATH.");
    Ok(())
}

fn install_nextpnr() -> Result<()> {
    println!("Installing NextPNR...");

    // Install NextPNR using Homebrew on macOS
    let status = Command::new("brew")
        .arg("install")
        .arg("nextpnr")
        .status()
        .context("Failed to install NextPNR using Homebrew")?;

    if !status.success() {
        println!("Failed to install NextPNR on macOS.");
        return Ok(());
    }

    println!("NextPNR installed successfully.");
    Ok(())
}

fn install_xray() -> Result<()> {
    println!("Installing Project XRay...");

    // Install Project XRay using Homebrew on macOS
    let status = Command::new("brew")
        .arg("install")
        .arg("xray")
        .status()
        .context("Failed to install Project XRay using Homebrew")?;

    if !status.success() {
        println!("Failed to install Project XRay on macOS.");
        return Ok(());
    }

    println!("Project XRay installed successfully.");
    Ok(())
}

fn is_arch_distro() -> bool {
    Command::new("pacman")
        .arg("--version")
        .output()
        .is_ok()
}
