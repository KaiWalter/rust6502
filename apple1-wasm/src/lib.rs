extern crate wasm_bindgen;

use std::fmt;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::mpsc::{Receiver, Sender};
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::*;

use rust6502::address_bus::*;
use rust6502::mc6821::*;
use rust6502::memory::*;
use rust6502::mos6502::*;

mod utils;
mod wasm_terminal;

use utils::set_panic_hook;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();

    let (tx_apple_output, rx_apple_output): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let (tx_apple_input, rx_apple_input): (Sender<InputSignal>, Receiver<InputSignal>) =
        mpsc::channel();

    let mut terminal = wasm_terminal::WasmTerminal::new(rx_apple_output);

    tx_apple_output.send(0x01);
    tx_apple_output.send(0x01);
    terminal.event_loop();
    tx_apple_output.send(0x0A);
    tx_apple_output.send(0x01);
    terminal.event_loop();
}
