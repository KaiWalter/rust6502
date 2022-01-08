mod address_bus;
mod memory;
mod mos6502;

use address_bus::*;
use memory::*;
use mos6502::*;

fn main() {
    const END_OF_FUNCTIONAL_TEST: u16 = 0x3469;
    let mut mem = Memory::load_rom(0, "./roms/6502_functional_test.bin".to_string());
    let mut address_bus = SimpleAddressBus::new(&mut (mem));
    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.run(0x0400, END_OF_FUNCTIONAL_TEST);
}
