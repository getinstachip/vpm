# Verilog Package Manager (VPM)
[![release](https://github.com/getinstachip/vpm/actions/workflows/release.yml/badge.svg)](https://github.com/getinstachip/vpm/actions/workflows/release.yml)
![downloads](https://img.shields.io/github/downloads/getinstachip/vpm/total?logo=github&logoColor=white&style=flat-square)

VPM is a package manager for Verilog projects, being piloted at Stanford and UC Berkeley. It's designed to simplify the management of IP cores and dependencies in hardware design workflows.

## Features

- Install submodules within repositories with dependencies automatically resolved
- Automatically handle synthesis collateral including what's needed for build (COMING SOON!)
- God-tier version control with a .lock file

## Installation

To install VPM, you don't need any dependencies! Just run the following command:

```bash
curl -sSfL https://raw.githubusercontent.com/getinstachip/vpm/main/install.sh | sh
```

After installation, you can use the `vpm` command in any terminal.

### Basic Commands

- Install a package: `vpm install <author/repo_name>`
- Include a module and its sub-modules: `vpm include <module_name.v> <author/repo_name>`

## Very useful stuff

### vpm include "top_module"
After running `vpm include "top_module"`, the Verilog Package Manager parses the file and downloads all the submodules too. It generates .vh files and handles synthesis collateral.

Example: running `vpm include pfcache` finds all dependences and includes/configures them for you.
```
your_project/
├─ vpm_modules/
│  ├─ pfcache/
│     ├─ pfcache.v
│     ├─ pfcache.vh
│     ├─ ffetch.v
│     ├─ ffetch.vh
│     ├─ fwb_module.v
│     ├─ fwb_module.vh
│     └─ pfcache.toml
└─ sim/
   └─pfcache_tb.v
```

## Configuration

### vpm.toml

Example `vpm.toml` file:

```yaml
// you can include entire repositories
[repositories]
https://github.com/ZipCPU/zipcpu = "ee644d4"
https://github.com/bensampson5/libsv = "c5aff5d"
https://github.com/alexforencich/verilog-pcie = "25156a9"

// or just specific modules
[modules]
pfcache = "https://github.com/ZipCPU/zipcpu/commit/ee644d451910a6b6fbd430a4e79edb4e95852d9f"
axis_arb_mux = "https://github.com/alexforencich/verilog-pcie/commit/25156a9a162c41c60f11f41590c7d006d015ae5a"
```
Close your eyes, relax. Submodule dependencies are taken care of with our parser. We are working on handling synthesis collateral.

### vpm.lock
```yaml
[repositories]
"https://github.com/ZipCPU/zipcpu" = "ee644d4"
"https://github.com/bensampson5/libsv" = "c5aff5d"
"https://github.com/alexforencich/verilog-pcie" = "25156a9"

[modules]
pfcache = "https://github.com/ZipCPU/zipcpu/commit/ee644d451910a6b6fbd430a4e79edb4e95852d9f"
ffetch = "https://github.com/ZipCPU/zipcpu/commit/ee644d451910a6b6fbd430a4e79edb4e95852d9f"
fwb_module = "https://github.com/ZipCPU/zipcpu/commit/ee644d451910a6b6fbd430a4e79edb4e95852d9f"
axis_arb_mux = "https://github.com/alexforencich/verilog-pcie/commit/25156a9a162c41c60f11f41590c7d006d015ae5a"
// ... (subdependencies for axis_arb_mux) 
```

## Enterprise version

We are receiving overwhelming interest for an enterprise version to manage internal IP for ASIC/FPGA companies.

[Join the waitlist if you're interested](https://www.waitlistr.com/lists/ce1719b7/vpm-enterprise-version-waitlist), we're launching an enterprise batch trial soon.

## Support

For issues and feature requests, please email sathvikr@getinstachip.com.
