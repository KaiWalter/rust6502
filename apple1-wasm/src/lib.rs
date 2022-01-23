#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;

use crossbeam_channel::*;
use rust6502::mc6821::*;
use rust6502::memory::*;
use rust6502::mos6502::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::{prelude::*, JsCast};

mod apple1_compact;
mod wasm_helpers;
mod wasm_terminal;

use apple1_compact::*;
use wasm_helpers::*;
use wasm_terminal::*;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

static mut COMPACT_APPLE1: Apple1Compact = Apple1Compact {
    cpu: None,
    bus: None,
    terminal: None,
    check_input: None,
};

static mut TX_APPLE_INPUT: Option<Mutex<Sender<InputSignal>>> = None;

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();

    unsafe {
        // channel from PIA to terminal (PIA=tx, terminal=rx)
        let (tx_apple_output, rx_apple_output): (Sender<u8>, Receiver<u8>) = unbounded();
        // channel from keyboard to PIA (keyboard=tx, PIA=rx)
        let (tx_apple_input, rx_apple_input): (Sender<InputSignal>, Receiver<InputSignal>) =
            unbounded();

        TX_APPLE_INPUT = Some(Mutex::new(tx_apple_input));

        COMPACT_APPLE1.bus = Some(Apple1CompactBus {
            mem: Some(Memory::new(0, 4 * 1024)),
            rom_monitor: Some(Memory::from_vec(
                0xFF00,
                vec![
                    0xd8, 0x58, 0xa0, 0x7f, 0x8c, 0x12, 0xd0, 0xa9, 0xa7, 0x8d, 0x11, 0xd0, 0x8d,
                    0x13, 0xd0, 0xc9, 0xdf, 0xf0, 0x13, 0xc9, 0x9b, 0xf0, 0x03, 0xc8, 0x10, 0x0f,
                    0xa9, 0xdc, 0x20, 0xef, 0xff, 0xa9, 0x8d, 0x20, 0xef, 0xff, 0xa0, 0x01, 0x88,
                    0x30, 0xf6, 0xad, 0x11, 0xd0, 0x10, 0xfb, 0xad, 0x10, 0xd0, 0x99, 0x00, 0x02,
                    0x20, 0xef, 0xff, 0xc9, 0x8d, 0xd0, 0xd4, 0xa0, 0xff, 0xa9, 0x00, 0xaa, 0x0a,
                    0x85, 0x2b, 0xc8, 0xb9, 0x00, 0x02, 0xc9, 0x8d, 0xf0, 0xd4, 0xc9, 0xae, 0x90,
                    0xf4, 0xf0, 0xf0, 0xc9, 0xba, 0xf0, 0xeb, 0xc9, 0xd2, 0xf0, 0x3b, 0x86, 0x28,
                    0x86, 0x29, 0x84, 0x2a, 0xb9, 0x00, 0x02, 0x49, 0xb0, 0xc9, 0x0a, 0x90, 0x06,
                    0x69, 0x88, 0xc9, 0xfa, 0x90, 0x11, 0x0a, 0x0a, 0x0a, 0x0a, 0xa2, 0x04, 0x0a,
                    0x26, 0x28, 0x26, 0x29, 0xca, 0xd0, 0xf8, 0xc8, 0xd0, 0xe0, 0xc4, 0x2a, 0xf0,
                    0x97, 0x24, 0x2b, 0x50, 0x10, 0xa5, 0x28, 0x81, 0x26, 0xe6, 0x26, 0xd0, 0xb5,
                    0xe6, 0x27, 0x4c, 0x44, 0xff, 0x6c, 0x24, 0x00, 0x30, 0x2b, 0xa2, 0x02, 0xb5,
                    0x27, 0x95, 0x25, 0x95, 0x23, 0xca, 0xd0, 0xf7, 0xd0, 0x14, 0xa9, 0x8d, 0x20,
                    0xef, 0xff, 0xa5, 0x25, 0x20, 0xdc, 0xff, 0xa5, 0x24, 0x20, 0xdc, 0xff, 0xa9,
                    0xba, 0x20, 0xef, 0xff, 0xa9, 0xa0, 0x20, 0xef, 0xff, 0xa1, 0x24, 0x20, 0xdc,
                    0xff, 0x86, 0x2b, 0xa5, 0x24, 0xc5, 0x28, 0xa5, 0x25, 0xe5, 0x29, 0xb0, 0xc1,
                    0xe6, 0x24, 0xd0, 0x02, 0xe6, 0x25, 0xa5, 0x24, 0x29, 0x07, 0x10, 0xc8, 0x48,
                    0x4a, 0x4a, 0x4a, 0x4a, 0x20, 0xe5, 0xff, 0x68, 0x29, 0x0f, 0x09, 0xb0, 0xc9,
                    0xba, 0x90, 0x02, 0x69, 0x06, 0x2c, 0x12, 0xd0, 0x30, 0xfb, 0x8d, 0x12, 0xd0,
                    0x60, 0x00, 0x00, 0x00, 0x0f, 0x00, 0xff, 0x00, 0x00,
                ],
            )),
            pia: Some(MC6821::new()),
        });
        if let Some(bus) = COMPACT_APPLE1.bus.as_mut() {
            if let Some(pia) = bus.pia.as_mut() {
                pia.set_output_channel_b(tx_apple_output);
                pia.set_input_channel(rx_apple_input);
            } else {
                log("cannot access PIA for initialization");
            }

            COMPACT_APPLE1.cpu = Some(Cpu::new(CpuRegisters::default(), bus));
        } else {
            log("cannot access bus for initialization");
        }
        if let Some(cpu) = COMPACT_APPLE1.cpu.as_mut() {
            cpu.reset();
            cpu.wait_for_system_reset_cycles();
        } else {
            log("cannot access CPU for initialization");
        }

        COMPACT_APPLE1.terminal = Some(WasmTerminal::new(rx_apple_output));

        COMPACT_APPLE1.check_input = Some(Box::new(|| {
            if let Some(terminal) = COMPACT_APPLE1.terminal.as_mut() {
                terminal.event_loop();
                if let Ok(mut c) = terminal.check_input() {
                    match c {
                        0x0A => c = 0x0D,
                        0xBE => c = 0x2E,
                        _ => {}
                    };

                    let tx_apple_input = TX_APPLE_INPUT.as_ref().unwrap().lock().unwrap().clone();

                    tx_apple_input.send(InputSignal::CA1(Signal::Fall)).unwrap();
                    tx_apple_input
                        .send(InputSignal::IRA(c.to_ascii_uppercase() as u8 | 0x80))
                        .unwrap();
                    tx_apple_input.send(InputSignal::CA1(Signal::Rise)).unwrap();
                    tx_apple_input.send(InputSignal::CA1(Signal::Fall)).unwrap();
                }
            } else {
                log("cannot access terminal for initialization");
            }
        }));
    }

    let inner = Rc::new(RefCell::new(None));
    let outer = inner.clone();

    *outer.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        unsafe {
            // check input from the terminal and send to PIA
            if let Some(check_input) = COMPACT_APPLE1.check_input.as_mut() {
                check_input();
            }

            // do minimum x processor cycles before checking input again
            if let Some(cpu) = COMPACT_APPLE1.cpu.as_mut() {
                let mut counter = 60;
                cpu.cycle(false);
                while counter > 0 && !cpu.completed_operation_cycles() {
                    cpu.cycle(false);
                    counter -= 1;
                }
            } else {
                log("cannot access CPU");
            }
        }

        request_animation_frame(inner.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(outer.borrow().as_ref().unwrap());
}
