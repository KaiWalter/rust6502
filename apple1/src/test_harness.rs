use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use rust6502::address_bus::{AddressBus, InternalAddressing};
use rust6502::mc6821::{InputSignal, MC6821, Signal};
use rust6502::memory::Memory;
use rust6502::mos6502::{Cpu, CpuRegisters};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

enum HarnessCommand {
    Type(Vec<u8>),
    RunCycles(usize, Sender<()>),
    Peek(u16, Sender<u8>),
    Stop,
}

pub struct Apple1ConsoleHarness {
    tx_command: Sender<HarnessCommand>,
    rx_output: Receiver<u8>,
    worker: Option<JoinHandle<()>>,
}

impl Apple1ConsoleHarness {
    pub fn start() -> Apple1ConsoleHarness {
        let (tx_command, rx_command): (Sender<HarnessCommand>, Receiver<HarnessCommand>) =
            unbounded();
        let (tx_output, rx_output): (Sender<u8>, Receiver<u8>) = unbounded();

        let worker = thread::spawn(move || {
            let mut address_bus = AddressBus::new(0x100);

            let mut mem = Memory::new(0, 4 * 1024);
            if address_bus.add_component(0, mem.len(), &mut mem).is_err() {
                panic!("add_component for RAM failed");
            }

            let mut pia = MC6821::new();
            pia.set_output_channel_b(tx_output);

            let (tx_apple_input, rx_apple_input): (Sender<InputSignal>, Receiver<InputSignal>) =
                unbounded();
            pia.set_input_channel(rx_apple_input);

            if address_bus.add_component(0xD000, 0x100, &mut pia).is_err() {
                panic!("add_component PIA failed");
            }

            let mut rom_monitor =
                Memory::load_rom(0xFF00, rom_path("Apple1_HexMonitor.bin"));
            if address_bus
                .add_component(0xFF00, rom_monitor.len(), &mut rom_monitor)
                .is_err()
            {
                panic!("add_component for monitor ROM failed");
            }

            let mut rom_basic = Memory::load_rom(0xE000, rom_path("Apple1_Basic.bin"));
            if address_bus
                .add_component(0xE000, rom_basic.len(), &mut rom_basic)
                .is_err()
            {
                panic!("add_component for BASIC ROM failed");
            }

            let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
            cpu.reset();
            cpu.wait_for_system_reset_cycles();

            let mut pending_input: VecDeque<u8> = VecDeque::new();
            const KEYBOARD_INPUT_EVERY_N_CYCLES: usize = 5_000;
            let mut key_input_counter = KEYBOARD_INPUT_EVERY_N_CYCLES;

            while let Ok(command) = rx_command.recv() {
                match command {
                    HarnessCommand::Type(bytes) => {
                        for b in bytes {
                            pending_input.push_back(b);
                        }
                    }
                    HarnessCommand::RunCycles(cycles, tx_done) => {
                        for _ in 0..cycles {
                            if key_input_counter >= KEYBOARD_INPUT_EVERY_N_CYCLES {
                                if let Some(c) = pending_input.pop_front() {
                                    inject_keyboard_byte(c, &tx_apple_input);
                                    key_input_counter = 0;
                                }
                            }

                            cpu.cycle(false);
                            key_input_counter += 1;
                        }
                        tx_done.send(()).unwrap();
                    }
                    HarnessCommand::Peek(addr, tx_value) => {
                        tx_value.send(cpu.read(addr)).unwrap();
                    }
                    HarnessCommand::Stop => break,
                }
            }
        });

        Apple1ConsoleHarness {
            tx_command,
            rx_output,
            worker: Some(worker),
        }
    }

    pub fn type_text(&self, input: &str) {
        self.tx_command
            .send(HarnessCommand::Type(input.as_bytes().to_vec()))
            .unwrap();
    }

    pub fn run_cycles(&self, cycles: usize) {
        let (tx_done, rx_done) = unbounded();
        self.tx_command
            .send(HarnessCommand::RunCycles(cycles, tx_done))
            .unwrap();
        rx_done.recv().unwrap();
    }

    pub fn peek_memory(&self, addr: u16) -> u8 {
        let (tx_value, rx_value) = unbounded();
        self.tx_command
            .send(HarnessCommand::Peek(addr, tx_value))
            .unwrap();
        rx_value.recv().unwrap()
    }

    pub fn drain_output_string(&self) -> String {
        let mut output = String::new();
        loop {
            match self.rx_output.try_recv() {
                Ok(b) => {
                    let c = (b & !0x80).to_ascii_uppercase();
                    match c {
                        0x0A | 0x0D => output.push('\n'),
                        _ => output.push(c as char),
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
        output
    }
}

impl Drop for Apple1ConsoleHarness {
    fn drop(&mut self) {
        let _ = self.tx_command.send(HarnessCommand::Stop);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn rom_path(file: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("roms");
    path.push(file);
    path.to_string_lossy().to_string()
}

fn inject_keyboard_byte(mut c: u8, tx_apple_input: &Sender<InputSignal>) {
    if c == b'\n' {
        c = b'\r';
    }

    tx_apple_input.send(InputSignal::CA1(Signal::Fall)).unwrap();
    tx_apple_input
        .send(InputSignal::IRA(c.to_ascii_uppercase() | 0x80))
        .unwrap();
    tx_apple_input.send(InputSignal::CA1(Signal::Rise)).unwrap();
    tx_apple_input.send(InputSignal::CA1(Signal::Fall)).unwrap();
}
