use anyhow::{Context, Result};
use std::process::Command;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use fastrand;
use crate::cmd::{Execute, Sim};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use fancy_regex::Regex;

impl Execute for Sim {
    async fn execute(&self) -> Result<()> {
        let mut verilog_files = self.verilog_files.clone();
        
        if !testbench_exists(&verilog_files) {
            generate_and_add_testbench(&mut verilog_files)?;
        }

        let output_path = compile_verilog(&verilog_files)?;

        if self.waveform {
            run_simulation_with_waveform(&output_path)?;
        } else {
            run_simulation(&output_path)?;
        }

        Ok(())
    }
}

fn testbench_exists(verilog_files: &[String]) -> bool {
    verilog_files.iter().any(|file| file.to_lowercase().contains("_tb.v"))
}

fn generate_and_add_testbench(verilog_files: &mut Vec<String>) -> Result<()> {
    if let Some(first_file) = verilog_files.first() {
        let is_systemverilog = first_file.ends_with(".sv");
        let testbench_content = generate_testbench(first_file, is_systemverilog)
            .context("Failed to generate testbench. Please check if the Verilog file is valid.")?;
        let testbench_path = format!("{}_tb.{}", first_file.trim_end_matches(if is_systemverilog { ".sv" } else { ".v" }), if is_systemverilog { "sv" } else { "v" });
        fs::write(&testbench_path, testbench_content)
            .context("Failed to write testbench file. Please check if you have write permissions.")?;
        // Remove comments from the original Verilog file
        remove_comments_from_file(first_file)?;

        verilog_files.push(testbench_path.clone());
        println!("Generated testbench: {}", testbench_path);
        Ok(())
    } else {
        Err(anyhow::anyhow!("No Verilog files provided. Please specify at least one Verilog file."))
    }
}

fn run_simulation_with_waveform(output_path: &Path) -> Result<()> {
    println!("Running simulation with waveform...");
    println!("Output path: {:?}", output_path);
    let testbench_dir = output_path.parent().unwrap();
    let vcd_path = testbench_dir.join("waveform.vcd");
    let current_dir = std::env::current_dir()
        .context("Failed to get current directory. Please check your file system permissions.")?;
    std::env::set_current_dir(testbench_dir)
        .context("Failed to change directory to testbench location. Please ensure the directory exists and you have necessary permissions.")?;
    
    let mut cmd = Command::new("vvp");
    cmd.arg(output_path.file_name().unwrap());
    cmd.arg(format!("-vcd={}", vcd_path.file_name().unwrap().to_str().unwrap()));
    
    let output = cmd.output()
        .context("Failed to run simulation with VCD output. Debug steps:\n1. Ensure 'vvp' is installed: Run 'vvp --version' in terminal.\n2. Check if 'vvp' is in your PATH: Run 'which vvp' (Unix) or 'where vvp' (Windows).\n3. If not found, install Icarus Verilog or add its bin directory to your PATH.")?;
    
    // Call gtkwave on the generated waveform file
    println!("Opening waveform in GTKWave...");
    let gtkwave_status = Command::new("gtkwave")
        .arg("waveform.vcd")
        .spawn()
        .context("Failed to open GTKWave. Debug steps:\n1. Ensure GTKWave is installed: Run 'gtkwave --version' in terminal.\n2. Check if 'gtkwave' is in your PATH: Run 'which gtkwave' (Unix) or 'where gtkwave' (Windows).\n3. If not found, install GTKWave or add its installation directory to your PATH.")?;

    // We don't wait for GTKWave to exit, as it's a GUI application
    println!("GTKWave opened successfully. You can now view the waveform.");
    
    std::env::set_current_dir(current_dir)
        .context("Failed to change back to the original directory. This is unexpected, please check your file system.")?;
    
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Simulation failed. Error details:\n{}\n\nDebugging steps:\n1. Check your Verilog code for syntax errors.\n2. Ensure all module dependencies are correctly included.\n3. Verify testbench inputs and timing.\n4. Run the simulation without waveform generation to isolate the issue.", error_message));
    }
    
    println!("Generated waveform file: {}", vcd_path.display());
    println!("If GTKWave didn't open automatically, you can manually open the waveform file using GTKWave.");
    Ok(())
}

pub fn generate_testbench(module_path: &str, is_systemverilog: bool) -> Result<String> {
    println!("Generating testbench for module: {}", module_path);

    let (module_name, ports, parameters) = extract_module_info(module_path)?;

    println!("Module name: {}", module_name);
    println!("Ports: {:?}", ports);
    println!("Parameters: {:?}", parameters);

    let mut testbench = String::new();
    testbench.push_str(&generate_testbench_header(&module_name));
    testbench.push_str(&declare_parameters(&parameters));
    testbench.push_str(&declare_wires_for_ports(&ports, is_systemverilog));
    testbench.push_str(&instantiate_module(&module_name, &ports, &parameters));
    testbench.push_str(&generate_clock(&ports));
    testbench.push_str(&generate_initial_block(&ports, is_systemverilog));
    testbench.push_str(&generate_check_outputs_task(&ports, is_systemverilog));
    testbench.push_str("endmodule\n");

    Ok(testbench)
}

