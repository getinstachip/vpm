# pfcache.v

Here's the comprehensive Markdown documentation for the `pfcache` Verilog module, including a pinout diagram:

# pfcache Module Documentation

## Overview

The `pfcache` module is a prefetch cache designed to keep a CPU fed with instructions at one per clock with minimal stalls. It is part of the Zip CPU project, a small, lightweight, RISC CPU soft core.

## Module Description

This module implements a cache system that aims to minimize latency in instruction fetching. It handles reading from block RAM, checking tag matches, and manages bus interactions for cache misses. The cache is designed to avoid the second clock cycle typically needed for tag checking when possible.

## Parameters

- `LGCACHELEN`: Log2 of the cache size in words (default: 12)
- `ADDRESS_WIDTH`: Width of the address bus (default: 30)
- `LGLINES`: Log2 of the number of separate cache lines (default: LGCACHELEN-3)
- `BUS_WIDTH`: Number of data bits on the bus (default: 32)
- `OPT_LITTLE_ENDIAN`: Endianness option (default: 0 for big-endian)

## Ports

### Inputs

- `i_clk`: Clock input
- `i_reset`: Reset signal
- `i_new_pc`: New program counter signal
- `i_clear_cache`: Cache clear signal
- `i_ready`: CPU ready signal
- `i_pc`: Program counter input [AW+WBLSB-1:0]
- `i_wb_stall`: Wishbone stall signal
- `i_wb_ack`: Wishbone acknowledge signal
- `i_wb_err`: Wishbone error signal
- `i_wb_data`: Wishbone data input [BUSW-1:0]

### Outputs

- `o_valid`: Output valid signal
- `o_illegal`: Illegal instruction signal
- `o_insn`: Instruction output [INSN_WIDTH-1:0]
- `o_pc`: Program counter output [AW+WBLSB-1:0]
- `o_wb_cyc`: Wishbone cycle signal
- `o_wb_stb`: Wishbone strobe signal
- `o_wb_we`: Wishbone write enable (always 0 for this module)
- `o_wb_addr`: Wishbone address output [AW-1:0]
- `o_wb_data`: Wishbone data output (unused in this module)

## Implementation Details

1. The cache is organized into lines, with each line containing multiple words.
2. Address words are separated into three components: Tag bits, Cache line number, and Cache position within the line.
3. The module implements a state machine to handle cache misses and bus interactions.
4. It uses a valid mask to keep track of which cache lines are valid.
5. Bus errors are handled by marking entire cache lines as having illegal values.

## Pinout Diagram

```
            +-------------+
i_clk    -->|             |
i_reset  -->|             |
i_new_pc -->|             |
i_clear_cache -->|        |
i_ready  -->|             |---> o_valid
i_pc     -->|             |---> o_illegal
i_wb_stall -->|           |---> o_insn
i_wb_ack  -->|  pfcache   |---> o_pc
i_wb_err  -->|            |---> o_wb_cyc
i_wb_data -->|            |---> o_wb_stb
            |             |---> o_wb_we
            |             |---> o_wb_addr
            |             |---> o_wb_data
            +-------------+
```

Note: The width of multi-bit ports is not shown in this diagram for simplicity.