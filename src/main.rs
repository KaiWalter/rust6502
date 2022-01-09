use rust6502::memory::*;
use rust6502::mos6502::*;

fn main() {
    const END_OF_FUNCTIONAL_TEST: u16 = 0x3469;
    let mut mem = Memory::load_rom(0, "./roms/6502_functional_test.bin".to_string());
    let mut cpu = Cpu::new(CpuRegisters::default(), &mut mem);
    cpu.run(0x0400, END_OF_FUNCTIONAL_TEST);
}
