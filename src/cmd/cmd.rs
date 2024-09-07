use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    about = "VPM - Verilog Package Manager",
    author,
    version,
    propagate_version = true,
    disable_help_subcommand = true,
    after_help = "Run 'vpm <COMMAND> --help' for more information on a specific command."
)]
pub enum Cmd {
    #[command(
        about = "vpm include <MODULE_URL> [--repo] [--riscv] // Add a module or repository to your project",
        long_about = "Include a module with one command. VPM's internal parser will identify and configure any subdependencies."
    )]
    Include(Include),

    #[command(
        about = "vpm update <MODULE_PATH> // Update a module to its latest version",
        long_about = "Update a specific module to its latest version. This command checks for updates to the specified module and applies them if available."
    )]
    Update(Update),

    #[command(
        about = "vpm remove <PACKAGE_PATH> // Remove a package from your project",
        long_about = "Remove a package from your project. This command uninstalls the specified package and removes it from your project's dependencies, helping you maintain a clean and efficient project structure."
    )]
    Remove(Remove),

    #[command(
        about = "vpm dotf <TOP_MODULE_PATH> // Generate a .f filelist for a module",
        long_about = "Generate a filelist (.f file) for a top module and all its submodules."
    )]
    Dotf(Dotf),

    #[command(
        about = "vpm docs <MODULE_PATH> [--url <URL>] // Generate documentation for a module",
        long_about = "Generate documentation for a module. This command creates comprehensive documentation for the specified module, including descriptions of inputs, outputs, and functionality. It supports both local modules and those hosted on remote repositories."
    )]
    Docs(Docs),

    #[command(
        about = "vpm install <TOOL_NAME> // Install a specified tool",
        long_about = "Install a specified tool. VPM automates the build process and installs missing subdependencies. Support for powerful linters, Icarus Verilog, Verilator, Yosys, GTKWave, the RISC-V toolchain, and more."
    )]
    Install(Install),

    #[command(
        about = "vpm list // List all available modules in the project",
        long_about = "List all available modules in the current project. This command provides an overview of all modules currently included in your project, helping you keep track of your dependencies and project structure."
    )]
    List(List),

    #[command(
        about = "vpm sim <FILE_PATHS>... // Simulate Verilog files",
        long_about = "Simulate one or more Verilog files. This command runs simulations on the specified Verilog files, allowing you to test and verify the behavior of your designs before synthesis or implementation."
    )]
    Sim(Sim),

    #[command(
        about = "vpm synth <TOP_MODULE_PATH> // Synthesize a top module",
        long_about = "Synthesize a top module. This command performs synthesis on the specified top module, converting your RTL design into a gate-level netlist. Supports synthesis for:
    • Board-agnostic (default)
    • Xilinx FPGAs
    • Altera FPGAs (coming soon)
    • Custom board files (coming soon)
    "
    )]
    Synth(Synth),

    #[command(
        about = "vpm load <TOP_MODULE_PATH> // Load a top module onto a target device",
        long_about = "Load a top module onto a target device. This command programs the synthesized design onto the specified hardware, allowing you to test your design on actual FPGA or ASIC hardware."
    )]
    Load(Load),

    #[command(
        about = "vpm run <PROGRAM> // Execute a specified program",
        long_about = "Run a specified program. This command executes the given program, which can be useful for running custom scripts, tools, or compiled designs as part of your Verilog development workflow."
    )]
    Run(Run),

    #[command(
        about = "vpm upgrade // Upgrade VPM to the latest version",
        long_about = "Upgrade VPM to the latest version available."
    )]
    Upgrade(Upgrade),
}

#[derive(Debug, Parser)]
pub struct Upgrade {}

#[derive(Debug, Parser)]
pub struct Include {
    #[arg(long, short, help = "If this flag is set, the URL will be treated as a full repository. If not set, the URL will be treated as a single module.")]
    pub repo: bool,
    #[arg(help = "GitHub URL of the module to include. This should point to a single .v or .sv file in GitHub. If --repo is set, <URL> should not be a full repository URL, but rather 'AUTHOR_NAME/REPO_NAME'")]
    pub url: String,
    #[arg(long, help = "Include RISC-V specific modules. Use this flag when including modules designed specifically for RISC-V architectures.")]
    pub riscv: bool,
}

