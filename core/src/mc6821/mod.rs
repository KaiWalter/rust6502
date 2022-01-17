#[cfg(test)]
mod tests;

use crate::address_bus::InternalAddressing;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

#[derive(PartialEq, Clone, Copy)]
pub enum Signal {
    Fall = 0,
    Rise = 1,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InterruptSignal {
    NoSignal = 0,
    IRQ = 1,
    NMI = 2,
    BRK = 3,
}

pub enum InputSignal {
    IRA(u8),
    IRB(u8),
    CA1(Signal),
    CA2(Signal),
    CB1(Signal),
    CB2(Signal),
}

pub struct MC6821 {
    ora: u8,      // Output register A
    ira: u8,      // Input register A
    ddra: u8,     // data direction register A             (Output=1, Input=0)
    ddra_neg: u8, // negative data direction register A    (Output=0, Input=1)
    ca1: Signal,  // control line A1
    ca2: Signal,  // control line A2

    cra: u8, // control register A
    cra_bit_0_enable_irq_a1: bool,
    cra_bit_1_ca1_positive_trans: bool,
    cra_bit_2_write_port: bool,
    cra_bit_3_enable_irq_a2: bool,
    cra_bit_3_pulse_output: bool,
    cra_bit_3_ca2_set_high: bool,
    cra_bit_4_ca2_positive_trans: bool,
    cra_bit_4_manual_output: bool,
    cra_bit_5_output_mode: bool,

    orb: u8,      // Output register B
    irb: u8,      // Input register B
    ddrb: u8,     // data direction register B             (Output=1, Input=0)
    ddrb_neg: u8, // negative data direction register B    (Output=0, Input=1)
    cb1: Signal,  // control line B1
    cb2: Signal,  // control line B2

    crb: u8, // control register B
    crb_bit_0_enable_irq_b1: bool,
    crb_bit_1_cb1_positive_trans: bool,
    crb_bit_2_write_port: bool,
    crb_bit_3_enable_irq_b2: bool,
    crb_bit_3_pulse_output: bool,
    crb_bit_3_cb2_set_high: bool,
    crb_bit_4_cb2_positive_trans: bool,
    crb_bit_4_manual_output: bool,
    crb_bit_5_output_mode: bool,

    input_channel: Option<Receiver<InputSignal>>,
    output_channel_a: Option<Sender<u8>>,
    output_channel_b: Option<Sender<u8>>,
    interrupt_channel: Option<Sender<InterruptSignal>>,
}

impl MC6821 {
    pub fn new() -> MC6821 {
        MC6821 {
            ora: 0,
            ira: 0xFF,
            irb: 0,
            ddra: 0,
            ddra_neg: 0xFF,
            ca1: Signal::Rise,
            ca2: Signal::Rise,

            cra: 0,
            cra_bit_0_enable_irq_a1: false,
            cra_bit_1_ca1_positive_trans: false,
            cra_bit_2_write_port: false,
            cra_bit_3_enable_irq_a2: false,
            cra_bit_3_pulse_output: false,
            cra_bit_3_ca2_set_high: false,
            cra_bit_4_ca2_positive_trans: false,
            cra_bit_4_manual_output: false,
            cra_bit_5_output_mode: false,

            orb: 0,
            ddrb: 0,
            ddrb_neg: 0xFF,
            cb1: Signal::Fall,
            cb2: Signal::Fall,

            crb: 0,
            crb_bit_0_enable_irq_b1: false,
            crb_bit_1_cb1_positive_trans: false,
            crb_bit_2_write_port: false,
            crb_bit_3_enable_irq_b2: false,
            crb_bit_3_pulse_output: false,
            crb_bit_3_cb2_set_high: false,
            crb_bit_4_cb2_positive_trans: false,
            crb_bit_4_manual_output: false,
            crb_bit_5_output_mode: false,

            input_channel: None,
            output_channel_a: None,
            output_channel_b: None,
            interrupt_channel: None,
        }
    }

