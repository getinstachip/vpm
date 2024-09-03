# Verilog Package Manager (VPM)

VPM is a powerful package manager for Verilog projects, currently being piloted at Stanford and UC Berkeley. It's designed to streamline the management, reuse, and communication of IP cores and dependencies in hardware design workflows, significantly accelerating your design process.

## Features

- **Module Management**: Easily include, update, and remove modules in your project.
- **Documentation Generation**: Automatically create comprehensive documentation for your Verilog modules.
- **Dependency Handling**: Manage project dependencies with ease.
- **Simulation Support**: Simulate your Verilog files directly through VPM.
- **Tool Integration**: Seamlessly install and set up open-source tools for your project.
- **File Generation**: Automatically generate necessary files like .f, .svh, .xcd, and .tcl.

## Installation

VPM is designed for easy installation with no additional dependencies. Simply run:

```bash
curl -sSfL https://raw.githubusercontent.com/getinstachip/vpm/main/install.sh | sh
```

After installation, the vpm command will be available in any terminal.

## Commands

- `vpm include <path_to_module.sv>`: Include any module from a repo (and all its submodules).
- `vpm docs <module.sv>`: Generate documentation for any module (highlighting bugs and edge cases)
- `vpm install <tool>`: Auto-integrate an open-source tool without manual setup
- `vpm update <module.sv>`: Update module to the latest version
- `vpm uninstall <module.sv>`: Remove a module from your project
- `vpm list`: List all modules in our standard library
- `vpm dotf <module.sv>`:  Generate a `.f` filelist when exporting your project
- `vpm sim <module.sv> <testbench.sv>`: Simulate Verilog module using Iverilog
  
### vpm include
Include a module or repository in your project.

This command:
- Downloads the specified module or repository
- Analyzes the module hierarchy
- Includes all necessary submodules and generates appropriate header files
- Updates the vpm.toml file with new module details

This command comes in two forms:
1. Include a module and all its submodules:
```bash
vpm include <URL_TO_TOP_MODULE.sv>
```
Example:
```bash
vpm include https://github.com/ZipCPU/zipcpu/blob/master/rtl/core/prefetch.v
```
2. Include a repository:
```bash
vpm include --repo <AUTHOR_NAME/REPO_NAME>
```
Press tab to select multiple modules and press ENTER to install. If no modules are selected, all modules in the repository will be installed.
Example:
```bash
vpm include --repo ZipCPU/zipcpu
```

### vpm update
Update a package to the latest version.

This command:
- Checks for the latest version of the specified module
- Downloads and replaces the current version with the latest
- Updates all dependencies and submodules
- Modifies the vpm.toml file to reflect the changes

```bash
vpm update <PACKAGE_PATH>
```

<PACKAGE_PATH>: Full module path of the package to update

Example:
```bash
vpm update my_project/modules/counter
```

### vpm remove
Remove a package from your project.

This command:
- Removes the specified module from your project
- Updates the vpm.toml file to remove the module entry
- Cleans up any orphaned dependencies

```bash
vpm remove <PACKAGE_PATH>
```

<PACKAGE_PATH>: Full module path of the package to remove

Example:
```bash
vpm remove my_project/modules/unused_module
```

### vpm dotf
Generate a .f file list for a Verilog or SystemVerilog module.

```bash
vpm dotf <PATH_TO_TOP_MODULE>

<PATH_TO_TOP_MODULE>: Path to the top module to generate the file list for

Example:
```bash
vpm dotf src/top_module.v
```

This command:
- Analyzes the specified top module
- Identifies all submodules and dependencies
- Generates a .f file containing all necessary file paths
- Includes all locally scoped defines for submodules

### vpm docs
Generate comprehensive documentation for a module.

```bash
vpm docs <MODULE> [URL]

<MODULE>: Name of the module to generate documentation for
[URL]: Optional URL of the repository to generate documentation for. If not specified, VPM will assume the module is local.

Example:
```bash
vpm docs my_module
vpm docs external_module https://github.com/example/external_repo
```

This command generates a Markdown README file containing:
- Overview and module description
- Pinout diagram
- Table of ports
- Table of parameters
- Important implementation details
- Simulation output and details (Coming soon!)
- List of any major bugs or caveats (if they exist)

### vpm install
Install and set up an open-source tool for integration into your project.

```bash
vpm install <TOOL_NAME>

<TOOL_NAME>: Name of the tool to install
Example:
```bash
vpm install verilator
```

Currently supported tools:
- Verilator
- Chipyard
- OpenROAD

This command:
- Downloads the specified tool
- Configures the tool for your system
- Integrates it with your VPM project setup

### vpm sim
Simulate Verilog files.

```bash
vpm sim <VERILOG_FILES>...

<VERILOG_FILES>: List of Verilog files to simulate

Example:
```bash
vpm sim testbench.v module1.v module2.v
```

This command:
- Compiles the specified Verilog files
- Runs the simulation
- Provides output and analysis of the simulation results

### vpm list
List all modules in VPM's standard library.
```bash
vpm list
```

This command displays all available modules in the standard Verilog library, including:
- Common modules
- RISC-V modules
- Configuration

VPM uses a vpm.toml file for project configuration. This file allows you to specify project properties, dependencies, and custom settings.

Example vpm.toml file:
```toml
[library]
name = "my_project"
version = "0.3.5"
description = "A comprehensive Verilog project"
authors = ["John Doe", "Jane Smith"]
license = [
    {type = "GPLv3", source = ["src/*.v"]},
    {type = "CC-4", source = ["docs/*.md"]},
]
include = ["modules/*", "tests/*"]

[config]
use_systemverilog = true
enable_assertions = true

[docs]
output_path = "./documentation"
include_waveforms = true
generate_pdf = false

[dependencies]
"https://github.com/example/module1" = { version = "1.2.0", alias = "core_module", modules = ["alu", "register_file"], branch = "main" }
"https://github.com/example/module2" = { version = "0.9.1", alias = "peripheral_module", modules = ["uart", "i2c"], commit = "a1b2c3d4" }

[dev-dependencies]
"./local_test_module" = { version = "0.1.0", alias = "test_utils" }
```

### Enterprise Version
We're developing an enterprise version with advanced features for managing internal IP in ASIC/FPGA companies. Join the waitlist for our upcoming enterprise batch pilot.

### Support and Contribution
For issues, feature requests, or contributions:
Email: support@getinstachip.com
GitHub Issues: Create an issue
Contributing: Please read our CONTRIBUTING.md file for guidelines on how to contribute to VPM.

### License
VPM is released under the MIT License.

### Acknowledgements
We'd like to thank our early adopters for their valuable feedback and support in developing VPM.