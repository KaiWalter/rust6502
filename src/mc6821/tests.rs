#[cfg(test)]
use super::*;

#[test]
fn test_apple1_keyboard_to_screen() {
    // arrange
    const DSP: u16 = 0xd012; // write ascii
    const DSPCR: u16 = 0xd013; // control port

    let mut pia = MC6821::new();
    let expected = 0x5A;
    static mut ACTUAL: u8 = 0;

    fn capture(b: u8) {
        unsafe { ACTUAL = b }
    }

    pia.set_output_b_handler(capture);

    // act
    pia.int_write(DSP, 0x7F); // 01111111 -> DDRB : configure all bits except highest bit for output
    pia.int_write(DSPCR, 0x04); // 00000100 -> CRB  : write to output port B
    pia.int_write(DSP, expected);

    // assert
    unsafe { assert_eq!(ACTUAL, expected) }
}
