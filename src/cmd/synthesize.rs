use anyhow::{Result, Context};
use std::path::PathBuf;
use std::process::Command;
use std::fs::File;
use std::io::Write;

use crate::cmd::{Execute, Synthesize};

impl Execute for Synthesize {
    async fn execute(&self) -> Result<()> {
        let top_module_path = PathBuf::from(&self.top_module_path);
        
        synthesize_xilinx(top_module_path)?;
        Ok(())
    }
}

pub fn synthesize_xilinx(top_module_path: PathBuf) -> Result<()> {
    println!("Starting synthesis with Yosys...");
    let module_name = top_module_path.file_stem().unwrap().to_str().unwrap();
    let top_module_path_str = top_module_path.to_string_lossy();
    let parent_dir = top_module_path.parent().unwrap().to_string_lossy();
    let board_name = "artix7"; // You might want to make this configurable
    let output_file = format!("{}/{}_{}_{}_synth.v", parent_dir, module_name, board_name, "xilinx");
    let script_content = format!(
        r#"
# Read the SystemVerilog file
read_verilog -sv {top_module_path_str}

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

# Print statistics
stat
"#
    );

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

    // Clean up the temporary script file
    std::fs::remove_file(script_file)?;

    println!("Synthesis complete. Output written to {}", output_file);

    Ok(())
}