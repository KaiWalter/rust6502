# Experimental 6502 emulator in Rust

> With this project I try to **learn Rust**. Hence it is not intended as a fully working emulator or a good example - it is rather a reflection of my progress.

> REMINDER to self : idiomatic Rust uses `snake_case` for variables, methods, macros, fields and modules; `UpperCamelCase` for types and enum variants; and `SCREAMING_SNAKE_CASE` for statics and constants

## general design ideas

- everything is a component (RAM, ROM, PIA) which is linked by interface (`address_bus::Addressing trait`) to the emulator, so it can be expanded for various use cases (as with my previous implementations Apple1, Commodore PET, ...) - also no loading of a ROM just into a big 64kB memory space
- only the component (e.g. `memory::Memory`) is aware of it's own address offset - `address_bus::AddressBus` expects to read from / write to the absolute address

## resources

 - [Writing an OS in Rust by Philipp Oppermann](https://os.phil-opp.com/)
 - [other sample ROMs](https://github.com/alexander-akhmetov/apple1/tree/master/roms)