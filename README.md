# Verilog Package Manager (VPM)
![downloads](https://img.shields.io/github/downloads/getinstachip/vpm/total?logo=github&logoColor=white&style=flat-square)

VPM is a package manager for Verilog projects being piloted at Stanford and UC Berkeley. It's designed to simplify the management, reuse, and communication of IP cores and dependencies in hardware design workflows. Easily import modules for use, manage dependencies, and create documentation to accelerate your design process.

You'll be able to:
- `vpm include` full module hierarchies using a single command
- Generate documentation for any `.v` or `.sv` module
- Simulate using `vpm sim`
- Automatically generate/handle `.f` files, `.svh`, `.xcd`, `.tcl`, etc.

## Installation (no setup, just run the command)

To install VPM, you don't need any dependencies! Just run the following command:

```bash
curl -f https://getinstachip.com/install.sh | sh
```

After installation, you can use the `vpm` command in any terminal.

## Full command list
- `vpm include <repo_url>`: Opens a menu where you can type to choose any module from the repo. Include its entire hierarchy.
- `vpm docs <module.sv> <repo_url>`: Generate documentation for any module (highlighting bugs and edge cases)
- `vpm install <tool>`: Auto-integrate an open-source tool without manual setup
- `vpm update <module.sv>`: Update module to the latest version
- `vpm uninstall <module.sv>`: Remove a module from your project
- `vpm list`: List all modules in our standard library
- `vpm dotf <module.sv>`:  Generate a `.f` filelist when exporting your project
- `vpm sim <module.sv> <testbench.sv>`
  
### include
`vpm include <module.sv>`: includes a `.v` or `.sv` file and all submodule dependencies from the given repo and updates the `vpm.toml` file with the new module's details
- Options:
  - `<module.sv>`: Module to install
  - `<repo_url>`: Link to the repository where the module is stored

![include](https://github.com/user-attachments/assets/481384eb-5b71-4284-b9e3-08ea807afa34)

### docs
`vpm docs <module.sv> <repo_url>`: generates a complete Markdown README documentation file for the given module 
- Options:
  - `<module.sv>`: Verilog or SystemVerilog module to generate documentation for
  - `<repo_url>`: Link to the repository where the module is stored
&nbsp;
- Generation location can be overwritten in `vpm.toml`. All documentation contains the following sections:
  1. Overview and module description
  2. Pinout diagram
  3. Table of ports
  4. Table of parameters
  5. Important implementation details
  6. Simulation output and details (Coming soon!)
  7. List of any major bugs or caveats (if they exist)

![docs](https://github.com/user-attachments/assets/9f1b9cb4-05e1-4e69-9440-16d498277f0f)

### install
`vpm install <tool>`: Automatically installs and sets up an open-source tool for integration into your project
- Options:
  - `<tool>`: Tool to install
&nbsp;
- Currently supported tools
  - Verilator
  - Chipyard
  - OpenROAD

![install](https://github.com/user-attachments/assets/78569e63-b2d7-41e2-9690-8f18f50516bc)

### update
`vpm update <module.sv>`: Updates an included Verilog or SystemVerilog module to the latest version and updates version control accordingly
- Options:
  - `<module.sv>`: Module to update

*Example video coming soon!*

### uninstall
`vpm uninstall <module.sv>`: Removes an included Verilog or SystemVerilog module from your project and updates version control accordingly
- Options:
  - `<module.sv>`: Module to uninstall

*Example video coming soon!*

### list
`vpm list`: Lists all modules in our [standard Verilog library](https://github.com/getinstachip/openchips)
- Current module lists:
  - Common modules
  - RISC-V

![list](https://github.com/user-attachments/assets/0e36a7cd-70bd-406d-9696-8a5550fff99b)

### dotf
`vpm dotf <module.sv>`: generates a `.f` file list for a Verilog or SystemVerilog module and for all locally scoped defines for the submodules and links everything accordingly
- Options:
  - `<module.sv>`: Local top Verilog module to generate the file list for

*Example video coming soon!*

## Configuration

Close your eyes and relax. Our parser takes care of submodule dependencies. Use the appropriate fields in `vpm.toml` to adjust your project's properties. We are working on handling synthesis collateral.

Example `vpm.toml` file:

```toml
[library]
name = "library_name"
version = "0.3.5"
description = "Most important library in the world"
authors = ["First Last"]
license = [
    {type="GPLv3", source=["folder_with_artifacts/*.whatever"]},
    {type="CC-4", source=["folder_with_artifacts/*.whatever"]},
    {type="Copyright@RandomStuffyCompany", source=["whatever"]},
]
include = [
    "folder_with_modules/*",
]

[config]
configparam1=true
configparam2=false

[docs]
docspath="./not-standard-docs-path"
docsoption1=true
docsoption2=false

[dependencies]
"https://github.com/ZipCPU/zipcpu" = {"version"="1.1.1", alias="unique_library_name", modules = ["m1", "m2"], branch="not-main", commit="hash"}
"https://github.com/ZipCPU/zipcpu" = {"version"="1.1.1", alias="unique_library_name", modules = ["m1", "m2"], branch="not-main", commit="hash"}

[dev-dependencies]
"./path/to/file" = {"version"="1.1.1", alias="unique_library_name", modules = ["m1", "m2"], branch="not-main", commit="hash"}
```

- `[library]`: Contains the metadata for the library/project
  - `name`: Name of the library/project
  - `version`: Version of the library/project
  - `description`: Description of the library/project
  - `authors`: List of authors
  - `license`: List of licenses and their source locations
  - `include`: List of directories to include in the library/project
&nbsp;
- `[config]`: Contains the configuration parameters for the library/project. Custom options will be added here.
&nbsp;
- `[docs]`: Contains the documentation generation parameters for the library. Custom options will be added here.
  - `docspath`: Path to folder with all generated documentation
&nbsp;
- `[dependencies]`: Contains the external dependencies for the library/project
  - `url`: URL of the dependency repository
  - `version`: User-specified version of the dependency
  - `alias`: Alias for the dependency
  - `modules`: List of modules in the dependency, including submodule dependencies
  - `branch`: Branch of the repository the dependency is on
  - `commit`: Commit hash of the repository the dependency is on
&nbsp;
- `[dev-dependencies]`: Contains the development dependencies for the library/project
  - `url`: URL of the dependency repository
  - `version`: User-specified version of the dependency
  - `alias`: Alias for the dependency
  - `modules`: List of modules in the dependency, including submodule dependencies
  - `branch`: Branch of the repository the dependency is on
  - `commit`: Commit hash of the repository the dependency is on

## Enterprise version

We are receiving overwhelming interest for an enterprise version with additional features and integrations to manage internal IP for ASIC/FPGA companies.

[Join the waitlist if you're interested](https://www.waitlistr.com/lists/ce1719b7/vpm-enterprise-version-waitlist). We're launching an enterprise batch pilot soon.

## Support

For issues and feature requests, please email sathvikr@getinstachip.com or create an [issue on GitHub](https://github.com/getinstachip/vpm/issues).