#[derive(Debug, Parser)]
pub struct Update {
    #[arg(help = "Full module path of the module to update. This should be the complete path to the module file within your project structure.")]
    pub module_path: String,
}

#[derive(Debug, Parser)]
pub struct Remove {
    #[arg(help = "Full module path of the package to remove. This should be the complete path to the package directory within your project structure.")]
    pub package_path: String,
}

#[derive(Debug, Parser)]
pub struct Dotf {
    #[arg(help = "Path to the top module to generate a filelist for. This should be the complete path to the top module file within your project structure.")]
    pub path_to_top_module: String,
}

#[derive(Debug, Parser)]
pub struct Docs {
    #[arg(help = "Path of the module to generate documentation for. This should be the path to the module file within your project structure, starting with 'vpm_modules/'.")]
    pub module_path: String,
    #[arg(long, help = "If this flag is set, the module path will be treated as a link to a .v or .sv file in a GitHub repository. If not set, the path will be treated as a local file path.")]
    pub from_repo: bool,
    #[arg(long, help = "Generate documentation in offline mode for code security.")]
    pub offline: bool,
}

#[derive(Debug, Parser)]
pub struct Install {
    #[arg(help = "Name of the tool to install. This should be a valid tool name recognized by VPM. Available options:
    • verilator: A fast Verilog/SystemVerilog simulator
    • iverilog: Icarus Verilog, a Verilog simulation and synthesis tool
    • yosys: Open-source Verilog synthesis suite
    • gtkwave: Waveform viewer for simulation results
    • verible: SystemVerilog parser, style linter, and formatter
    • edalize: One-stop library for interfacing EDA tools
    • riscv-gnu-toolchain: GNU toolchain for RISC-V, including GCC compiler and associated tools")]
    pub tool_name: String,
}

#[derive(Debug, Parser)]
pub struct Sim {
    #[arg(help = "List of paths to .v or .sv files you want to simulate. Include '_tb' in the testbench file name; otherwise, a base testbench and waveform will be generated.")]
    pub verilog_files: Vec<String>,
    #[arg(long, help = "Generate waveform output. If set, the simulation will produce waveform data and open it in GTKWave.")]
    pub waveform: bool,
}

#[derive(Debug, Parser)]
pub struct List {}

#[derive(Debug, Parser)]
pub struct Synth {
    #[arg(help = "Top module path to synthesize. This should be the path to the main module of your design that you want to synthesize.")]
    pub top_module_path: String,
    #[arg(long, help = "Set this flag if you're working with a RISC-V based design.")]
    pub riscv: bool,
    #[arg(long, help = "Path to RISC-V core. Required if --riscv is set. This should be the path to your RISC-V core implementation.")]
    pub core_path: Option<String>,
    #[arg(long, help = "Specify target board. Use this to optimize the synthesis for a specific FPGA board. Current options:
    • xilinx: Optimize for Xilinx FPGA boards
    • altera: Optimize for Altera FPGA boards (coming soon)
    • custom: Use a custom board file (coming soon)")]
    pub board: Option<String>,
    #[arg(long, help = "Generate synthesis script. If set, the command will produce a Yosys synthesis script instead of running the synthesis directly.")]
    pub gen_yosys_script: bool,
}

#[derive(Debug, Parser)]
pub struct Load {
    #[arg(help = "Path to the top module to load. This should be the path to the synthesized netlist or bitstream file.")]
    pub top_module_path: String,
    #[arg(help = "Path to the .xcd constraint file. This file should contain timing and placement constraints for your design.")]
    pub constraints_path: String,
    #[arg(long, help = "Use RISC-V toolchain. Set this flag if you're working with a RISC-V based design and need to use RISC-V specific tools for loading.")]
    pub riscv: bool,
}

#[derive(Debug, Parser)]
pub struct Run {
    #[arg(help = "Path to the program to run. This can be a compiled binary, a script, or any executable file.")]
    pub program_path: String,
    #[arg(long, help = "Use RISC-V toolchain. Set this flag if you're running a program compiled for RISC-V architecture and need to use RISC-V specific tools or emulators.")]
    pub riscv: bool,
}