    fn update_control_registers(&mut self) {
        // section A -----------------------------------------
        self.cra_bit_0_enable_irq_a1 = (self.cra & 0x01) == 0x01;
        self.cra_bit_1_ca1_positive_trans = (self.cra & 0x02) == 0x02;
        self.cra_bit_2_write_port = (self.cra & 0x04) == 0x04;
        self.cra_bit_5_output_mode = (self.cra & 0x20) == 0x20;

        self.cra_bit_3_enable_irq_a2 = false;
        self.cra_bit_3_pulse_output = false;
        self.cra_bit_3_ca2_set_high = false;
        self.cra_bit_4_ca2_positive_trans = false;
        self.cra_bit_4_manual_output = false;

        if self.cra_bit_5_output_mode {
            self.cra_bit_4_manual_output = (self.cra & 0x10) == 0x10;
            if self.cra_bit_4_manual_output {
                self.cra_bit_3_ca2_set_high = (self.cra & 0x08) == 0x08;
                self.ca2 = if self.cra_bit_3_ca2_set_high {
                    Signal::Rise
                } else {
                    Signal::Fall
                };
            } else {
                self.cra_bit_3_pulse_output = (self.cra & 0x08) == 0x08;
            }
        } else {
            self.cra_bit_3_enable_irq_a2 = (self.cra & 0x08) == 0x08;
            self.cra_bit_4_ca2_positive_trans = (self.cra & 0x10) == 0x10;
        }

        // section B -----------------------------------------
        self.crb_bit_0_enable_irq_b1 = (self.crb & 0x01) == 0x01;
        self.crb_bit_1_cb1_positive_trans = (self.crb & 0x02) == 0x02;
        self.crb_bit_2_write_port = (self.crb & 0x04) == 0x04;
        self.crb_bit_5_output_mode = (self.crb & 0x20) == 0x20;

        self.crb_bit_3_enable_irq_b2 = false;
        self.crb_bit_3_pulse_output = false;
        self.crb_bit_3_cb2_set_high = false;
        self.crb_bit_4_cb2_positive_trans = false;
        self.crb_bit_4_manual_output = false;

        if self.crb_bit_5_output_mode {
            self.crb_bit_4_manual_output = (self.crb & 0x10) == 0x10;
            if self.crb_bit_4_manual_output {
                self.crb_bit_3_cb2_set_high = (self.crb & 0x08) == 0x08;
                self.cb2 = if self.crb_bit_3_cb2_set_high {
                    Signal::Rise
                } else {
                    Signal::Fall
                };
            } else {
                self.crb_bit_3_pulse_output = (self.crb & 0x08) == 0x08;
            }
        } else {
            self.crb_bit_3_enable_irq_b2 = (self.crb & 0x08) == 0x08;
            self.crb_bit_4_cb2_positive_trans = (self.crb & 0x10) == 0x10;
        }
    }

    fn update_irq(&self) {
        if (self.cra_bit_0_enable_irq_a1 && (self.cra & 0x80) == 0x80)
            || (self.cra_bit_3_enable_irq_a2 && (self.cra & 0x40) == 0x40)
            || (self.crb_bit_0_enable_irq_b1 && (self.crb & 0x80) == 0x80)
            || (self.crb_bit_3_enable_irq_b2 && (self.crb & 0x40) == 0x40)
        {
            match &self.interrupt_channel {
                Some(tx) => tx.send(InterruptSignal::IRQ).unwrap(),
                None => (),
            }
        }
    }

    pub fn set_input_channel(&mut self, rx: Receiver<InputSignal>) {
        self.input_channel = Some(rx);
    }

    pub fn process_input(&mut self) {
        loop {
            match &self.input_channel {
                Some(rx) => match rx.try_recv() {
                    Ok(input) => match input {
                        InputSignal::IRA(b) => self.ira = b,
                        InputSignal::IRB(b) => self.irb = b,
                        InputSignal::CA1(s) => self.set_ca1(s),
                        InputSignal::CA2(s) => self.set_ca2(s),
                        InputSignal::CB1(s) => self.set_cb1(s),
                        InputSignal::CB2(s) => self.set_cb2(s),
                    },
                    Err(_) => break,
                },
                None => (),
            }
        }
    }

    pub fn set_output_channel_a(&mut self, tx: Sender<u8>) {
        self.output_channel_a = Some(tx);
    }

    pub fn set_output_channel_b(&mut self, tx: Sender<u8>) {
        self.output_channel_b = Some(tx);
    }

    pub fn set_interrupt_channel(&mut self, tx: Sender<InterruptSignal>) {
        self.interrupt_channel = Some(tx);
    }

    fn set_ca1(&mut self, s: Signal) {
        // flag interrupt
        if self.ca1 != s
            && (if self.cra_bit_1_ca1_positive_trans {
                Signal::Rise
            } else {
                Signal::Fall
            }) == s
        {
            self.cra |= 0x80; // set bit 7 IRQA1
            self.update_irq();
            if self.cra_bit_5_output_mode
                && !self.cra_bit_4_manual_output
                && !self.cra_bit_3_pulse_output
            {
                // handshake mode
                self.ca2 = Signal::Rise;
            }
        }
        self.ca1 = s;
    }

