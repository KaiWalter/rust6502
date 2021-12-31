#[cfg(test)]
use super::*;

#[test]
fn writes_and_reads_memory() {
    // arrange
    let mut mem = Memory::new(0, 0x100);
    let expected = 42u8;

    // act
    mem.write(10, 42);
    let actual = mem.read(10);

    // assert
    assert_eq!(expected, actual);
}
