# CFG-Based Metamorphic Engine

This project implements a **Control-Flow Graph (CFG)-based Metamorphic Engine** in Rust, designed for generating and evaluating metamorphic variants of assembly code. It is intended for research and educational purposes, particularly in the context of malware analysis and software diversity.

## Features

- **Rust-based CFG Metamorphic Engine**: Generates multiple functionally equivalent assembly variants using control-flow and instruction-level transformations.
- **Automated Build System**: Batch script (`build_cfg.bat`) for assembling and linking all variants using Visual Studio tools.
- **Evaluation Suite**: Python script (`evaluate.py`) to measure similarity between original and variant binaries using opcode n-grams, fuzzy hashing (ssdeep), and CFG metrics.
- **Docker Support**: Dockerfile and docker-compose for reproducible evaluation environments.

## Project Structure

- `src/main.rs` — Rust source code for the CFG-based metamorphic engine.
- `output/` — Generated assembly files, opcodes, and executables.
- `Dataset1/` — Example dataset with original and variant assembly/opcode files, plus evaluation results.
- `evaluate.py` — Python script for evaluating similarity metrics.
- `build_cfg.bat` — Batch script to build all variants (requires Visual Studio and ml64).
- `Dockerfile`, `docker-compose.yml` — For running the evaluation in a containerized environment.

## Usage

### 1. Build Metamorphic Variants

1. Ensure you have Rust, Visual Studio (with ml64), and Python 3.11+ installed.
2. Run the Rust engine to generate assembly variants:
   ```
   cargo run --release
   ```
   This creates `cfg_original.asm` and multiple `cfg_variant_*.asm` files in `output/`.

3. Build all variants:
   ```
   build_cfg.bat
   ```
   This assembles and links all `.asm` files, producing `.exe` binaries.

### 2. Evaluate Variants

- To evaluate similarity metrics, run:
  ```
  python evaluate.py
  ```
  Or use Docker:
  ```
  docker compose up
  ```

### 3. Results

- Evaluation results are saved in `Dataset1/` as `.json` and `.txt` reports.

## Requirements

- Rust (2021+)
- Visual Studio 2022 (for `ml64` and `link`)
- Python 3.11+ (with `networkx`, `matplotlib`, `numpy`, `ssdeep`)
- Docker (optional, for containerized evaluation)

## References

- Bachelor's Thesis: *Design and Evaluation of a Rust Based Metamorphic Malware Transformation Engine*
- See `Dataset1/cfg_evaluation_report.txt` for sample evaluation output.