fn extract_module_info(module_path: &str) -> Result<(String, Vec<(String, Option<String>, String)>, Vec<(String, String)>)> {
    let file = File::open(module_path)
        .context(format!("Failed to open module file: {}. Please check if the file exists and you have read permissions.", module_path))?;
    let reader = BufReader::new(file);

    let mut module_name = String::new();
    let mut ports = Vec::new();
    let mut parameters = Vec::new();
    let mut in_module = false;
    let module_regex = Regex::new(r"module\s+(\w+)\s*(?:\(|#)").unwrap();
    let port_regex = Regex::new(r"(input|output|inout)\s+(?:reg|wire|logic)?\s*(\[.*?\])?\s*(\w+)").unwrap();
    let parameter_regex = Regex::new(r"parameter\s+(\w+)\s*=\s*([^,\)]+)").unwrap();
    let inline_parameter_regex = Regex::new(r"(\w+)\s*=\s*([^,\)]+)").unwrap();

    for line in reader.lines() {
        let line = line?;
        if !in_module {
            if let Ok(Some(captures)) = module_regex.captures(&line) {
                module_name = captures[1].to_string();
                in_module = true;
            }
        } else {
            for capture_result in port_regex.captures_iter(&line) {
                if let Ok(capture) = capture_result {
                    let direction = capture[1].to_string();
                    let bus_width = capture.get(2).map(|m| m.as_str().to_string());
                    let name = capture[3].to_string();
                    ports.push((direction, bus_width, name));
                }
            }
            for capture_result in parameter_regex.captures_iter(&line) {
                if let Ok(capture) = capture_result {
                    let name = capture[1].to_string();
                    let value = capture[2].to_string();
                    if let Some(existing) = parameters.iter_mut().find(|(n, _)| n == &name) {
                        existing.1 = value;
                    } else {
                        parameters.push((name, value));
                    }
                }
            }
            for capture_result in inline_parameter_regex.captures_iter(&line) {
                if let Ok(capture) = capture_result {
                    let name = capture[1].to_string();
                    let value = capture[2].to_string();
                    if !parameters.iter().any(|(n, _)| n == &name) {
                        parameters.push((name, value));
                    }
                }
            }
            if line.contains(");") {
                break;
            }
        }
    }

    if module_name.is_empty() {
        return Err(anyhow::anyhow!("Could not find module declaration in {}. Please ensure the file contains a valid Verilog or SystemVerilog module.", module_path));
    }

    Ok((module_name, ports, parameters))
}

fn generate_testbench_header(module_name: &str) -> String {
    format!("`timescale 1ns / 1ps\n\nmodule {}_tb;\n\n", module_name)
}

fn declare_parameters(parameters: &[(String, String)]) -> String {
    let mut declarations = String::new();
    for (name, value) in parameters {
        let mut line = format!("    parameter {} = {}", name, value);
        if !line.ends_with(')') && line.contains('(') {
            line.push(')');
        }
        declarations.push_str(&line);
        declarations.push_str(";\n");
    }
    declarations.push_str("\n");
    declarations
}

fn declare_wires_for_ports(ports: &[(String, Option<String>, String)], is_systemverilog: bool) -> String {
    let mut declarations = String::new();
    for (direction, bus_width, name) in ports {
        let wire_type = if is_systemverilog { "logic" } else { "reg" };
        let declaration = match bus_width {
            Some(width) => format!("    {} {} {};\n", if direction == "input" { wire_type } else { "wire" }, width, name),
            None => format!("    {} {};\n", if direction == "input" { wire_type } else { "wire" }, name),
        };
        declarations.push_str(&declaration);
    }
    declarations.push_str("\n");
    declarations
}

fn instantiate_module(module_name: &str, ports: &[(String, Option<String>, String)], parameters: &[(String, String)]) -> String {
    let mut instantiation = String::new();
    if !parameters.is_empty() {
        instantiation.push_str(&format!("    {} #(\n", module_name));
        for (i, (name, _)) in parameters.iter().enumerate() {
            instantiation.push_str(&format!("        .{}({}){}\n", name, name, if i < parameters.len() - 1 { "," } else { "" }));
        }
        instantiation.push_str("    ) uut (\n");
    } else {
        instantiation.push_str(&format!("    {} uut (\n", module_name));
    }
    for (i, (_, _, name)) in ports.iter().enumerate() {
        instantiation.push_str(&format!("        .{}({}){}\n", name, name, if i < ports.len() - 1 { "," } else { "" }));
    }
    instantiation.push_str("    );\n\n");
    instantiation
}

