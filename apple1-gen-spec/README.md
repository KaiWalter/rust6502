# Apple1 - generated emulator

This folder `./apple1-gen-spec` contains the specifications an Apple1 emulator to be generated into folder `./apple1-gen` with Copilot basend on requirements document `./requirements.md`.

## prompt steps

```
/clear
```

```
@copilot adhering to all technical requirements, create a basic project structure in a new folder `apple1-gen` for Rust WASM in the current workspace.
Do not try to create a new workspace.
Add this project as member to the root workspace manifest.
For each subsequent generation all technical requirements stated in requirements.md shall be observed.
At each step check, add or fix required Rust dependencies as well as features.
```

```
Implement the basic emulator with SPA and main loop considering all technical requirements and but only up to functional requirement FUNC001.
Build and fix unresolved dependencies and references until a successful build. After each failed build ask whether to continue or stop the process.
```

----

### TECH999

> suggestions I requested from Copilot based on compiler errors that happend on first generations

The main loop should use a self-referential closure pattern compatible with Rust/WASM ensuring ownership rules are respected.

When implementing animation loops or callbacks with Rc/RefCell in Rust WASM, always capture only a weak reference (Rc::downgrade) or clone Rc inside the closure. Never move the Rc or RefCell itself into the closure, to prevent ownership and borrow errors.

Any value moved into a closure (especially for animation loops) must not be used outside the closure after the move.

Require that all necessary web-sys features (e.g., Window, HtmlCanvasElement) are explicitly enabled in Cargo.toml.
