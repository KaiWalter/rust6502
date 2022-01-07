# Experimental 6502 emulator in Rust

> With this project I try to **learn Rust**. Hence it is not intended as a fully working emulator or a good example - it is rather a reflection of my progress.

## general design ideas

- **everything is a component** (RAM, ROM, PIA) which is linked by interface (`address_bus::Addressing trait`) to the emulator, so it can be expanded for various use cases (as with my previous implementations Apple1, Commodore PET, ...)
- no loading of a ROM just into a big 64kB memory space - **ROMs have separate spaces** addressable by `address_bus::AddressBus`
- **variable RAM/memory size** - not fixed to e.g. 64kB
- only the component itself (e.g. `memory::Memory`) is aware of it's own address offset - `address_bus::AddressBus` expects to read from / write to the absolute address

## learnings

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
pub fn read(&self, addr: u16) -> Result<u8, AddressingError> {
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
    pub fn read(&self, addr: u16) -> Result<u8, AddressingError> {
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


## resources

- [6502 instruction set](https://masswerk.at/6502/6502_instruction_set.html)
- [Writing an OS in Rust by Philipp Oppermann](https://os.phil-opp.com/)
- [1st other 6502 implementation to peek](https://github.com/alexander-akhmetov/mos6502) -> [applied for Apple 1](https://github.com/alexander-akhmetov/apple1)
- [2nd other 6502 implementation to peek](https://github.com/GarettCooper/emulator_6502) -> [applied for NES](https://github.com/GarettCooper/gc_nes_emulator)
- [other sample ROMs](https://github.com/alexander-akhmetov/apple1/tree/master/roms)
- [Disassembler](https://masswerk.at/6502/disassembler.html)

### helpers

```shell
diff -y --suppress-common-lines ./func-go.txt ./func-rust.txt | less
awk 'NR>26764000' func-rust.txt > func-rust-tail.txt
diff -y ./func-go-tail.txt ./func-rust-tail.txt | less
```

> REMINDER to self : idiomatic Rust uses `snake_case` for variables, methods, macros, fields and modules; `UpperCamelCase` for types and enum variants; and `SCREAMING_SNAKE_CASE` for statics and constants


## perf tests

<https://github.com/flamegraph-rs/flamegraph>
<https://stackoverflow.com/a/60276918/4947644>

### for WSL2

```shell
sudo apt install build-essential flex bison libssl-dev libelf-dev
sudo apt-get install libunwind-dev libw-dev 
cd ~/src
git clone --depth=1 https://github.com/microsoft/WSL2-Linux-Kernel.git
cd WSL2-Linux-Kernel/tools/perf
sudo make DESTDIR=/usr/lib/linux-tools/linux-tools-`uname -r` install
```


```shell
export PERF=/usr/lib/linux-tools/linux-tools-`uname -r`/perf
cargo flamegraph --dev --unit-test -- mos6502::tests::functional_test
/usr/lib/linux-tools/linux-tools-`uname -r`/perf report -i perf.data
```

problem: 

```
test mos6502::tests::functional_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 100.64s

[ perf record: Woken up 3224 times to write data ]
[ perf record: Captured and wrote 806.051 MB perf.data (100139 samples) ]
writing flamegraph to "flamegraph.svg"
Error: unable to generate a flamegraph from the collapsed stack data

Caused by:
    0: I/O error: No stack counts found
    1: No stack counts found
```
