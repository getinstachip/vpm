use anyhow::Result;
use std::path::Path;
use std::process::Command;

use crate::cmd::{Execute, Build};

impl Execute for Build {
    async fn execute(&self) -> Result<()> {
        let top_module_path = Path::new(&self.top_module_path);
        let constraints_path = Path::new(&self.constraints_path);
        if self.riscv {
            load_xilinx(top_module_path, constraints_path)?;
        } else {
            unimplemented!("Non RISC-V loading not yet implemented");
        }
        Ok(())
    }
}

fn build_xilinx(edif_path: &Path, constraints_path: &Path) -> Result<()> {
    let output_dir = edif_path.parent().unwrap_or(Path::new("."));
    let edif_file = edif_path.file_name().unwrap().to_str().unwrap();
    let bitstream_file = format!("{}.bit", edif_path.file_stem().unwrap().to_str().unwrap());

    println!("EDIF file: {}", edif_file);
    println!("Bitstream file: {}", bitstream_file);

    // Run nextpnr-xilinx to generate the bitstream
    let status = Command::new("nextpnr-xilinx")
        .args(&[
            "--chipdb", {
                let chipdb_path = "/usr/local/nextpnr/xilinx/xc7a35t.bin";
                if !Path::new(chipdb_path).exists() {
                    anyhow::bail!("Chipdb file not found: {}", chipdb_path);
                }
                chipdb_path
            },
            "--xdc", constraints_path.to_str().ok_or_else(|| anyhow::anyhow!("Constraints file not found"))?,
            "--json", edif_file,
            "--write", "design.pnr",
            "--fasm", "design.fasm",
        ])
        .current_dir(output_dir)
        .status()?;
    
    println!("Nextpnr-xilinx status: {:?}", status);
    if !status.success() {
        anyhow::bail!("nextpnr-xilinx failed");
    }

    // Run fasm2bits to convert FASM to bitstream
    let status = Command::new("fasm2bits")
        .args(&[
            "--part", "xc7a35tcpg236-1",
            "design.fasm",
            &bitstream_file,
        ])
        .current_dir(output_dir)
        .status()?;

    println!("Fasm2bits status: {:?}", status);
    if !status.success() {
        anyhow::bail!("fasm2bits failed");
    }

    println!("Bitstream generated: {}", bitstream_file);
    Ok(())
}