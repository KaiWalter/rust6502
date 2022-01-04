#[cfg(test)]
use super::*;

#[test]
fn writes_and_reads_memory() {
    // arrange
    let mut mem = Memory::new(0, 0x100);
    let expected = 42u8;

    // act
    mem.write(10, expected);
    let actual = mem.read(10);

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn read_vec_memory() {
    // arrange
    let mut mem = Memory::from_vec(0, vec![0x01, 0x02, 0x03, 0x00]);
    let expected = 42u8;

    // act
    mem.write(3, expected);
    let actual = mem.read(3);

    // assert
    assert_eq!(expected, actual);
}

#[test]
fn load_rom() {
    // arrange
    let rom_monitor = Memory::load_rom(0xFF00, "./roms/Apple1_HexMonitor.bin".to_string());
    let expected = 216u8;

    let addr = 0xFF00;

    // act
    let actual = rom_monitor.read(addr);

    // assert
    assert_eq!(expected, actual);
}
