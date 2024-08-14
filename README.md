# Verilog Package Manager (VPM)
[![release](https://github.com/getinstachip/vpm/actions/workflows/release.yml/badge.svg)](https://github.com/getinstachip/vpm/actions/workflows/release.yml)
![downloads](https://img.shields.io/github/downloads/getinstachip/vpm/total?logo=github&logoColor=white&style=flat-square)

VPM is a package manager for Verilog projects being piloted at Stanford and UC Berkeley. It's designed to simplify the management, reuse, and communication of IP cores and dependencies in hardware design workflows. Easily import modules for use, manage dependencies, and create documentation to acceperatebyour design process.

## Features

- Install submodules within repositories with dependencies automatically resolved
- Automatically handle synthesis collateral including what's needed for build (COMING SOON!)
- Module version control with a .lock file and customizability with a .toml file
- Automated comprehensive documentation generation for modules

## Installation

To install VPM, you don't need any dependencies! Just run the following command:

```bash
curl -f https://getinstachip.com/install.sh | sh
```

After installation, you can use the `vpm` command in any terminal.

### Basic Commands

- `vpm install <top_module.v> <repo_url>`: install a Verilog (.v) file and all submodule dependencies

![vpm_install](https://github.com/user-attachments/assets/481384eb-5b71-4284-b9e3-08ea807afa34)
- `vpm docs <top_module.v> <repo_url>`: generate documentation for any module

![docs](https://github.com/user-attachments/assets/9f1b9cb4-05e1-4e69-9440-16d498277f0f)

## Configuration

Close your eyes, relax. Submodule dependencies are taken care of with our parser. Use the appropriate fields in `vpm.toml` to adjust the properties of your project. We are working on handling synthesis collateral.

### vpm.toml

Example `vpm.toml` file:

```yaml
[tool.vpm.library]
name = "library_name"
version = "0.3.5"
description = "Most important library in the world"
authors = ["First Last"]
license = [
    {type="BSD-3-Clause", source=["folder_with_artifacts/*.whatever"]},
    {type="CC-4", source=["folder_with_artifacts/*.whatever"]},
    {type="Copyright@RandomStuffyCompany", source=["whatever"]},
]
include = [
    "folder_with_modules/*",
]

[tool.vpm.config]
configparam1=true
configparam2=false

[tool.vpm.docs]
docspath="./not-standard-docs-path"
docsoption1=true
docsoption2=false

[tool.vpm.dependencies]
"https://github.com/ZipCPU/zipcpu" = {"version"="1.1.1", alias="unique_library_name", modules = ["m1", "m2"], branch="not-main", commit="hash"}
"https://github.com/ZipCPU/zipcpu" = {"version"="1.1.1", alias="unique_library_name", modules = ["m1", "m2"], branch="not-main", commit="hash"}

[tool.vpm.dev-dependencies]
"./path/to/file" = {"version"="1.1.1", alias="unique_library_name", modules = ["m1", "m2"], branch="not-main", commit="hash"}
```

### vpm.lock

Example `vpm.lock` file:

```yaml
[tool.vpm.library]
name = "library_name”
version = "0.3.5"
description = "Most important library in the world"
authors = ["First Last"]
license = [
    {type="BSD-3-Clause", source=["folder_with_artifacts/*.whatever"]},
    {type="CC-4", source=["folder_with_artifacts/*.whatever"]},
    {type="Copyright@RandomStuffyCompany", source=["whatever"]},
]
include = [
    "folder_with_modules/*",
]

[tool.vpm.config]
configparam1=true
configparam2=false

[tool.vpm.docs]
docspath="./not-standard-docs-path"
docsoption1=true
docsoption2=false

[tool.vpm.lock-dependencies]
"https://github.com/ZipCPU/zipcpu" = {"version"="1.1.1", alias="unique_library_name", modules = ["m1", "m2"], branch="not-main", commit="hash"}
"https://github.com/ZipCPU/zipcpu" = {"version"="1.1.1", alias="unique_library_name", modules = ["m1", "m2"], branch="not-main", commit="hash"}
```

## Enterprise version

We are receiving overwhelming interest for an enterprise version with additional features to manage internal IP for ASIC/FPGA companies.

[Join the waitlist if you're interested](https://www.waitlistr.com/lists/ce1719b7/vpm-enterprise-version-waitlist), we're launching an enterprise batch trial soon.

## Support

For issues and feature requests, please email sathvikr@getinstachip.com or create an issue on GitHub.
