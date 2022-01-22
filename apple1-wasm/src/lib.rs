extern crate wasm_bindgen;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::mpsc::{Receiver, Sender};
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::*;

use rust6502::address_bus::*;
use rust6502::mc6821::*;
use rust6502::memory::*;
use rust6502::mos6502::*;

mod wasm_helpers;
mod wasm_terminal;

use wasm_helpers::*;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();

    let mut address_bus = AddressBus::new(0x100);

    let mut mem = Memory::new(0, 4 * 1024);
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component for RAM failed");
    }

    let mut pia = MC6821::new();

    // channel from PIA to terminal (PIA=tx, terminal=rx)
    let (tx_apple_output, rx_apple_output): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    pia.set_output_channel_b(tx_apple_output);
    let mut terminal = wasm_terminal::WasmTerminal::new(rx_apple_output);

    // channel from keyboard to PIA (keyboard=tx, PIA=rx)
    let (tx_apple_input, rx_apple_input): (Sender<InputSignal>, Receiver<InputSignal>) =
        mpsc::channel();
    pia.set_input_channel(rx_apple_input);
    let mut check_input = || -> bool {
        let mut stop = false;
        terminal.event_loop();
        if let Ok(mut c) = terminal.check_input() {
            match c {
                0x03 => stop = true, // ^c
                0x0A => c = 0x0D,
                _ => {}
            };

            tx_apple_input.send(InputSignal::CA1(Signal::Fall)).unwrap();
            tx_apple_input
                .send(InputSignal::IRA(c.to_ascii_uppercase() as u8 | 0x80))
                .unwrap();
            tx_apple_input.send(InputSignal::CA1(Signal::Rise)).unwrap();
            tx_apple_input.send(InputSignal::CA1(Signal::Fall)).unwrap();
        }
        stop
    };

    if address_bus
        .add_component(0xD000, 0x100, &mut (pia))
        .is_err()
    {
        panic!("add_component PIA failed");
    }

    let mut rom_monitor = Memory::from_vec(
        0xFF00,
        vec![
            0xd8, 0x58, 0xa0, 0x7f, 0x8c, 0x12, 0xd0, 0xa9, 0xa7, 0x8d, 0x11, 0xd0, 0x8d, 0x13,
            0xd0, 0xc9, 0xdf, 0xf0, 0x13, 0xc9, 0x9b, 0xf0, 0x03, 0xc8, 0x10, 0x0f, 0xa9, 0xdc,
            0x20, 0xef, 0xff, 0xa9, 0x8d, 0x20, 0xef, 0xff, 0xa0, 0x01, 0x88, 0x30, 0xf6, 0xad,
            0x11, 0xd0, 0x10, 0xfb, 0xad, 0x10, 0xd0, 0x99, 0x00, 0x02, 0x20, 0xef, 0xff, 0xc9,
            0x8d, 0xd0, 0xd4, 0xa0, 0xff, 0xa9, 0x00, 0xaa, 0x0a, 0x85, 0x2b, 0xc8, 0xb9, 0x00,
            0x02, 0xc9, 0x8d, 0xf0, 0xd4, 0xc9, 0xae, 0x90, 0xf4, 0xf0, 0xf0, 0xc9, 0xba, 0xf0,
            0xeb, 0xc9, 0xd2, 0xf0, 0x3b, 0x86, 0x28, 0x86, 0x29, 0x84, 0x2a, 0xb9, 0x00, 0x02,
            0x49, 0xb0, 0xc9, 0x0a, 0x90, 0x06, 0x69, 0x88, 0xc9, 0xfa, 0x90, 0x11, 0x0a, 0x0a,
            0x0a, 0x0a, 0xa2, 0x04, 0x0a, 0x26, 0x28, 0x26, 0x29, 0xca, 0xd0, 0xf8, 0xc8, 0xd0,
            0xe0, 0xc4, 0x2a, 0xf0, 0x97, 0x24, 0x2b, 0x50, 0x10, 0xa5, 0x28, 0x81, 0x26, 0xe6,
            0x26, 0xd0, 0xb5, 0xe6, 0x27, 0x4c, 0x44, 0xff, 0x6c, 0x24, 0x00, 0x30, 0x2b, 0xa2,
            0x02, 0xb5, 0x27, 0x95, 0x25, 0x95, 0x23, 0xca, 0xd0, 0xf7, 0xd0, 0x14, 0xa9, 0x8d,
            0x20, 0xef, 0xff, 0xa5, 0x25, 0x20, 0xdc, 0xff, 0xa5, 0x24, 0x20, 0xdc, 0xff, 0xa9,
            0xba, 0x20, 0xef, 0xff, 0xa9, 0xa0, 0x20, 0xef, 0xff, 0xa1, 0x24, 0x20, 0xdc, 0xff,
            0x86, 0x2b, 0xa5, 0x24, 0xc5, 0x28, 0xa5, 0x25, 0xe5, 0x29, 0xb0, 0xc1, 0xe6, 0x24,
            0xd0, 0x02, 0xe6, 0x25, 0xa5, 0x24, 0x29, 0x07, 0x10, 0xc8, 0x48, 0x4a, 0x4a, 0x4a,
            0x4a, 0x20, 0xe5, 0xff, 0x68, 0x29, 0x0f, 0x09, 0xb0, 0xc9, 0xba, 0x90, 0x02, 0x69,
            0x06, 0x2c, 0x12, 0xd0, 0x30, 0xfb, 0x8d, 0x12, 0xd0, 0x60, 0x00, 0x00, 0x00, 0x0f,
            0x00, 0xff, 0x00, 0x00,
        ],
    );
    if address_bus
        .add_component(0xFF00, rom_monitor.len(), &mut rom_monitor)
        .is_err()
    {
        panic!("add_component for ROM failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);

    cpu.reset();
    cpu.wait_for_system_reset_cycles();
    log("before main loop");

    // main emulation loop
    loop {
        // check input from the terminal and send to PIA
        check_input();

        // processor cycle
        cpu.cycle(false);

        log("cycle");
    }

    // let f = Rc::new(RefCell::new(None));
    // let g = f.clone();

    // let mut i = 0;
    // *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    //     if i > 300 {
    //         // Drop our handle to this closure so that it will get cleaned
    //         // up once we return.
    //         let _ = f.borrow_mut().take();
    //         return;
    //     }

    //     // check input from the terminal and send to PIA
    //     check_input();

    //     // processor cycle
    //     cpu.cycle(false);

    //     i += 1;

    //     // Schedule ourself for another requestAnimationFrame callback.
    //     request_animation_frame(f.borrow().as_ref().unwrap());
    // }) as Box<dyn FnMut()>));

    // request_animation_frame(g.borrow().as_ref().unwrap());

    // tx_apple_output.send(0x01);
    // tx_apple_output.send(0x01);
    // terminal.event_loop();
    // tx_apple_output.send(0x0A);
    // tx_apple_output.send(0x01);
    // terminal.event_loop();
}
