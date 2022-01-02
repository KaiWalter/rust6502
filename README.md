# Experimental 6502 emulator in Rust

> With this project I try to **learn Rust**. Hence it is not intended as a fully working emulator or a good example - it is rather a reflection of my progress.

> REMINDER to self : idiomatic Rust uses `snake_case` for variables, methods, macros, fields and modules; `UpperCamelCase` for types and enum variants; and `SCREAMING_SNAKE_CASE` for statics and constants

## general design ideas

- everything is a component (CPU, memory) which is linked by interface to the the emulator can be expanded for various use cases (as with my previous implementations Apple1, Commodore PET, ...)

## resources

 - [Writing an OS in Rust by Philipp Oppermann](https://os.phil-opp.com/)