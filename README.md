# Verilog Package Manager (VPM)

VPM is a package manager for Verilog projects, written in Rust. It's designed to simplify the management of IP cores and dependencies in hardware design workflows.

## Features

- Manage Verilog project dependencies
- Install packages from various sources (GitHub, local repositories, etc.)
- Turn generic IP into IP optimized for your use case (coming soon!)

## Installation

To install VPM, you don't need any dependencies! Just run the following command:

```bash
curl -sSL https://raw.githubusercontent.com/getinstachip/vpm/main/install.sh | bash
```

After installation, you can use the `vpm` command in any terminal.

### Basic Commands

- Install a package: `vpm install <author/repo_name>`
- Install a package tuned to your use case: `vpm install --flex <author/repo_name>
- List installed packages: `vpm list installed`
- List outdated packages: `vpm list outdated`
- List available packages: `vpm list available`
- Update a package: `vpm update <author/repo_name>`
- Remove a package: `vpm remove <author/repo_name>`

## Configuration

### vpm.toml

Example `vpm.toml` file:

```yaml
[repositories]
ZipCPU/zipcpu = "0.0.1"
LibSV/libsv = "1.0.0"
OpenCores/opencores = "2.1.3"
VerilogI2C/verilog-i2c = "0.0.2"
VerilogUART/verilog-uart = "0.1.0"
VerilogPCIe/verilog-pcie = "0.0.5"
VerilogWishbone/verilog-wishbone = "0.2.1"
VerilogAXI/verilog-axi = "0.3.0"
BaseJumpSTL/basejump-stl = "1.2.0"
OH/open-hardware = "0.9.4"
LibSV/libsv = "1.0.0"
OpenRAM/openram = "1.0.0"
Huancun/huancun = "0.1.1"
OpenFPGA/openfpga = "0.5.0"
FABulous/fabulous = "0.3.2"
getinstachip/openchips = "0.1.4"
```

## Support

For issues and feature requests, please email sathvikr@getinstachip.com.
