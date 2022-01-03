mod address_bus;
mod memory;
mod mos6502;
use address_bus::*;
use memory::Memory;

fn main() {
    mos6502::cycle();

    // Apple 1 configuration
    let mut mem = Memory::new(0, 0x1000); // 4kB memory
    let mut address_bus = AddressBus::new(0x1000); // potential separate component/ROM for each 4kB
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }
    address_bus
        .write(0x110, 10)
        .expect("accessing wrong address");
    println!("{:x}", address_bus.read(0x110).unwrap());
}
