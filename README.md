# Experimental 6502 emulator in Rust

With this project I try to **learn Rust**. Hence it is not intended as a fully working emulator or a good example - it is rather a reflection of my progress.

## TL;DR - run stuff

> generally check out `.devcontainer/setup.sh` for system or Rust/Cargo dependencies used

### functional test

from `/`

```shell
cargo test --release
```

### Apple 1 with Linux terminal

> packages `libncurses5-dev libncursesw5-dev` required

from `/`

```shell
cargo run --bin apple1 --release
```

### Apple 1 with Wasm

> `(cargo) wasm-pack` and Python 3 to run `http.server` required

from `/apple1-wasm`

```shell
./run.sh
```

## general design ideas

- **everything is a component** (RAM, ROM, PIA) which is linked by interface (`address_bus::ExternalAddressing trait`) to the emulator, so it can be expanded for various use cases (as with my previous implementations Apple1, Commodore PET, ...)
- no loading of a ROM just into a big 64kB memory space - **ROMs have separate spaces** addressable by `address_bus::AddressBus`
- **variable RAM/memory size** - not fixed to e.g. 64kB
- only the component itself (e.g. `memory::Memory`) is aware of it's own address offset - `address_bus::AddressBus` expects to read from / write to the absolute address

### for Apple 1 Wasm

- have the lowest possible footprint of JavaScript, directly handle DOM and events from Rust

----

## learnings

