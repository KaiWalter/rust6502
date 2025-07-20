# Apple1 Emulator Requirements

## Technical Requirements

All technical requirements are coded with a `TECH` prefix. All folder specifications in this document are relative to this folder.

### TECH001 - Architecture Style

Emulator shall run in a single page web app with a single index.html using a canvas to display the emulator frame rendered by the Rust WebAssembly implementation.
The canvas shall directly invoke the emulator main loop.

### TECH002 - Naming

Package name shall be `apple1-gen`.

### TECH003 - Emulation Cycles

Emulator shall be a cycle correct implementation of an Apple1 with 60 FPS.

### TECH100 - Existing Dependencies

Emulator must depend on the 6502 processor implementation in folder `../core`. 
Name this dependency `rust6502` and refer to it with that name.
The implementation in this folder shall be member of the workspace already existing in `../Cargo.toml`.
Consider workspace dependencies in `../Cargo.toml` to be re-used in this project.

### TECH101 - Existing Documents

`requirements.md` and `README.md` shall not be modified.

### TECH200 - only WASM

The project shall only produce a WebAssembly (WASM) library for web integration. No binaries, console applications, or native executables shall be generated or included. All entry points must be compatible with WASM and callable from JavaScript in a browser context.

### TECH201 - Rust Ownership and WASM

All code must comply with Rust’s ownership and lifetime rules, especially for WASM and asynchronous/event-driven code.

## Functional Requirements

All functional requirements are coded with a `FUNC` prefix.

### FUNC001

Emulator character output shall be terminal style green on black.
Title shall be "Copilot generated Apple1 Emulator".

### FUNC002

Emulator shall provide user input over keyboard.

### FUNC003

Emulator shall display characters based on character map file `../roms/Apple1_charmap.bin`. Expect a 8x8 pixels character definition.

### FUNC004

Emulator shall load file `../roms/Apple1_HexMonitor.bin`. ROM shall be loaded at startup.
