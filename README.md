# CacheSim: A Professional Cache Simulation Tool

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Technical Specifications](#technical-specifications)
  - [Architecture](#architecture)
  - [Cache Configuration](#cache-configuration)
  - [Fixed Parameters](#fixed-parameters)
- [Installation](#installation)
- [Usage](#usage)
  - [Command-Line Interface](#command-line-interface)
  - [Trace File Format](#trace-file-format)
- [Output](#output)
  - [Verbose Mode](#verbose-mode)
  - [Summary Statistics](#summary-statistics)
- [Error Handling](#error-handling)
- [Development Guidelines](#development-guidelines)
  - [Software Requirements](#software-requirements)
  - [Extending CacheSim](#extending-cachesim)
  - [Testing](#testing)
- [Example](#example)

## Overview

CacheSim is a high-performance cache simulator implemented in Rust, designed to model and analyze cache behavior under various configurations. It enables users to simulate memory access patterns from trace files, offering configurable parameters such as cache size, associativity, and number of sets. This tool is essential for hardware engineers, system architects, and performance optimization specialists seeking to evaluate how cache configurations impact memory access efficiency and system performance.

## Key Features

- **Configurable Parameters**: Customize cache size, associativity, and number of sets to suit specific simulation needs.
- **LRU Replacement Policy**: Employs the Least Recently Used eviction strategy for realistic cache behavior.
- **Comprehensive Analytics**: Delivers detailed metrics including miss rates, cache hits, and access times.
- **Trace File Support**: Analyzes memory access traces from real application executions.
- **Verbose Mode**: Provides operation-by-operation insights for in-depth analysis.
- **Accurate Operation Modeling**: Simulates both read and write memory accesses with precision.

## Technical Specifications

### Architecture

CacheSim leverages Rust's performance and memory safety features, featuring a modular design with:
- **Parsing Module**: Efficiently processes trace files.
- **Cache Operations Module**: Handles cache logic and eviction policies.
- **Metrics Collection Module**: Gathers and computes simulation statistics.

### Cache Configuration

- **Cache Size (C)**: Total size in bytes, must be a power of 2.
- **Associativity (E)**: Number of lines per set, must be a power of 2 (supports direct-mapped to fully associative).
- **Number of Sets (S)**: Total sets, must be a power of 2.
- **Block Size**: Derived from the above parameters for consistency.

### Fixed Parameters

- **Eviction Policy**: Least Recently Used (LRU).
- **Memory Access Penalty**: 100 cycles for time calculations.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone <repository_url>
   ```
2. **Build the Project**:
   ```bash
   cargo build --release
   ```
3. **Verify Installation**:
   Ensure the executable `cachesim` is available in the `target/release` directory.

## Usage

### Command-Line Interface

Run CacheSim using the following syntax:

```bash
./cachesim <trace_file> <cache_size> <associativity> <sets> [-v <start_op> <end_op>]
```

#### Parameters

- `<trace_file>`: Path to the trace file containing memory access operations.
- `<cache_size>`: Cache size in bytes (e.g., 2048 for 2KB).
- `<associativity>`: Lines per set (e.g., 4 for 4-way associativity).
- `<sets>`: Number of cache sets (e.g., 8).
- `[-v <start_op> <end_op>]`: Optional flag to enable verbose mode for operations from `<start_op>` to `<end_op>` (inclusive).

#### Example

```bash
./cachesim blowfish.xex 2048 4 8
```

This simulates a 2KB cache with 4-way associativity and 8 sets using the `blowfish.xex` trace file.

### Trace File Format

Trace files define memory access operations with the following format per line:

```
<instruction_pointer> <operation_type> <memory_address> <bytes> <data>
```

- `<instruction_pointer>`: Hexadecimal address of the executing instruction (e.g., `0xb7fc7489`).
- `<operation_type>`: `R` for read, `W` for write.
- `<memory_address>`: Hexadecimal target memory address (e.g., `0xbff20468`).
- `<bytes>`: Number of bytes accessed (e.g., 4).
- `<data>`: Hexadecimal data read or written (e.g., `0xb7fc748e`).

**Note**: Memory addresses are 32-bit, and trace files are derived from the [MiBench](https://ieeexplore.ieee.org/document/990739) benchmark suite.

## Output

### Verbose Mode

When `-v <start_op> <end_op>` is specified, CacheSim outputs detailed information for each operation in the range, including:
- **Operation Index**: Line number in the trace file (starting at 0).
- **Case Type**: `1` (hit), `2a` (clean miss), or `2b` (dirty miss).
- **Cache Index**: Hexadecimal set index (range `[0, S)`).
- **Cache Tag**: Hexadecimal tag for the address.
- **Cache Line**: Decimal line number within the set (range `[0, E)`).
- **Line Tag**: Previous tag in the line (`-1` if invalid).
- **Valid Bit**: `1` (valid) or `0` (invalid).
- **Dirty Bit**: `1` (dirty) or `0` (clean).
- **Last Used**: Operation index of the last use (for E > 1).

### Summary Statistics

At simulation end, CacheSim prints:

```
loads: <number>
stores: <number>
total accesses: <number>
read misses: <number>
write misses: <number>
total misses: <number>
dirty read misses: <number>
dirty write misses: <number>
bytes read: <number>
bytes written: <number>
read access time: <cycles>
write access time: <cycles>
miss rate: <percentage>
```

## Error Handling

CacheSim exits with a non-zero status and an error message to `stderr` if:
- Argument count is not 4 or 7.
- Trace file does not exist.
- `<cache_size>`, `<associativity>`, or `<sets>` is not a power of 2.
- Cache parameters are invalid (e.g., incompatible combinations).
- Verbose mode parameters `<start_op>` and `<end_op>` are not integers or `<start_op> > <end_op>`.

## Development Guidelines

### Software Requirements

- **Language**: Rust (latest stable version recommended).
- **Build Tool**: Cargo (included with Rust).

### Extending CacheSim

The modular structure supports:
- Adding new eviction policies.
- Enhancing metrics with custom statistics.
- Improving trace file parsing for additional formats.

### Testing

- **Unit Tests**: Use `pytest` for validation:
  ```bash
  pip install pytest pytest-testdox
  ./runTester.sh
  ```
- **Memory Safety**: Check for leaks with Valgrind:
  ```bash
  ./runValgrind.sh
  ```

## Example

Simulate with verbose output for operations 0 to 2:

```bash
./cachesim tracefile.xex 1024 2 4 -v 0 2
```

**Sample Trace File** (`tracefile.xex`):
```
0xb7fc7489 W 0xbff20468 4 0xb7fc748e
0xb7fc748e R 0xbff20468 4 0xb7fc748e
0xb7fc7495 W 0xbff20478 4 0xbff204b0
```

**Output**:
```
0 2b 0x10 0x1ff40 0 -1 0 0 -
1 1 0x10 0x1ff40 0 0x1ff40 1 0 0
2 2a 0x11 0x1ff40 1 -1 0 0 -
loads: 1
stores: 2
total accesses: 3
read misses: 0
write misses: 2
total misses: 2
dirty read misses: 0
dirty write misses: 1
bytes read: 0
bytes written: 8
read access time: 1
write access time: 202
miss rate: 0.6667
```
