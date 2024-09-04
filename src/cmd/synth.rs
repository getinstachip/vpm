use anyhow::{Context, Result};
use std::process::Command;
use std::path::PathBuf;
use crate::cmd::{Execute, Synth};

impl Execute for Synth {
    async fn execute(&self) -> Result<()> {
        // Synthesize the Verilog design
        self.synthesize_design()?;

        Ok(())
    }
}

impl Synth {
    pub fn synthesize_design(&self) -> Result<()> {
        println!("Synthesizing design...");

        // Prepare the Yosys script
        let script_path = self.create_yosys_script()?;

        // Run Yosys with the created script
        self.run_yosys(&script_path)?;

        println!("Synthesis completed successfully.");
        Ok(())
    }

    fn create_yosys_script(&self) -> Result<PathBuf> {
        let output_file = self.output_file.clone().unwrap_or_else(|| format!("{}_synth.v", self.top_module));

        let script_content = format!(
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
            self.input_file,
            self.top_module,
            output_file
        );

        let script_path = PathBuf::from(format!("{}_synth_script.ys", self.top_module));
        std::fs::write(&script_path, script_content).context("Failed to create Yosys script")?;

        Ok(script_path)
    }

    fn run_yosys(&self, script_path: &PathBuf) -> Result<()> {
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
}
