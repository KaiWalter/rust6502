mod address_bus;
mod memory;
mod mos6502;
use address_bus::*;
use memory::Memory;

fn main() {
    mos6502::cycle();
    let mem = Memory::new(0, 0x200);
    let mem_addr: Box<dyn Addressing> = Box::new(mem);
    let mut address_bus = AddressBus::new(0x100);
    address_bus.add_component(0, 0x200, mem_addr);
    address_bus
        .write(0x110, 10)
        .expect("accessing wrong address");
    println!("{:x}", address_bus.read(0x110).unwrap());
}
