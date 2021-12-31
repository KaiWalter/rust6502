#[cfg(test)]
use super::*;
use crate::memory::*;

#[test]
fn writes_and_reads_memory_block_zero() {
    // arrange
    let mem = Memory::new(0, 0x200);
    let mem_addr: Box<dyn Addressing> = Box::new(mem);
    let mut address_bus = AddressBus::new(0x100);
    address_bus.add_component(0, 0x1FF, mem_addr);
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
    let mem = Memory::new(0, 0x200);
    let mem_addr: Box<dyn Addressing> = Box::new(mem);
    let mut address_bus = AddressBus::new(0x100);
    address_bus.add_component(0, 0x1FF, mem_addr);
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
    let mem = Memory::new(0, 0x200);
    let mem_addr: Box<dyn Addressing> = Box::new(mem);
    let mut address_bus = AddressBus::new(0x100);
    address_bus.add_component(0xF000, 0xF1FF, mem_addr);
    let addr = 0xF010;
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
    let mem = Memory::new(0, 0x100);
    let mem_addr: Box<dyn Addressing> = Box::new(mem);
    let mut address_bus = AddressBus::new(0x100);
    address_bus.add_component(0, 0x0FF, mem_addr);
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
    let mem = Memory::new(0, 0x100);
    let mem_addr: Box<dyn Addressing> = Box::new(mem);
    let mut address_bus = AddressBus::new(0x100);
    address_bus.add_component(0, 0x0FF, mem_addr);
    let addr = 0x1000;

    // act
    let actual = address_bus.read(addr);

    // assert
    assert_eq!(actual.is_ok(), false);
    assert_eq!(actual.is_err(), true);
}