I migrated the code base from my previous [Go 6502 implementation](https://github.com/KaiWalter/go6502). Here is what I stumbled over in the beginning of the tranistion:

### unsigned integer overflows

**Rust** does not accept overflows on its unsigned int.

```Rust
pub fn cmp(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values) as u16;
    let temp = cpu.r.a as u16 - fetched;
    cpu.set_flag(StatusFlag::C, cpu.r.a as u16 >= fetched);
    cpu.set_flag(StatusFlag::Z, temp & 0x00FF == 0);
    cpu.set_flag(StatusFlag::N, temp & 0x0080 != 0);
    1
}
```

this is caused by [`6502_functional_test.bin`](https://github.com/Klaus2m5/6502_65C02_functional_tests/blob/7954e2dbb49c469ea286070bf46cdd71aeb29e4b/bin_files/6502_functional_test.lst#L1211) in this section with a `attempt to subtract with overflow`:

```asm
0596 : c900                     cmp #0          ;test compare immediate 
                                trap_ne
```

In **Go** it seems a negativ subtraction on uints automatically rolls over - hence no problem.

```Golang
func CMP() int {
    fetch()
    temp := uint16(A) - uint16(fetched)
    SetFlag(C, A >= fetched)
    SetFlag(Z, (temp&0x00FF) == 0x0000)
    SetFlag(N, temp&0x0080 != 0)
    return 1
}
```

In **Rust** it needed this modification:

```Rust
pub fn cmp(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values) as i16;
    let temp = (cpu.r.a as i16 - fetched) as u8;
    cpu.set_flag(StatusFlag::C, register >= fetched as u8);
    cpu.set_flag(StatusFlag::Z, temp & 0x00FF == 0);
    cpu.set_flag(StatusFlag::N, temp & 0x0080 != 0);
    1
}
```

Checking other implementations, I found the wrapping intrinsics, which handle type overflows:

```Rust
pub fn cmp(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values);
    let temp = cpu.r.a.wrapping_sub(fetched);
    cpu.set_flag(StatusFlag::C, cpu.r.a >= fetched);
    cpu.set_flag(StatusFlag::Z, temp & 0x00FF == 0);
    cpu.set_flag(StatusFlag::N, temp & 0x0080 != 0);
    1
}
```

### operator precedence

In [**Go**](https://kuree.gitbooks.io/the-go-programming-language-report/content/31/text.html) *bitwise and* `&` has precedence over *arithmetic plus* `+` - compared to [**Rust**](https://doc.rust-lang.org/reference/expressions.html#expression-precedence).

So when converting


```Golang
		if temp < 0x0f {
			temp = temp&0x0f + (uint16(A) & 0xf0) + (uint16(fetched) & 0xf0)
		} else {
			temp = temp&0x0f + (uint16(A) & 0xf0) + (uint16(fetched) & 0xf0) + 0x10
		}
```
1:1 into **Rust**, does not yield the same results.

```Rust
        if temp_bcd < 0x0F {
            temp_bcd = temp_bcd & 0x0F + (cpu.r.a as u16 & 0xF0) + (fetched & 0xF0);
        } else {
            temp_bcd = temp_bcd & 0x0F + (cpu.r.a as u16 & 0xF0) + (fetched & 0xF0) + 0x10;
        }
```

Hence expressions with mixed `&` and `+` need to be put in parenthesis explicitly.

```Rust
        if temp_bcd < 0x0F {
            temp_bcd = (temp_bcd & 0x0F) + (cpu.r.a as u16 & 0xF0) + (fetched & 0xF0);
        } else {
            temp_bcd = (temp_bcd & 0x0F) + (cpu.r.a as u16 & 0xF0) + (fetched & 0xF0) + 0x10;
        }
```

## performance of HashMap

I wanted to keep the amount of components (ROMs, RAM, PIAs) addressable by the `AddressBus` flexible and designed 2 `HashMap` structure elements which map from a **target address** to a **component** which handles the final read from and write to memory/component. 

```Rust
pub struct AddressBus<'a> {
    block_size: usize,
    block_component_map: HashMap<u16, u16>, // map a 1..n blocks to 1 components
    component_addr: HashMap<u16, &'a mut (dyn Addressing)>, // 1:1 map component to its addressing
}
...
pub fn read(&mut self, addr: u16) -> Result<u8, AddressingError> {
    let block = (addr as usize / self.block_size) as u16;
    if self.block_component_map.contains_key(&block) {
        let component_key = self.block_component_map[&block];
        Ok(self.component_addr[&component_key].read(addr))
    } else {
        Err(AddressingError::new("read", addr))
    }
}

pub fn write(&mut self, addr: u16, data: u8) -> Result<(), AddressingError> {
    let block = (addr as usize / self.block_size) as u16;
    if self.block_component_map.contains_key(&block) {
        let component_key = self.block_component_map[&block];
        if let Some(x) = self.component_addr.get_mut(&component_key) {
            x.write(addr, data);
        };
        Ok(())
    } else {
        Err(AddressingError::new("write", addr))
    }
}
```

Comparing results of `functional_test` to the **Go** implementation revealed a massive performance deviation (**Go** a few seconds, **Rust** > 1 minute). Using `perf` I identified, that these `HashMap` operations consume most of the processing time.

Converting this to an array based approach, brought the performance for the **Rust** implementation to the [**Go** map based implementation](https://github.com/KaiWalter/go6502/blob/main/pkg/addressbus/multi.go).


```Rust
pub struct AddressBus<'a> {
    block_size: usize,
    block_component_map: Vec<usize>, // map a 1..n blocks to 1 components
    component_addr: Vec<&'a mut dyn Addressing>, // 1:1 map component to its addressing
}
...
    pub fn read(&mut self, addr: u16) -> Result<u8, AddressingError> {
        let block = addr as usize / self.block_size;
        if self.block_component_map[block] == usize::MAX {
            Err(AddressingError::new("read", addr))
        } else {
            let component_key = self.block_component_map[block];
            match self.component_addr.get(component_key) {
                Some(component) => Ok(component.read(addr)),
                None => Err(AddressingError::new("read", addr)),
            }
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) -> Result<(), AddressingError> {
        let block = addr as usize / self.block_size;
        if self.block_component_map[block] == usize::MAX {
            Err(AddressingError::new("write", addr))
        } else {
            let component_key = self.block_component_map[block];
            match self.component_addr.get_mut(component_key) {
                Some(component) => {
                    component.write(addr, data);
                    Ok(())
                }
                None => Err(AddressingError::new("write", addr)),
            }
        }
    }
```

## Wasm - a totally different beast

Migration of the pure Linux console version of the Apple 1 - after getting used to some of the **Rust** pecularities - turned out quite straight forward.

The **Go** version is implemented with SDL2 terminal rendering. For the **Rust** version I wanted to stay in the GitHub Codespace, which cannot run GUI applications. Hence I tried to get terminal rendering (with the original character set) working with Wasm.

To really get the native **Rust Wasm** "feeling", I did not want a JavaScript heavy implementation like [Rust Wasm Chip-8 emulator](https://github.com/ColinEberhardt/wasm-rust-chip8) - also not extensively relying on **Node.js** and **Webpack**. Just a plain `index.html` and whatever minimum plumbing (**wasm-pack** in this case) is required. 

### yak shaving - iteration 0

For the Wasm implementation I needed to have a compact implementation of Apple 1:

- Hex monitor already "baked" into memory as loading ROMs from file system is not supported - loading it with JS `fetch` would be an alterative, but I did not want (yet) to spend the effort
- the multi-threaded approach could not easily be migrated from the [console version](apple1/src/main.rs) to Wasm; hence no usage of the flexible `address_bus`, but a fixed implementation just for Apple 1 Wasm use case
- as `thread::sleep` is not supported in Wasm (to give the browser some breathing space), I needed to bring it into a `request_animation_frame` flow; only cycling processor operations and checking inputs blocked the browser

**approach:** make a `request_animation_frame` flow implementation like

```
    let inner = Rc::new(RefCell::new(None));
    let outer = inner.clone();

    *outer.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // check input from the terminal and send to PIA
        ...

        // do minimum x processor cycles before checking input again
        ...

        request_animation_frame(inner.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(outer.borrow().as_ref().unwrap());
```

### yak shaving - iteration 1

Struggling with bringing the instance variables of Cpu, Memory, ... into the `Closure` of the `request_animation_frame` loop ...

**approach:** make a `static` compact Apple 1 implementation/instance

### yak shaving - iteration 2

Too lazy to deal with `lazy_static!` macro ...

```Rust
pub struct Apple1Compact<'a> {
    pub cpu: Option<Cpu<'a>>,
    pub bus: Option<Apple1CompactBus>,
    pub terminal: Option<WasmTerminal>,
    pub check_input: Option<Box<dyn Fn()>>,
}

static mut COMPACT_APPLE1: Apple1Compact = Apple1Compact {
    cpu: None,
    bus: None,
    terminal: None,
    check_input: None,
};
```

**approach:** initialize in startup code - not in declaration or constructors

### yak shaving - iteration 3

`std::sync::mpsc::channel` variables did not make the hop from startup code into the above mentioned `Closure`. Message: `mpsc::Sender cannot be shared between threads` ...

**approach:** change to `crossbeam-channel` to allow sharing of `Sender` / `Receiver` variables

### yak shaving - iteration 4

In setup code the channel from keyboard to PIA is created:

```Rust
        // channel from keyboard to PIA (keyboard=tx, PIA=rx)
        let (tx_apple_input, rx_apple_input): (Sender<InputSignal>, Receiver<InputSignal>) =
            unbounded();
```

to be added to the address bus.

However this cannot be used in the `COMPACT_APPLE1.check_input` closure:

```
114 | |                     tx_apple_input.send(InputSignal::CA1(Signal::Fall)).unwrap();
    | |                     ^^^^^^^^^^^^^^ borrowed value does not live long enough
```

**approach:** wrap it in a static `TX_APPLE_INPUT = Some(Mutex::new(tx_apple_input));` and unwrap it in the closure `let tx_apple_input = TX_APPLE_INPUT.as_ref().unwrap().lock().unwrap().clone();` until I better comprehend this lifetime issue

----

## resources

### emulation

- [6502 instruction set](https://masswerk.at/6502/6502_instruction_set.html)
- [Writing an OS in Rust by Philipp Oppermann](https://os.phil-opp.com/)
- [1st other 6502 implementation to peek](https://github.com/alexander-akhmetov/mos6502) -> [applied for Apple 1](https://github.com/alexander-akhmetov/apple1)
- [2nd other 6502 implementation to peek](https://github.com/GarettCooper/emulator_6502) -> [applied for NES](https://github.com/GarettCooper/gc_nes_emulator)
- [other sample ROMs](https://github.com/alexander-akhmetov/apple1/tree/master/roms)
- [Disassembler](https://masswerk.at/6502/disassembler.html)
- [Rust Wasm Chip-8 emulator](https://github.com/ColinEberhardt/wasm-rust-chip8)

### Rust

- [Visualizing memory layout of Rust's data types](https://youtu.be/rDoqT-a6UFg)

### unsorted / helpers

#### cut run logs for easier comparison

```shell
diff -y --suppress-common-lines ./func-go.txt ./func-rust.txt | less
awk 'NR>26764000' func-rust.txt > func-rust-tail.txt
diff -y ./func-go-tail.txt ./func-rust-tail.txt | less
```

> REMINDER to self : idiomatic Rust uses `snake_case` for variables, methods, macros, fields and modules; `UpperCamelCase` for types and enum variants; and `SCREAMING_SNAKE_CASE` for statics and constants

#### render ROM to u8 vec

```shell
hexdump -v -e '16/1 "0x%02x, " "\n"' roms/Apple1_charmap.bin > rom.txt
```
