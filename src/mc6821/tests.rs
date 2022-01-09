#[cfg(test)]
use super::*;

#[test]
fn Test_Input_Output() {
    // arrange
    const kbd: u16 = 0xd010; // read key
    const kbdcr: u16 = 0xd011; // control port
    const dsp: u16 = 0xd012; // write ascii
    const dspcr: u16 = 0xd013; // control port

    let mut pia = MC6821::new();
    let expected = 0x5A;
    let mut actual = 0u8;

    fn screenOutputChannel(b: u8) {
        actual = b;
    }

    pia.set_output_channel(screenOutputChannel);

    // act
    pia.Write(dsp, 0x7F); // 01111111 -> DDRB : configure all bits except highest bit for output
    pia.Write(dspcr, 0x04); // 00000100 -> CRB  : write to output port B
    pia.Write(dsp, expected);

    // assert
    assert_eq!(actual, expected);
}
