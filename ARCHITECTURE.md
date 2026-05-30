# ARCHITECTURE

This is a snapshot of the current design to support future refactoring.

## Workspace layout

Rust workspace members:

- `core` (`rust6502` library): emulator primitives (CPU, memory, bus, PIA) + functional test binary
- `apple1`: native terminal Apple 1 executable (`ncurses`)
- `apple1-wasm`: browser/Wasm Apple 1 executable (`wasm-bindgen`, `web-sys`)

## High-level design

The project is component-based.

- `Cpu` executes 6502 instructions.
- `AddressBus` maps absolute 16-bit addresses to components.
- Components implement `InternalAddressing` and expose read/write behavior:
  - `Memory` (RAM/ROM)
  - `MC6821` (PIA)
- Host-specific frontends (`apple1`, `apple1-wasm`) wire devices together and provide I/O loops.

## Layering (inside-out)

From innermost core logic to outermost runtime integration:

1. **CPU execution core** (`core::mos6502`)
   - Registers, opcode table, addressing modes, instruction execution, cycle accounting.
2. **Addressing contracts** (`core::address_bus` traits)
   - `InternalAddressing` and `ExternalAddressing` define how components communicate.
3. **Memory-mapped components** (`core::memory`, `core::mc6821`)
   - Concrete devices implementing `InternalAddressing`.
4. **Interconnect / composition layer** (`core::address_bus::AddressBus`)
   - Routes absolute addresses to mapped components.
5. **Machine assembly layer** (`apple1`, `apple1-wasm` composition code)
   - Instantiates RAM/ROM/PIA, maps ranges, creates CPU, wires channels.
6. **Runtime/host integration layer**
   - Native: threads + `ncurses` loop.
   - Wasm: browser event handling + `requestAnimationFrame` loop.
7. **User-facing I/O surface**
   - Keyboard input and terminal/canvas output exposed by host-specific frontends.

Dependency direction should remain outside-in at construction time, but inside-out at runtime flow:

- Construction: outer layers build and inject inner dependencies.
- Runtime: CPU (inner) drives bus/device interactions, while outer loop supplies events and renders output.

## Core crate (`core/src`)

### 1) Address bus (`address_bus`)

Main types:

- `InternalAddressing` trait: `int_read`, `int_write`, `len`
- `ExternalAddressing` trait: `read`, `write` returning `Result<_, AddressingError>`
- `AddressBus`:
  - `block_component_map: Vec<usize>` maps address blocks to component index
  - `component_addr: Vec<&mut dyn InternalAddressing>` stores component references

Behavior:

- Fixed 64KB address space assumption via `0x10000 / block_size` mapping table.
- `add_component(from_addr, size, component)` requires block-aligned sizes.
- Runtime accesses are two-stage: block lookup -> component dispatch.

### 2) Memory (`memory`)

- `Memory { offset, mem: Vec<u8> }`
- Supports RAM (`new`), ROM from vec (`from_vec`), ROM from file (`load_rom`)
- Internal and external addressing implemented via offset-relative indexing.

### 3) PIA (`mc6821`)

- Emulates Motorola MC6821-like behavior (ports A/B, control/data direction registers, control lines, IRQ flags)
- Uses `crossbeam_channel` for async host/device communication:
  - input channel (`InputSignal`) into PIA
  - output channels for port A/B bytes
  - optional interrupt channel (`InterruptSignal`)
- `int_read`/`int_write` expose register behavior mapped by low address bits (`addr & 0x03`).

### 4) CPU (`mos6502`)

- `Cpu` owns register state and mutable reference to `dyn ExternalAddressing`.
- Opcode dispatch table (`OPCODES[256]`) binds opcode -> operation fn + addressing mode fn + base cycles.
- Cycle model:
  - `cycle()` fetch/decode/execute when `remaining_cycles == 0`
  - decrements one cycle each call
- Reset vector read from `0xFFFC/0xFFFD`.

## Native Apple 1 (`apple1/src/main.rs`)

Composition:

- `AddressBus` with block size `0x100`
- RAM: `Memory` at `0x0000` (4KB)
- PIA: `MC6821` at `0xD000` (0x100 mapped range)
- ROM monitor: `0xFF00`
- BASIC ROM: `0xE000`
- CPU connected to bus

I/O model:

- `ncurses` terminal for output
- background thread for keyboard input
- channels bridge terminal <-> PIA
- main loop:
  1. poll terminal input and emit PIA input signals
  2. execute one CPU cycle
  3. sleep (`100µs`)

## Wasm Apple 1 (`apple1-wasm/src/lib.rs`)

Design differs from native build due to browser runtime constraints.

Composition:

- `Apple1CompactBus` (custom compact bus wrapper) with:
  - 4KB RAM
  - monitor ROM embedded as `Vec<u8>`
  - PIA
- thread-local global state (`COMPACT_APPLE1`, `TX_APPLE_INPUT`) via `thread_local!`

Runtime model:

- `#[wasm_bindgen(start)]` initializes CPU, bus, channels, terminal, closures.
- main execution uses `requestAnimationFrame` loop.
- each frame:
  - process keyboard/terminal events
  - feed PIA input signals
  - run CPU cycles (bounded inner loop)

## Data/control flow summary

1. CPU reads/writes absolute addresses via `ExternalAddressing`.
2. Address bus routes to component by block mapping.
3. PIA and memory perform device-local address translation.
4. Frontend-specific event loop injects input and renders output.

## Current architectural constraints and refactor candidates

- `AddressBus` stores mutable trait-object references with lifetimes that can make composition awkward.
- 64KB + block mapping assumptions are hard-coded.
- Error handling still panics in core execution paths (CPU fetch/decode paths).
- Native and Wasm Apple 1 wiring are structurally similar but duplicated.
- Wasm uses global/thread-local mutable state and unsafe-style indirection patterns (through shared mutable setup), indicating a candidate for cleaner ownership boundaries.
- Large opcode table and operation definitions are monolithic in `mos6502`.

## Useful entry points

- `core/src/lib.rs`
- `core/src/address_bus/mod.rs`
- `core/src/memory/mod.rs`
- `core/src/mc6821/mod.rs`
- `core/src/mos6502/mod.rs`
- `apple1/src/main.rs`
- `apple1-wasm/src/lib.rs`
