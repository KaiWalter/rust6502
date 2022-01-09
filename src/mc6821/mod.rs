#[cfg(test)]
mod tests;

use crate::addressing::*;

pub enum SignalProcessing {
    Fall = 0,
    Rise = 1,
}

pub enum InterruptSignal {
    NoSignal = 0,
    IRQ = 1,
    NMI = 2,
    BRK = 3,
}

pub type SendOutputFunction = fn(data: u8);
pub type SendInterruptFunction = fn(data: InterruptSignal);

pub struct MC6821 {
    nORA: u8,      // Output register A
    nIRA: u8,      // Input register A
    nDDRA: u8,     // data direction register A             (Output=1, Input=0)
    nDDRA_neg: u8, // negative data direction register A    (Output=0, Input=1)
    nCA1: Signal,  // control line A1
    nCA2: Signal,  // control line A2

    nCRA: u8, // control register A
    bCRA_Bit0_EnableIRQA1: bool,
    bCRA_Bit1_CA1_PositiveTrans: bool,
    bCRA_Bit2_WritePort: bool,
    bCRA_Bit3_EnableIRQA2: bool,
    bCRA_Bit3_PulseOutput: bool,
    bCRA_Bit3_CA2_set_high: bool,
    bCRA_Bit4_CA2_PositiveTrans: bool,
    bCRA_Bit4_ManualOutput: bool,
    bCRA_Bit5_OutputMode: bool,

    nORB: u8,      // Output register B
    nIRB: u8,      // Input register B
    nDDRB: u8,     // data direction register B             (Output=1, Input=0)
    nDDRB_neg: u8, // negative data direction register B    (Output=0, Input=1)
    nCB1: Signal,  // control line B1
    nCB2: Signal,  // control line B2

    nCRB: u8, // control register B
    bCRB_Bit0_EnableIRQB1: bool,
    bCRB_Bit1_CB1_PositiveTrans: bool,
    bCRB_Bit2_WritePort: bool,
    bCRB_Bit3_EnableIRQB2: bool,
    bCRB_Bit3_PulseOutput: bool,
    bCRB_Bit3_CB2_set_high: bool,
    bCRB_Bit4_CB2_PositiveTrans: bool,
    bCRB_Bit4_ManualOutput: bool,
    bCRB_Bit5_OutputMode: bool,

    fSendOutputA: Option<SendOutputFunction>,
    fSendOutputB: Option<SendOutputFunction>,
    fSendInterrupt: Option<SendInterruptFunction>,
}

impl MC6821 {
    pub fn new() -> MC6821 {
        MC6821 {
            nORA: 0,
            nIRA: 0xFF,
            nIRB: 0,
            nDDRA: 0,
            nDDRA_neg: 0xFF,
            nCA1: Signal::Rise,
            nCA2: Signal::Rise,

            nCRA: 0,
            bCRA_Bit0_EnableIRQA1: false,
            bCRA_Bit1_CA1_PositiveTrans: false,
            bCRA_Bit2_WritePort: false,
            bCRA_Bit3_EnableIRQA2: false,
            bCRA_Bit3_PulseOutput: false,
            bCRA_Bit3_CA2_set_high: false,
            bCRA_Bit4_CA2_PositiveTrans: false,
            bCRA_Bit4_ManualOutput: false,
            bCRA_Bit5_OutputMode: false,

            nORB: 0,
            nDDRB: 0,
            nDDRB_neg: 0xFF,
            nCB1: Signal::Fall,
            nCB2: Signal::Fall,

            nCRB: 0,
            bCRB_Bit0_EnableIRQB1: false,
            bCRB_Bit1_CB1_PositiveTrans: false,
            bCRB_Bit2_WritePort: false,
            bCRB_Bit3_EnableIRQB2: false,
            bCRB_Bit3_PulseOutput: false,
            bCRB_Bit3_CB2_set_high: false,
            bCRB_Bit4_CB2_PositiveTrans: false,
            bCRB_Bit4_ManualOutput: false,
            bCRB_Bit5_OutputMode: false,
        }
    }

