#[cfg(test)]
use super::*;
use crate::memory::*;

#[test]
fn writes_and_reads_memory_block_zero() {
    // arrange
    let mut mem = Memory::new(0, 0x200);
    let mut address_bus = AddressBus::new(0x100);
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let addr = 10;
    let expected = 42u8;

    // act
    address_bus.write(addr, 42).expect("wrong address");
    let actual = address_bus.read(addr);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn writes_and_reads_memory_block_one() {
    // arrange
    let mut mem = Memory::new(0, 0x200);
    let mut address_bus = AddressBus::new(0x100);
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let addr = 0x110;
    let expected = 42u8;

    // act
    address_bus.write(addr, 42).expect("wrong address");
    let actual = address_bus.read(addr);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn writes_and_reads_memory_nonzero_offset() {
    // arrange
    let offset = 0xF000u16;
    let mut mem = Memory::new(offset, 0x200);
    let mut address_bus = AddressBus::new(0x100);
    if address_bus
        .add_component(offset, mem.len(), &mut (mem))
        .is_err()
    {
        panic!("add_component failed");
    }

    let addr = offset + 10;
    let expected = 42u8;

    // act
    address_bus.write(addr, 42).expect("wrong address");
    let actual = address_bus.read(addr);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn writes_to_invalid_address() {
    // arrange
    let mut mem = Memory::new(0, 0x100);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let addr = 0x1000;

    // act
    let actual = address_bus.write(addr, 42);

    // assert
    assert_eq!(actual.is_ok(), false);
    assert_eq!(actual.is_err(), true);
}

#[test]
fn reads_from_invalid_address() {
    // arrange
    let mut mem = Memory::new(0, 0x100);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let addr = 0x1000;

    // act
    let actual = address_bus.read(addr);

    // assert
    assert_eq!(actual.is_ok(), false);
    assert_eq!(actual.is_err(), true);
}

#[test]
fn invalid_mem_block_size() {
    // arrange
    let mut mem = Memory::from_vec(0, vec![0x01, 0x02, 0x03, 0x00]);
    let mut address_bus = AddressBus::new(0x100);

    // act
    let actual = address_bus.add_component(0, 0x100, &mut (mem));

    // assert
    assert_eq!(actual.is_ok(), false);
    assert_eq!(actual.is_err(), true);
}

#[test]
fn load_rom() {
    // arrange
    let mut rom_monitor = Memory::load_rom(0xFF00, "../roms/Apple1_HexMonitor.bin".to_string());
    let mut address_bus = AddressBus::new(0x100);
    if address_bus
        .add_component(0xFF00, rom_monitor.len(), &mut (rom_monitor))
        .is_err()
    {
        panic!("add_component failed");
    }

    let addr = 0xFF00;

    // act
    let actual = address_bus.read(addr);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
}

#[test]
fn add_component_unaligned_start() {
    // arrange
    let offset = 0x10u16;
    let mut mem = Memory::new(offset, 0x200); // two blocks in size
    let mut bus = AddressBus::new(0x100);

    let size = mem.len();
    assert!(bus.add_component(offset, size, &mut mem).is_ok());

    let addr = offset + size as u16 - 1; // last byte in range
    bus.write(addr, 0xAA).expect("write failed");
    let val = bus.read(addr).unwrap();

    // assert
    assert_eq!(val, 0xAA);
}
