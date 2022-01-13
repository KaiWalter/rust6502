use std::{thread, time::Duration};

#[cfg(test)]
use super::*;

#[test]
fn test_input_channel() {
    // arrange
    const DSP: u16 = 0xd012; // write ascii
    const DSPCR: u16 = 0xd013; // control port

    let mut pia = MC6821::new();
    let expected = 0x5A;
    static mut ACTUAL: u8 = 0;

    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();

    pia.set_output_channel_b(tx);

    // act
    let capture_thread = thread::spawn(move || {
        for _i in 0..100 {
            match rx.try_recv() {
                Ok(b) => {
                    unsafe { ACTUAL = b };
                    break;
                }
                Err(_) => {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    });

    let input_thread = thread::spawn(move || {
        pia.int_write(DSP, 0x7F); // 01111111 -> DDRB : configure all bits except highest bit for output
        pia.int_write(DSPCR, 0x04); // 00000100 -> CRB  : write to output port B
        pia.int_write(DSP, expected);
    });

    input_thread.join().unwrap();
    capture_thread.join().unwrap();

    // assert
    unsafe { assert_eq!(ACTUAL, expected) }
}
