#[cfg(test)]
use super::*;

#[test]
fn writes_and_reads_memory() {
    // arrange
    let mut mem = Memory::new(0, 0x100);
    let expected = 42u8;

    // act
    mem.int_write(10, expected);
    let actual = mem.int_read(10);

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn read_vec_memory() {
    // arrange
    let mut mem = Memory::from_vec(0, vec![0x01, 0x02, 0x03, 0x00]);
    let expected = 42u8;

    // act
    mem.int_write(3, expected);
    let actual = mem.int_read(3);

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn load_rom() {
    // arrange
    let mut rom_monitor = Memory::load_rom(0xFF00, "./roms/Apple1_HexMonitor.bin".to_string());
    let expected = 216u8;

    let addr = 0xFF00;

    // act
    let actual = rom_monitor.int_read(addr);

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn writes_and_reads_external() {
    // arrange
    let mut mem = Memory::new(0, 0x200);

    let addr = 10;
    let expected = 42u8;

    // act
    mem.write(addr, 42).expect("wrong address");
    let actual = mem.read(addr);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn writes_and_reads_external_with_offset() {
    // arrange
    let mut mem = Memory::new(0x100, 0x200);

    let addr = 0x211;
    let expected = 42u8;

    // act
    mem.write(addr, 42).expect("wrong address");
    let actual = mem.read(addr);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn writes_and_reads_external_wrong_lower_addr() {
    // arrange
    let mut mem = Memory::new(0x100, 0x200);

    let addr = 0x050;

    // act
    let actual_write = mem.write(addr, 42);
    let actual_read = mem.read(addr);

    // assert
    assert_eq!(actual_write.is_ok(), false);
    assert_eq!(actual_write.is_err(), true);
    assert_eq!(actual_read.is_ok(), false);
    assert_eq!(actual_read.is_err(), true);
}

#[test]
fn writes_and_reads_external_wrong_higher_addr() {
    // arrange
    let mut mem = Memory::new(0x100, 0x200);

    let addr = 0x301;

    // act
    let actual_write = mem.write(addr, 42);
    let actual_read = mem.read(addr);

    // assert
    assert_eq!(actual_write.is_ok(), false);
    assert_eq!(actual_write.is_err(), true);
    assert_eq!(actual_read.is_ok(), false);
    assert_eq!(actual_read.is_err(), true);
}
