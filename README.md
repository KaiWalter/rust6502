# Experimental 6502 emulator in Rust

> With this project I try to **learn Rust**. Hence it is not intended as a fully working emulator or a good example - it is rather a reflection of my progress.

> REMINDER to self : idiomatic Rust uses `snake_case` for variables, methods, macros, fields and modules; `UpperCamelCase` for types and enum variants; and `SCREAMING_SNAKE_CASE` for statics and constants

## general design ideas

- **everything is a component** (RAM, ROM, PIA) which is linked by interface (`address_bus::Addressing trait`) to the emulator, so it can be expanded for various use cases (as with my previous implementations Apple1, Commodore PET, ...)
- no loading of a ROM just into a big 64kB memory space - **ROMs have separate spaces** addressable by `address_bus::AddressBus`
- **variable RAM/memory size** - not fixed to e.g. 64kB 
- only the component itself (e.g. `memory::Memory`) is aware of it's own address offset - `address_bus::AddressBus` expects to read from / write to the absolute address

## resources

- [6502 instruction set](https://masswerk.at/6502/6502_instruction_set.html)
- [Writing an OS in Rust by Philipp Oppermann](https://os.phil-opp.com/)
- [1st other 6502 implementation to peek](https://github.com/alexander-akhmetov/mos6502) -> [applied for Apple 1](https://github.com/alexander-akhmetov/apple1)
- [2nd other 6502 implementation to peek](https://github.com/GarettCooper/emulator_6502) -> [applied for NES](https://github.com/GarettCooper/gc_nes_emulator)
- [other sample ROMs](https://github.com/alexander-akhmetov/apple1/tree/master/roms)