# ra-data

This repository contains the data and tools required to generate Peripheral Access Crates (PACs) for Renesas RA microcontrollers. It follows a data-driven approach where chip definitions and register maps are stored in intermediary YAML files.

## Project Structure

- `ra-data-types/`: Rust crate defining the shared data models for chips, peripherals, and memory maps.
- `ra-data-gen/`: The tool responsible for extracting data from SVD and Rzone files into the intermediary YAML format.
- `ra-metapac-gen/`: The generator tool that transforms YAML data into the `ra-metapac` Rust crate.

- `data/`: The intermediary data store.
  - `data/chips/`: YAML files for each specific Part Number (PN), containing memory maps and peripheral instances.
  - `data/registers/`: YAML files defining the register blocks, fieldsets, and enums for each peripheral type.
- `d`: A helper script to run common tasks like data generation and PAC generation.

## Data Model (YAML)

The project relies on a two-tier YAML data model to ensure accuracy and deduplication.

### Chip Definitions (`data/chips/`)
Each chip YAML (e.g., `r7fa0e1073cfj.yaml`) is based on the `Chip` struct in `ra-data-types`. It includes:
- **Memory Map**: Precise boundaries for Flash, RAM, and Data Flash extracted from Renesas `.rzone` files.
- **Peripherals**: A list of peripheral instances (e.g., `SRAM`, `UART`) with their base addresses and a reference to their register block definition.
- **Interrupts**: The NVIC interrupt vector table for that specific device.

### Register Definitions (`data/registers/`)
These files define the internal structure of a peripheral. They are generated using `chiptool` and include:
- **Blocks**: The memory layout of the peripheral (registers and their offsets).
- **Fieldsets**: Bit-level definitions for each register.
- **Enums**: Named values for register fields.

## Components

### `ra-data-types`
A core library that ensures consistency between the data extraction tools and the PAC generator. It defines the `Chip`, `Peripheral`, `Memory`, and `Interrupt` structures used for serialization.

### `ra-metapac-gen`
The primary generation engine. It performs the following steps:
1.  **Data Loading**: Parses all chip and register YAMLs into memory.
2.  **Deduplication**: Identifies unique register blocks used across the entire RA family.
3.  **PAC Generation**: Creates a structured Rust crate (`ra-metapac`) with:
    - Shared peripheral modules in `src/peripherals/`.
    - Per-chip modules in `src/chips/`.
    - A `build.rs` script for dynamic chip selection.
4.  **Metadata Generation**: Produces a static metadata object for each chip, allowing HALs to be chip-agnostic.

## Workflow

### Generating the PAC
To regenerate the `ra-metapac` crate from the current YAML data:
```bash
./d gen-pac
```
The output will be generated in the `out/` directory.

### Adding New Data
1.  Place new SVD or Rzone files in the `sources/` directory.
2.  Use the extraction scripts (in `scripts/` or via `./d`) to update the YAML files in `data/`.
3.  Run `./d gen-pac` to update the generated PAC.