    fn update_control_registers(&mut self) {
        // section A -----------------------------------------
        self.bCRA_Bit0_EnableIRQA1 = (self.nCRA & 0x01) == 0x01;
        self.bCRA_Bit1_CA1_PositiveTrans = (self.nCRA & 0x02) == 0x02;
        self.bCRA_Bit2_WritePort = (self.nCRA & 0x04) == 0x04;
        self.bCRA_Bit5_OutputMode = (self.nCRA & 0x20) == 0x20;

        self.bCRA_Bit3_EnableIRQA2 = false;
        self.bCRA_Bit3_PulseOutput = false;
        self.bCRA_Bit3_CA2_set_high = false;
        self.bCRA_Bit4_CA2_PositiveTrans = false;
        self.bCRA_Bit4_ManualOutput = false;

        if (self.bCRA_Bit5_OutputMode) {
            self.bCRA_Bit4_ManualOutput = (self.nCRA & 0x10) == 0x10;
            if (self.bCRA_Bit4_ManualOutput) {
                self.bCRA_Bit3_CA2_set_high = (self.nCRA & 0x08) == 0x08;
                nCA2 = if self.bCRA_Bit3_CA2_set_high {
                    Signal::Rise
                } else {
                    Signal::Fall
                };
            } else {
                self.bCRA_Bit3_PulseOutput = (self.nCRA & 0x08) == 0x08;
            }
        } else {
            self.bCRA_Bit3_EnableIRQA2 = (self.nCRA & 0x08) == 0x08;
            self.bCRA_Bit4_CA2_PositiveTrans = (self.nCRA & 0x10) == 0x10;
        }

        // section B -----------------------------------------
        self.bCRB_Bit0_EnableIRQB1 = (self.nCRB & 0x01) == 0x01;
        self.bCRB_Bit1_CB1_PositiveTrans = (self.nCRB & 0x02) == 0x02;
        self.bCRB_Bit2_WritePort = (self.nCRB & 0x04) == 0x04;
        self.bCRB_Bit5_OutputMode = (self.nCRB & 0x20) == 0x20;

        self.bCRB_Bit3_EnableIRQB2 = false;
        self.bCRB_Bit3_PulseOutput = false;
        self.bCRB_Bit3_CB2_set_high = false;
        self.bCRB_Bit4_CB2_PositiveTrans = false;
        self.bCRB_Bit4_ManualOutput = false;

        if (self.bCRB_Bit5_OutputMode) {
            self.bCRB_Bit4_ManualOutput = (self.nCRB & 0x10) == 0x10;
            if (self.bCRB_Bit4_ManualOutput) {
                self.bCRB_Bit3_CB2_set_high = (self.nCRB & 0x08) == 0x08;
                nCB2 = if self.bCRB_Bit3_CB2_set_high {
                    Signal::Rise
                } else {
                    Signal::Fall
                };
            } else {
                self.bCRB_Bit3_PulseOutput = (self.nCRB & 0x08) == 0x08;
            }
        } else {
            self.bCRB_Bit3_EnableIRQB2 = (self.nCRB & 0x08) == 0x08;
            self.bCRB_Bit4_CB2_PositiveTrans = (self.nCRB & 0x10) == 0x10;
        }
    }

    fn update_IRQ() {
        if (self.bCRA_Bit0_EnableIRQA1 && (self.nCRA & 0x80) == 0x80)
            || (self.bCRA_Bit3_EnableIRQA2 && (self.nCRA & 0x40) == 0x40)
            || (self.bCRB_Bit0_EnableIRQB1 && (self.nCRB & 0x80) == 0x80)
            || (self.bCRB_Bit3_EnableIRQB2 && (self.nCRB & 0x40) == 0x40)
        {
            match fSendInterrupt {
                Some(f) => f(Interrupt::IRQ),
                None => (),
            }
        }
    }

    pub fn setInputA(&mut self, b: u8) {
        self.nIRA = b;
    }

    pub fn setInputB(&mut self, b: u8) {
        self.nIRB = b;
    }

    pub fn setOutputAHandler(&mut self, f: Option<Fn(u8)>) {
        self.fOutputA = f;
    }

    pub fn setOutputBHandler(&mut self, f: Option<Fn(u8)>) {
        self.fOutputB = f;
    }

    pub fn setInterruptHandler(&mut self, f: Option<Fn(Interrupt)>) {
        self.fSendInterrupt = f;
    }

    pub fn setCA1(&mut self, s: Signal) {
        // flag interrupt
        if (self.nCA1 != b
            && (if self.bCRA_Bit1_CA1_PositiveTrans {
                Signal::Rise
            } else {
                Signal::Fall
            }) == b)
        {
            self.nCRA |= 0x80; // set bit 7 IRQA1
            self.updateIRQ();
            if (self.bCRA_Bit5_OutputMode
                && !self.bCRA_Bit4_ManualOutput
                && !self.bCRA_Bit3_PulseOutput)
            {
                // handshake mode
                self.nCA2 = Signal::Rise;
            }
        }
        self.nCA1 = b;
    }

