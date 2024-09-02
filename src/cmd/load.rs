use anyhow::Result;
use std::path::Path;
use std::process::Command;

use crate::cmd::{Execute, Load};

impl Execute for Load {
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

fn load_xilinx(edif_path: &Path, constraints_path: &Path) -> Result<()> {
    let edif_path_str = edif_path.to_str().unwrap();
    let constraints_path_str = constraints_path.to_str().unwrap();
    
    Command::new("yosys")
        .args(&["-p", &format!("read_edif {}; write_json design.json", edif_path_str)])
        .status()?;

    Command::new("nextpnr-xilinx")
        .args(&[
            "--chipdb", "vpm_modules/chipdb-xc7a35t.bin",
            "--xdc", constraints_path_str,
            "--json", "design.json",
            "--write", "output.fasm",
            "--device", "xc7a35tcsg324-1"
        ])
        .status()?;

    let fasm_output = Command::new("fasm2frames")
        .args(&["--part", "xc7a35tcsg324-1", "output.fasm"])
        .output()?;
    std::fs::write("output.frames", fasm_output.stdout)?;

    Command::new("xc7frames2bit")
        .args(&[
            "--part_file", "vpm_modules/xc7a35tcsg324-1.yaml",
            "--part_name", "xc7a35tcsg324-1",
            "--frm_file", "output.frames",
            "--output_file", "output.bit"
        ])
        .status()?;

    println!("Bitstream generated successfully: output.bit");
    Ok(())
}