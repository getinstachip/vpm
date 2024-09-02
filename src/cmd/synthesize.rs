use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use std::fs::File;
use std::io::Write;

use crate::cmd::{Execute, Synthesize};

impl Execute for Synthesize {
    async fn execute(&self) -> Result<()> {
        let top_module_path = PathBuf::from(&self.top_module_path);
        
        synthesize_xilinx(top_module_path, self.riscv, self.core_path.clone())?;
        Ok(())
    }
}

pub fn synthesize_xilinx(top_module_path: PathBuf, riscv: bool, core_path: Option<String>) -> Result<()> {
    println!("Starting synthesis with Yosys...");
    let module_name = top_module_path.file_stem().unwrap().to_str().unwrap();
    let top_module_path_str = top_module_path.to_string_lossy();
    let parent_dir = top_module_path.parent().unwrap().to_string_lossy();
    let board_name = "artix7";
    let output_file = format!("{}/{}_{}_{}_synth.v", parent_dir, module_name, board_name, "xilinx");
    
    let mut script_content = format!(
        r#"
# Read the SystemVerilog file
read_verilog -sv {top_module_path_str}
"#
    );

    if riscv {
        if let Some(core_path) = core_path {
            script_content.push_str(&format!(
                r#"
# Read the RISC-V core
read_verilog -sv {core_path}
"#
            ));
        } else {
            return Err(anyhow::anyhow!("RISC-V core path is required when riscv flag is set"));
        }
    }

    script_content.push_str(&format!(
        r#"
# Synthesize for Xilinx 7 series (Artix-7)
synth_xilinx -top {module_name} -family xc7

# Optimize the design
opt

# Map to Xilinx 7 series cells
abc -lut 6

# Clean up
clean

# Write the synthesized design to a Verilog file
write_verilog {output_file}

write_edif {output_file}.edif

# Print statistics
stat
"#
    ));

    // Write the script to a temporary file
    let script_file = "temp_synth_script.ys";
    let mut file = File::create(script_file)?;
    file.write_all(script_content.as_bytes())?;

    // Run Yosys with the created script
    let output = Command::new("yosys")
        .arg("-s")
        .arg(script_file)
        .output()?;

    // Print Yosys output
    println!("Yosys output:");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    if !output.status.success() {
        eprintln!("Yosys error:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    std::fs::remove_file(script_file)?;

    println!("Synthesis complete. Output written to {}", output_file);

    Ok(())
}