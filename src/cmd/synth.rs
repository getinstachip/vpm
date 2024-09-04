use anyhow::{Result, Context};
use std::path::PathBuf;
use std::process::Command;
use std::fs::File;
use std::io::Write;

use crate::cmd::{Execute, Synth};

impl Execute for Synth {
    async fn execute(&self) -> Result<()> {
        let top_module_path = PathBuf::from(&self.top_module_path);
        
        match &self.board {
            Some(board) if board.to_lowercase() == "xilinx" => {
                synthesize_xilinx(top_module_path, self.riscv, self.core_path.clone())?;
            },
            None => {
                let input_file = top_module_path.to_str().unwrap();
                let top_module = top_module_path.file_stem().unwrap().to_str().unwrap();
                let output_file = format!("{}_synth.v", top_module);
                synthesize_design(input_file, top_module, &output_file)?;
            },
            Some(other) => {
                return Err(anyhow::anyhow!("Unsupported board: {}", other));
            }
        }
        Ok(())
    }
}

pub fn synthesize_design(input_file: &str, top_module: &str, output_file: &str) -> Result<()> {
    println!("Synthesizing design...");

    let script_path = create_yosys_script(input_file, top_module, output_file)?;
    run_yosys(&script_path)?;

    println!("Synthesis completed successfully.");
    Ok(())
}

pub fn create_yosys_script(input_file: &str, top_module: &str, output_file: &str) -> Result<PathBuf> {
    let script_content = generate_yosys_script_content(input_file, top_module, output_file);

    let script_path = PathBuf::from(input_file).with_file_name(format!("{}_synth_script.ys", top_module));
    std::fs::write(&script_path, script_content).context("Failed to create Yosys script")?;

    Ok(script_path)
}

fn generate_yosys_script_content(input_file: &str, top_module: &str, output_file: &str) -> String {
    format!(
        r#"
# Read the Verilog file
read_verilog {}

# Synthesize the design
synth -top {}

# Optimize the design
opt

# Write the synthesized design
write_verilog {}
        "#,
        input_file,
        top_module,
        output_file
    )
}

fn run_yosys(script_path: &PathBuf) -> Result<()> {
    let output = Command::new("yosys")
        .arg(script_path)
        .output()
        .context("Failed to execute Yosys")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Yosys synthesis failed: {}", error_message));
    }

    println!("Yosys Output:");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

pub fn synthesize_xilinx(top_module_path: PathBuf, riscv: bool, core_path: Option<String>) -> Result<()> {
    println!("Starting synthesis with Yosys...");
    let (module_name, top_module_path_str, parent_dir) = extract_path_info(&top_module_path);
    let board_name = "artix7";
    let output_file = format!("{}/{}_{}_{}_synth.v", parent_dir, module_name, board_name, "xilinx");
    
    let script_content = generate_xilinx_script_content(&top_module_path_str, riscv, core_path, &module_name, &output_file)?;

    let script_file = top_module_path.with_file_name(format!("{}_xilinx_synth_script.ys", module_name));
    write_script_to_file(&script_file, &script_content)?;

    run_yosys_with_script(&script_file)?;

    println!("Synthesis complete. Output written to {}", output_file);

    Ok(())
}

fn extract_path_info(top_module_path: &PathBuf) -> (String, String, String) {
    let module_name = top_module_path.file_stem().unwrap().to_str().unwrap().to_string();
    let top_module_path_str = top_module_path.to_string_lossy().to_string();
    let parent_dir = top_module_path.parent().unwrap().to_string_lossy().to_string();
    (module_name, top_module_path_str, parent_dir)
}

fn generate_xilinx_script_content(top_module_path_str: &str, riscv: bool, core_path: Option<String>, module_name: &str, output_file: &str) -> Result<String> {
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

    Ok(script_content)
}

fn write_script_to_file(script_file: &PathBuf, script_content: &str) -> Result<()> {
    let mut file = File::create(script_file)?;
    file.write_all(script_content.as_bytes())?;
    Ok(())
}

fn run_yosys_with_script(script_file: &PathBuf) -> Result<()> {
    let output = Command::new("yosys")
        .arg("-s")
        .arg(script_file)
        .output()?;

    println!("Yosys output:");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    if !output.status.success() {
        eprintln!("Yosys error:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}