    pub fn get_ca1(&self) -> Signal {
        self.ca1
    }

    fn set_ca2(&mut self, s: Signal) {
        // flag interrupt
        if self.ca2 != s
            && (if self.cra_bit_4_ca2_positive_trans {
                Signal::Rise
            } else {
                Signal::Fall
            }) == s
        {
            self.cra |= 0x40; // set bit 6 IRQA2
            self.update_irq();
        }
        self.ca1 = s;
    }

    pub fn get_ca2(&self) -> Signal {
        self.ca2
    }

    fn set_cb1(&mut self, s: Signal) {
        // flag interrupt
        if self.cb1 != s
            && (if self.crb_bit_1_cb1_positive_trans {
                Signal::Rise
            } else {
                Signal::Fall
            }) == s
        {
            self.crb |= 0x80; // set bit 7 IRQB1
            self.update_irq();
            if self.crb_bit_5_output_mode
                && !self.crb_bit_4_manual_output
                && !self.crb_bit_3_pulse_output
            {
                // handshake mode
                self.cb2 = Signal::Rise;
            }
        }
        self.cb1 = s;
    }

    pub fn get_cb1(&self) -> Signal {
        self.cb1
    }

    fn set_cb2(&mut self, s: Signal) {
        // flag interrupt
        if self.cb2 != s
            && (if self.crb_bit_4_cb2_positive_trans {
                Signal::Rise
            } else {
                Signal::Fall
            }) == s
        {
            self.crb |= 0x40; // set bit 6 IRQB2
            self.update_irq();
        }
        self.cb1 = s;
    }

    pub fn get_cb2(&self) -> Signal {
        self.cb2
    }
}

impl InternalAddressing for MC6821 {
    fn int_read(&mut self, addr: u16) -> u8 {
        self.process_input();

        let reg = (addr & 0x03) as u8;
        let mut data = 0u8;

        match reg {
            // PA
            0 => {
                self.cra &= 0x3F; // IRQ flags implicitly cleared by a read

                // mix input and output
                data |= self.ora & self.ddra;
                data |= self.ira & self.ddra_neg;
            }

            // CRA
            1 => {
                data = self.cra;
            }

            // PB
            2 => {
                self.crb &= 0x3F; // IRQ flags implicitly cleared by a read

                // mix input and output
                data |= self.orb & self.ddrb;
                data |= self.irb & self.ddrb_neg;
            }

            // CRB
            3 => {
                data = self.crb;
            }

            4_u8..=u8::MAX => (),
        }

        data
    }

    fn int_write(&mut self, addr: u16, data: u8) {
        let reg = (addr & 0x03) as u8;

        match reg {
            // DDRA / PA
            0 => {
                if self.cra_bit_2_write_port {
                    self.ora = data; // into output register A
                    match &self.output_channel_a {
                        Some(tx) => {
                            // mix input and output
                            let mut out = 0u8;
                            out |= self.ora & self.ddra;
                            out |= self.ira & self.ddra_neg;
                            tx.send(out).unwrap();
                        }
                        None => (),
                    }
                } else {
                    self.ddra = data; // into data direction register A
                    self.ddra_neg = !data;
                }
            }

            // CRA
            1 => {
                self.cra = (self.cra & 0xC0) | (data & 0x3F); // do not change IRQ flags
                self.update_control_registers();
                self.update_irq();
            }

            // DDRB / PB
            2 => {
                if self.crb_bit_2_write_port {
                    self.orb = data; // into output register B
                    match &self.output_channel_b {
                        Some(tx) => {
                            // mix input and output
                            let mut out = 0u8;
                            out |= self.orb & self.ddrb;
                            out |= self.irb & self.ddrb_neg;
                            tx.send(out).unwrap();

                            if self.crb_bit_5_output_mode && !self.crb_bit_4_manual_output
                            // handshake on write mode
                            {
                                self.cb2 = Signal::Fall;
                                if self.crb_bit_3_pulse_output {
                                    self.cb2 = Signal::Rise
                                };
                            }
                        }
                        None => (),
                    }
                } else {
                    self.ddrb = data; // into data direction register B
                    self.ddrb_neg = !data;
                }
            }

            // CRB
            3 => {
                self.crb = (self.crb & 0xC0) | (data & 0x3F); // do not change IRQ flags
                self.update_control_registers();
                self.update_irq();
            }

            4_u8..=u8::MAX => (),
        }
    }

    fn len(&self) -> usize {
        0
    }
}