fn generate_clock(ports: &[(String, Option<String>, String)]) -> String {
    if let Some((_, _, clock_name)) = ports.iter().find(|(_, _, name)| name.to_lowercase().contains("clk")) {
        format!("    localparam CLOCK_PERIOD = 10;\n    initial begin\n        {} = 0;\n        forever #(CLOCK_PERIOD/2) {} = ~{};\n    end\n\n", clock_name, clock_name, clock_name)
    } else {
        String::new()
    }
}

fn generate_initial_block(ports: &[(String, Option<String>, String)], is_systemverilog: bool) -> String {
    let mut initial_block = String::from("    initial begin\n        $dumpfile(\"waveform.vcd\");\n        $dumpvars(0, uut);\n\n");
    for (direction, bus_width, name) in ports {
        if direction == "input" && !name.to_lowercase().contains("clk") {
            match bus_width {
                Some(width) => {
                    let width_value = width.trim_matches(|c| c == '[' || c == ']').split(':').next().unwrap_or("0");
                    if is_systemverilog {
                        initial_block.push_str(&format!("        {} = '{{{} {{1'b0}}}};\n", name, width_value));
                    } else {
                        initial_block.push_str(&format!("        {} = {{'b0}};\n", name));
                    }
                },
                None => initial_block.push_str(&format!("        {} = 1'b0;\n", name)),
            }
        }
    }
    initial_block.push_str("\n        #100;\n\n        #1000;\n        $finish;\n    end\n\n");
    initial_block
}

fn generate_check_outputs_task(ports: &[(String, Option<String>, String)], is_systemverilog: bool) -> String {
    let mut task = String::from("    task check_outputs;\n");
    if is_systemverilog {
        task.push_str("        input string test_case;\n");
    } else {
        task.push_str("        input [8*32-1:0] test_case;\n");
    }
    task.push_str("        begin\n            $display(\"Checking outputs for %s\", test_case);\n");
    for (direction, bus_width, name) in ports {
        if direction == "output" {
            let format_specifier = match bus_width {
                Some(_) => "%h",
                None => "%b",
            };
            task.push_str(&format!("            $display(\"  {} = {}\", {});\n", name, format_specifier, name));
        }
    }
    task.push_str("        end\n    endtask\n\n");
    task
}

fn remove_comments_from_file(file_path: &str) -> Result<()> {
    let file = File::open(file_path)
        .context(format!("Failed to open file: {}", file_path))?;
    let reader = BufReader::new(file);
    let mut content = String::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("//").collect();
        if !parts.is_empty() {
            let code_part = parts[0].trim_end();
            if !code_part.is_empty() {
                content.push_str(code_part);
                content.push('\n');
            }
        }
    }

    fs::write(file_path, content)
        .context(format!("Failed to write updated content to file: {}", file_path))?;

    Ok(())
}

pub fn compile_verilog(verilog_files: &Vec<String>) -> Result<PathBuf> {
    println!("Compiling Verilog files...");

    let first_file = &verilog_files[0];
    let output_dir = Path::new(first_file).parent().unwrap();
    let random_output_name = generate_random_output_name();
    let output_path = output_dir.join(&random_output_name);
    let command_status = run_iverilog_command(output_path.to_str().unwrap(), verilog_files)?;

    if !command_status.success() {
        return Err(anyhow::anyhow!("Failed to compile Verilog files. Please check your Verilog code for syntax errors."));
    }

    if !output_path.exists() {
        return Err(anyhow::anyhow!("Output binary not found: {:?}. Compilation may have failed silently.", output_path));
    }
    println!("Compiled output: {:?}", output_path);
    Ok(output_path)
}

fn generate_random_output_name() -> String {
    std::iter::repeat_with(fastrand::alphanumeric)
        .take(10)
        .collect()
}

fn run_iverilog_command(output_name: &str, verilog_files: &[String]) -> Result<std::process::ExitStatus> {
    let mut command = Command::new("iverilog");
    command.arg("-o").arg(output_name);
    for file in verilog_files {
        command.arg(file);
    }
    command.status()
        .context("Failed to execute Icarus Verilog compilation. Please ensure Icarus Verilog is installed and accessible.")
}

pub fn run_simulation(output_path: &PathBuf) -> Result<()> {
    println!("Running simulation...");

    let current_dir = env::current_dir()
        .context("Failed to get current directory. Please check your file system permissions.")?;
    
    let binary_path: PathBuf = current_dir.join(output_path);

    let status = Command::new(&binary_path)
        .status()
        .context(format!("Failed to execute simulation. Please ensure the binary at {:?} is executable.", binary_path))?;

    if !status.success() {
        eprintln!("Warning: Simulation completed with non-zero exit status. This may indicate errors in your Verilog code.");
    } else {
        println!("Simulation completed successfully.");
    }

    if let Err(e) = fs::remove_file(&binary_path) {
        eprintln!("Warning: Failed to remove temporary binary file: {}. You may want to delete it manually.", e);
    }

    Ok(())
}
