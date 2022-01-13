// this is the character / text mode implementation
extern crate ncurses;

use ncurses::*;

use core::time;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rust6502::address_bus::*;
use rust6502::mc6821::*;
use rust6502::memory::*;
use rust6502::mos6502::*;

struct ConsoleTerminal {
    pub rx_input: Receiver<u8>,
}

impl ConsoleTerminal {
    pub fn new(rx_output: Receiver<u8>) -> ConsoleTerminal {
        initscr();
        noecho();
        addstr("Apple1 console - hit Ctrl-C to quit\n\n");

        let (tx_input, rx_input) = mpsc::channel();
        thread::spawn(move || loop {
            tx_input.send(getch() as u8).unwrap();
        });

        thread::spawn(move || loop {
            match rx_output.try_recv() {
                Ok(b) => {
                    let c = (b & !0x80).to_ascii_uppercase() as u8;
                    match c {
                        0x0A | 0x0D => {
                            addch('\n' as chtype);
                        }
                        _ => {
                            addch(c as chtype);
                        }
                    }
                    refresh();
                }
                Err(_) => {
                    thread::sleep(time::Duration::from_millis(100));
                }
            }
        });

        ConsoleTerminal { rx_input }
    }

    pub fn check_input(&self) -> Result<u8, TryRecvError> {
        self.rx_input.try_recv()
    }
}

fn main() {
    let mut address_bus = AddressBus::new(0x100);

    let mut mem = Memory::new(0, 4 * 1024);
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component for RAM failed");
    }

    let mut pia = MC6821::new();

    // channel from PIA to terminal (PIA=tx, terminal=rx)
    let (tx_apple_output, rx_apple_output): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    pia.set_output_channel_b(tx_apple_output);
    let terminal = ConsoleTerminal::new(rx_apple_output);

    // channel from keyboard to PIA (keyboard=tx, PIA=rx)
    let (tx_apple_input, rx_apple_input): (Sender<InputSignal>, Receiver<InputSignal>) =
        mpsc::channel();
    pia.set_input_channel(rx_apple_input);
    let check_input = || -> bool {
        let mut stop = false;
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

    let mut rom_monitor = Memory::load_rom(0xFF00, "./roms/Apple1_HexMonitor.bin".to_string());
    if address_bus
        .add_component(0xFF00, rom_monitor.len(), &mut (rom_monitor))
        .is_err()
    {
        panic!("add_component for ROM failed");
    }

    let mut rom_basic = Memory::load_rom(0xE000, "./roms/Apple1_Basic.bin".to_string());
    if address_bus
        .add_component(0xE000, rom_basic.len(), &mut (rom_basic))
        .is_err()
    {
        panic!("add_component for ROM failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);

    cpu.reset();
    cpu.wait_for_system_reset_cycles();

    // main emulation loop
    loop {
        // check input from the terminal and send to PIA
        check_input();

        // processor cycle
        cpu.cycle(false);

        thread::sleep(time::Duration::from_micros(100));
    }
}