    pub fn getCA1(&self) -> Signal {
        self.nCA1
    }

    pub fn setCA2(&mut self, s: Signal) {
        // flag interrupt
        if (self.nCA2 != b
            && (if self.bCRA_Bit1_CA2_PositiveTrans {
                Signal::Rise
            } else {
                Signal::Fall
            }) == b)
        {
            self.nCRA |= 0x40; // set bit 6 IRQA2
            self.updateIRQ();
        }
        self.nCA1 = b;
    }

    pub fn getCA2(&self) -> Signal {
        self.nCA2
    }

    pub fn setCB1(&mut self, s: Signal) {
        // flag interrupt
        if (self.nCB1 != b
            && (if self.bCRB_Bit1_CB1_PositiveTrans {
                Signal::Rise
            } else {
                Signal::Fall
            }) == b)
        {
            self.nCRB |= 0x80; // set bit 7 IRQB1
            self.updateIRQ();
            if (self.bCRB_Bit5_OutputMode
                && !self.bCRB_Bit4_ManualOutput
                && !self.bCRB_Bit3_PulseOutput)
            {
                // handshake mode
                self.nCB2 = Signal::Rise;
            }
        }
        self.nCB1 = b;
    }

    pub fn getCB1(&self) -> Signal {
        self.nCB1
    }

    pub fn setCB2(&mut self, s: Signal) {
        // flag interrupt
        if (self.nCB2 != b
            && (if self.bCRB_Bit1_CB2_PositiveTrans {
                Signal::Rise
            } else {
                Signal::Fall
            }) == b)
        {
            self.nCRB |= 0x40; // set bit 6 IRQB2
            self.updateIRQ();
        }
        self.nCB1 = b;
    }

    pub fn getCB2(&self) -> Signal {
        self.nCB2
    }
}

impl Addressing for MC6821 {
    fn read(&self, addr: u16) -> u8 {
        let reg = addr & 0x03;
        let mut data = 0u8;

        match reg {
            // PA
            0 => {
                self.nCRA &= 0x3F; // IRQ flags implicitly cleared by a read

                // mix input and output
                data |= self.nORA & self.nDDRA;
                data |= self.nIRA & self.nDDRA_neg;
            }

            // CRA
            1 => {
                data = self.nCRA;
            }

            // PB
            2 => {
                self.nCRB &= 0x3F; // IRQ flags implicitly cleared by a read

                // mix input and output
                data |= self.nORB & self.nDDRB;
                data |= self.nIRB & self.nDDRB_neg;
            }

            // CRB
            3 => {
                data = self.nCRB;
            }
        }

        data
    }

    fn write(&mut self, addr: u16, data: u8) {
        let reg = addr & 0x03;

        match reg {
            // DDRA / PA
            0 => {
                if (bCRA_Bit2_WritePort) {
                    nORA = data; // into output register A
                    match fSendOutputA {
                        Some(f) => {
                            // mix input and output
                            let bOut = 0u8;
                            bOut |= nORA & nDDRA;
                            bOut |= nIRA & nDDRA_neg;
                            f(bOut);
                        }
                    }
                } else {
                    nDDRA = data; // into data direction register A
                    nDDRA_neg = !data;
                }
            }

            // CRA
            1 => {
                self.nCRA = (self.nCRA & 0xC0) | (data & 0x3F); // do not change IRQ flags
                self.update_control_registers();
                self.update_IRQ();
            }

            // DDRB / PB
            2 => {
                if (bCRB_Bit2_WritePort) {
                    nORB = data; // into output register B
                    match fSendOutputB {
                        Some(f) => {
                            // mix input and output
                            let bOut = 0u8;
                            bOut |= self.nORB & self.nDDRB;
                            bOut |= self.nIRB & self.nDDRB_neg;
                            f(bOut);

                            if (self.bCRB_Bit5_OutputMode && !self.bCRB_Bit4_ManualOutput)
                            // handshake on write mode
                            {
                                self.nCB2 = Signal::Fall;
                                if self.bCRB_Bit3_PulseOutput {
                                    self.nCB2 = Signal::Rise
                                };
                            }
                        }
                    }
                } else {
                    self.nDDRB = data; // into data direction register B
                    self.nDDRB_neg = !data;
                }
            }

            // CRB
            3 => {
                self.nCRB = (self.nCRB & 0xC0) | (data & 0x3F); // do not change IRQ flags
                self.update_control_registers();
                self.update_IRQ();
            }
        }
    }
}
