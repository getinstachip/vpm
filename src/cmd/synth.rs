use anyhow::{Result, Context};
use std::path::PathBuf;
use std::process::Command;
use std::fs::File;
use std::io::Write;

use crate::cmd::{Execute, Synth};

impl Execute for Synth {
    async fn execute(&self) -> Result<()> {
        synthesize_design(
            &self.top_module_path,
            self.riscv,
            self.core_path.as_ref(),
            &self.board,
            self.gen_yosys_script
        )
    }
}

fn synthesize_design(
    top_module_path: &str,
    riscv: bool,
    core_path: Option<&String>,
    board: &Option<String>,
    gen_yosys_script: bool
) -> Result<()> {
    let top_module_path = PathBuf::from(top_module_path);
    let (input_file, module_name, parent_dir, _) = extract_path_info(&top_module_path);
    
    let script_content = match board {
        Some(board) if board.to_lowercase() == "xilinx" => {
            let board_name = "artix7";
            let output_file = format!("{}/{}_{}_{}_synth.v", parent_dir, module_name, board_name, "xilinx");
            generate_xilinx_script_content(&input_file, riscv, core_path.cloned(), &module_name, &output_file)?
        },
        None => {
            let output_file = format!("{}/{}_synth.v", parent_dir, module_name);
            generate_yosys_script_content(&input_file, &module_name, &output_file)
        },
        Some(other) => {
            return Err(anyhow::anyhow!("Unsupported board: {}", other));
        }
    };

    if gen_yosys_script {
        let script_file = PathBuf::from(&parent_dir).join(format!("{}_synth_script.ys", module_name));
        write_script_to_file(&script_file, &script_content)?;
        println!("Yosys script generated at: {:?}", script_file);
    }

    run_yosys_with_script_content(&script_content)?;
    println!("Synthesis completed successfully.");
    Ok(())
}

fn extract_path_info(top_module_path: &PathBuf) -> (String, String, String, String) {
    let input_file = top_module_path.to_str().unwrap().to_string();
    let top_module = top_module_path.file_stem().unwrap().to_str().unwrap().to_string();
    let parent_dir = top_module_path.parent().unwrap().to_string_lossy().to_string();
    let output_file = format!("{}/{}_synth.v", parent_dir, top_module);
    (input_file, top_module, parent_dir, output_file)
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

fn run_yosys_with_script_content(script_content: &str) -> Result<()> {
    let output = Command::new("yosys")
        .arg("-p")
        .arg(script_content)
        .output()
        .context("Failed to execute Yosys")?;

    println!("Yosys output:");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Yosys synthesis failed: {}", error_message));
    }

    Ok(())
}