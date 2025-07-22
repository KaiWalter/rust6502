use std::time::Duration;

#[cfg(test)]
use super::*;

#[test]
fn test_output_channel() {
    // arrange
    const DSP: u16 = 0xd012; // write ascii
    const DSPCR: u16 = 0xd013; // control port

    let mut pia = MC6821::new();
    let (tx, rx): (
        crossbeam_channel::Sender<u8>,
        crossbeam_channel::Receiver<u8>,
    ) = crossbeam_channel::unbounded();
    pia.set_output_channel_b(tx);

    let expected = 0x5A;

    // act
    pia.int_write(DSP, 0x7F); // 01111111 -> DDRB : configure all bits except highest bit for output
    pia.int_write(DSPCR, 0x04); // 00000100 -> CRB  : write to output port B
    pia.int_write(DSP, expected);

    // assert
    let actual = rx
        .recv_timeout(Duration::from_secs(1))
        .expect("timeout waiting for output");
    assert_eq!(actual, expected);
}

#[test]
fn test_input_channel() {
    // arrange
    let mut pia = MC6821::new();
    let (tx, rx): (
        crossbeam_channel::Sender<InputSignal>,
        crossbeam_channel::Receiver<InputSignal>,
    ) = crossbeam_channel::unbounded();
    pia.set_input_channel(rx);

    const KBD: u16 = 0xd010; // read ascii
    let expected = 0x5A;

    // act
    tx.send(InputSignal::IRA(expected)).unwrap();

    // assert
    assert_eq!(pia.int_read(KBD), expected);
}

#[test]
fn test_ca2_and_cb2_input() {
    // arrange
    let mut pia = MC6821::new();
    let (tx, rx) = crossbeam_channel::unbounded();
    pia.set_input_channel(rx);

    // act
    tx.send(InputSignal::CA2(Signal::Rise)).unwrap();
    tx.send(InputSignal::CB2(Signal::Rise)).unwrap();
    pia.process_input();

    // assert
    assert_eq!(pia.get_ca2(), Signal::Rise);
    assert_eq!(pia.get_cb2(), Signal::Rise);
}
