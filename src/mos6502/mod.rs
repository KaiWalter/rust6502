mod addressmodes;
mod operations;
#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::Write;

use crate::address_bus::AddressBus;
use addressmodes::*;
use operations::*;

macro_rules! instr {
    ($name:expr,$operation:expr,$address_mode:expr,$cycles:expr) => {{
        OperationDefinition {
            name: $name,
            operation: $operation,
            address_mode: $address_mode,
            cycles: $cycles,
        }
    }};
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AddressModeResult {
    Absolute,
    Relative,
    Fetched,
}

#[derive(Clone, Copy)]
pub struct AddressModeValues {
    result: AddressModeResult,
    absolute_address: u16,
    relative_address: u16,
    fetched_value: u8,
    add_cycles: u8,
}

#[derive(Debug, Clone, Default)]
pub struct CpuRegisters {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    status: u8,
}

pub struct Cpu<'a> {
    r: CpuRegisters,
    remaining_cycles: u8,
    address_bus: AddressBus<'a>,
    // DEBUG INFORMATION
    current_pc: u16,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CpuError {
    operation: String,
    pc: u16,
}

impl CpuError {
    fn new(operation: &str, pc: u16) -> CpuError {
        CpuError {
            operation: operation.to_string(),
            pc: pc,
        }
    }
}
type AddressModeFunction = fn(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError>;
type OpCodeFunction = fn(cpu: &mut Cpu, address_mode_values: AddressModeValues, opcode: u8) -> u8;

struct OperationDefinition<'a> {
    name: &'a str,
    operation: OpCodeFunction,
    address_mode: AddressModeFunction,
    cycles: u8,
}

static OPCODES: [OperationDefinition; 256] = [
    instr! {"brk", brk, imm, 7}, // 00
    instr! {"ora", ora, izx, 6}, // 01
    instr! {"???", xxx, imp, 2}, // 02
    instr! {"???", xxx, imp, 8}, // 03
    instr! {"???", nop, imp, 3}, // 04
    instr! {"ora", ora, zp0, 3}, // 05
    instr! {"asl", asl, zp0, 5}, // 06
    instr! {"???", xxx, imp, 5}, // 07
    instr! {"php", php, imp, 3}, // 08
    instr! {"ora", ora, imm, 2}, // 09
    instr! {"asl", asl, imp, 2}, // 0A
    instr! {"???", xxx, imp, 2}, // 0B
    instr! {"???", nop, imp, 4}, // 0C
    instr! {"ora", ora, abs, 4}, // 0D
    instr! {"asl", asl, abs, 6}, // 0E
    instr! {"???", xxx, imp, 6}, // 0F
    instr! {"bpl", bpl, rel, 2}, // 10
    instr! {"ora", ora, izy, 5}, // 11
    instr! {"???", xxx, imp, 2}, // 12
    instr! {"???", xxx, imp, 8}, // 13
    instr! {"???", nop, imp, 4}, // 14
    instr! {"ora", ora, zpx, 4}, // 15
    instr! {"asl", asl, zpx, 6}, // 16
    instr! {"???", xxx, imp, 6}, // 17
    instr! {"clc", clc, imp, 2}, // 18
    instr! {"ora", ora, aby, 4}, // 19
    instr! {"???", nop, imp, 2}, // 1A
    instr! {"???", xxx, imp, 7}, // 1B
    instr! {"???", nop, imp, 4}, // 1C
    instr! {"ora", ora, abx, 4}, // 1D
    instr! {"asl", asl, abx, 7}, // 1E
    instr! {"???", xxx, imp, 7}, // 1F
    instr! {"jsr", jsr, abs, 6}, // 20
    instr! {"and", and, izx, 6}, // 21
    instr! {"???", xxx, imp, 2}, // 22
    instr! {"???", xxx, imp, 8}, // 23
    instr! {"bit", bit, zp0, 3}, // 24
    instr! {"and", and, zp0, 3}, // 25
    instr! {"rol", rol, zp0, 5}, // 26
    instr! {"???", xxx, imp, 5}, // 27
    instr! {"plp", plp, imp, 4}, // 28
    instr! {"and", and, imm, 2}, // 29
    instr! {"rol", rol, imp, 2}, // 2A
    instr! {"???", xxx, imp, 2}, // 2B
    instr! {"bit", bit, abs, 4}, // 2C
    instr! {"and", and, abs, 4}, // 2D
    instr! {"rol", rol, abs, 6}, // 2E
    instr! {"???", xxx, imp, 6}, // 2F
    instr! {"bmi", bmi, rel, 2}, // 30
    instr! {"and", and, izy, 5}, // 31
    instr! {"???", xxx, imp, 2}, // 32
    instr! {"???", xxx, imp, 8}, // 33
    instr! {"???", nop, imp, 4}, // 34
    instr! {"and", and, zpx, 4}, // 35
    instr! {"rol", rol, zpx, 6}, // 36
    instr! {"???", xxx, imp, 6}, // 37
    instr! {"sec", sec, imp, 2}, // 38
    instr! {"and", and, aby, 4}, // 39
    instr! {"???", nop, imp, 2}, // 3A
    instr! {"???", xxx, imp, 7}, // 3B
    instr! {"???", nop, imp, 4}, // 3C
    instr! {"and", and, abx, 4}, // 3D
    instr! {"rol", rol, abx, 7}, // 3E
    instr! {"???", xxx, imp, 7}, // 3F
    instr! {"rti", rti, imp, 6}, // 40
    instr! {"eor", eor, izx, 6}, // 41
    instr! {"???", xxx, imp, 2}, // 42
    instr! {"???", xxx, imp, 8}, // 43
    instr! {"???", nop, imp, 3}, // 44
    instr! {"eor", eor, zp0, 3}, // 45
    instr! {"lsr", lsr, zp0, 5}, // 46
    instr! {"???", xxx, imp, 5}, // 47
    instr! {"pha", pha, imp, 3}, // 48
    instr! {"eor", eor, imm, 2}, // 49
    instr! {"lsr", lsr, imp, 2}, // 4A
    instr! {"???", xxx, imp, 2}, // 4B
    instr! {"jmp", jmp, abs, 3}, // 4C
    instr! {"eor", eor, abs, 4}, // 4D
    instr! {"lsr", lsr, abs, 6}, // 4E
    instr! {"???", xxx, imp, 6}, // 4F
    instr! {"bvc", bvc, rel, 2}, // 50
    instr! {"eor", eor, izy, 5}, // 51
    instr! {"???", xxx, imp, 2}, // 52
    instr! {"???", xxx, imp, 8}, // 53
    instr! {"???", nop, imp, 4}, // 54
    instr! {"eor", eor, zpx, 4}, // 55
    instr! {"lsr", lsr, zpx, 6}, // 56
    instr! {"???", xxx, imp, 6}, // 57
    instr! {"cli", cli, imp, 2}, // 58
    instr! {"eor", eor, aby, 4}, // 59
    instr! {"???", nop, imp, 2}, // 5A
    instr! {"???", xxx, imp, 7}, // 5B
    instr! {"???", nop, imp, 4}, // 5C
    instr! {"eor", eor, abx, 4}, // 5D
    instr! {"lsr", lsr, abx, 7}, // 5E
    instr! {"???", xxx, imp, 7}, // 5F
    instr! {"rts", rts, imp, 6}, // 60
    instr! {"adc", adc, izx, 6}, // 61
    instr! {"???", xxx, imp, 2}, // 62
    instr! {"???", xxx, imp, 8}, // 63
    instr! {"???", nop, imp, 3}, // 64
    instr! {"adc", adc, zp0, 3}, // 65
    instr! {"ror", ror, zp0, 5}, // 66
    instr! {"???", xxx, imp, 5}, // 67
    instr! {"pla", pla, imp, 4}, // 68
    instr! {"adc", adc, imm, 2}, // 69
    instr! {"ror", ror, imp, 2}, // 6A
    instr! {"???", xxx, imp, 2}, // 6B
    instr! {"jmp", jmp, ind, 5}, // 6C
    instr! {"adc", adc, abs, 4}, // 6D
    instr! {"ror", ror, abs, 6}, // 6E
    instr! {"???", xxx, imp, 6}, // 6F
    instr! {"bvs", bvs, rel, 2}, // 70
    instr! {"adc", adc, izy, 5}, // 71
    instr! {"???", xxx, imp, 2}, // 72
    instr! {"???", xxx, imp, 8}, // 73
    instr! {"???", nop, imp, 4}, // 74
    instr! {"adc", adc, zpx, 4}, // 75
    instr! {"ror", ror, zpx, 6}, // 76
    instr! {"???", xxx, imp, 6}, // 77
    instr! {"sei", sei, imp, 2}, // 78
    instr! {"adc", adc, aby, 4}, // 79
    instr! {"???", nop, imp, 2}, // 7A
    instr! {"???", xxx, imp, 7}, // 7B
    instr! {"???", nop, imp, 4}, // 7C
    instr! {"adc", adc, abx, 4}, // 7D
    instr! {"ror", ror, abx, 7}, // 7E
    instr! {"???", xxx, imp, 7}, // 7F
    instr! {"???", nop, imp, 2}, // 80
    instr! {"sta", sta, izx, 6}, // 81
    instr! {"???", nop, imp, 2}, // 82
    instr! {"???", xxx, imp, 6}, // 83
    instr! {"sty", sty, zp0, 3}, // 84
    instr! {"sta", sta, zp0, 3}, // 85
    instr! {"stx", stx, zp0, 3}, // 86
    instr! {"???", xxx, imp, 3}, // 87
    instr! {"dey", dey, imp, 2}, // 88
    instr! {"???", nop, imp, 2}, // 89
    instr! {"txa", txa, imp, 2}, // 8A
    instr! {"???", xxx, imp, 2}, // 8B
    instr! {"sty", sty, abs, 4}, // 8C
    instr! {"sta", sta, abs, 4}, // 8D
    instr! {"stx", stx, abs, 4}, // 8E
    instr! {"???", xxx, imp, 4}, // 8F
    instr! {"bcc", bcc, rel, 2}, // 90
    instr! {"sta", sta, izy, 6}, // 91
    instr! {"???", xxx, imp, 2}, // 92
    instr! {"???", xxx, imp, 6}, // 93
    instr! {"sty", sty, zpx, 4}, // 94
    instr! {"sta", sta, zpx, 4}, // 95
    instr! {"stx", stx, zpy, 4}, // 96
    instr! {"???", xxx, imp, 4}, // 97
    instr! {"tya", tya, imp, 2}, // 98
    instr! {"sta", sta, aby, 5}, // 99
    instr! {"txs", txs, imp, 2}, // 9A
    instr! {"???", xxx, imp, 5}, // 9B
    instr! {"???", nop, imp, 5}, // 9C
    instr! {"sta", sta, abx, 5}, // 9D
    instr! {"???", xxx, imp, 5}, // 9E
    instr! {"???", xxx, imp, 5}, // 9F
    instr! {"ldy", ldy, imm, 2}, // A0
    instr! {"lda", lda, izx, 6}, // A1
    instr! {"ldx", ldx, imm, 2}, // A2
    instr! {"???", xxx, imp, 6}, // A3
    instr! {"ldy", ldy, zp0, 3}, // A4
    instr! {"lda", lda, zp0, 3}, // A5
    instr! {"ldx", ldx, zp0, 3}, // A6
    instr! {"???", xxx, imp, 3}, // A7
    instr! {"tay", tay, imp, 2}, // A8
    instr! {"lda", lda, imm, 2}, // A9
    instr! {"tax", tax, imp, 2}, // AA
    instr! {"???", xxx, imp, 2}, // AB
    instr! {"ldy", ldy, abs, 4}, // AC
    instr! {"lda", lda, abs, 4}, // AD
    instr! {"ldx", ldx, abs, 4}, // AE
    instr! {"???", xxx, imp, 4}, // AF
    instr! {"bcs", bcs, rel, 2}, // B0
    instr! {"lda", lda, izy, 5}, // B1
    instr! {"???", xxx, imp, 2}, // B2
    instr! {"???", xxx, imp, 5}, // B3
    instr! {"ldy", ldy, zpx, 4}, // B4
    instr! {"lda", lda, zpx, 4}, // B5
    instr! {"ldx", ldx, zpy, 4}, // B6
    instr! {"???", xxx, imp, 4}, // B7
    instr! {"clv", clv, imp, 2}, // B8
    instr! {"lda", lda, aby, 4}, // B9
    instr! {"tsx", tsx, imp, 2}, // BA
    instr! {"???", xxx, imp, 4}, // BB
    instr! {"ldy", ldy, abx, 4}, // BC
    instr! {"lda", lda, abx, 4}, // BD
    instr! {"ldx", ldx, aby, 4}, // BE
    instr! {"???", xxx, imp, 4}, // BF
    instr! {"cpy", cpy, imm, 2}, // C0
    instr! {"cmp", cmp, izx, 6}, // C1
    instr! {"???", nop, imp, 2}, // C2
    instr! {"???", xxx, imp, 8}, // C3
    instr! {"cpy", cpy, zp0, 3}, // C4
    instr! {"cmp", cmp, zp0, 3}, // C5
    instr! {"dec", dec, zp0, 5}, // C6
    instr! {"???", xxx, imp, 5}, // C7
    instr! {"iny", iny, imp, 2}, // C8
    instr! {"cmp", cmp, imm, 2}, // C9
    instr! {"dex", dex, imp, 2}, // CA
    instr! {"???", xxx, imp, 2}, // CB
    instr! {"cpy", cpy, abs, 4}, // CC
    instr! {"cmp", cmp, abs, 4}, // CD
    instr! {"dec", dec, abs, 6}, // CE
    instr! {"???", xxx, imp, 6}, // CF
    instr! {"bne", bne, rel, 2}, // D0
    instr! {"cmp", cmp, izy, 5}, // D1
    instr! {"???", xxx, imp, 2}, // D2
    instr! {"???", xxx, imp, 8}, // D3
    instr! {"???", nop, imp, 4}, // D4
    instr! {"cmp", cmp, zpx, 4}, // D5
    instr! {"dec", dec, zpx, 6}, // D6
    instr! {"???", xxx, imp, 6}, // D7
    instr! {"cld", cld, imp, 2}, // D8
    instr! {"cmp", cmp, aby, 4}, // D9
    instr! {"nop", nop, imp, 2}, // DA
    instr! {"???", xxx, imp, 7}, // DB
    instr! {"???", nop, imp, 4}, // DC
    instr! {"cmp", cmp, abx, 4}, // DD
    instr! {"dec", dec, abx, 7}, // DE
    instr! {"???", xxx, imp, 7}, // DF
    instr! {"cpx", cpx, imm, 2}, // E0
    instr! {"sbc", sbc, izx, 6}, // E1
    instr! {"???", nop, imp, 2}, // E2
    instr! {"???", xxx, imp, 8}, // E3
    instr! {"cpx", cpx, zp0, 3}, // E4
    instr! {"sbc", sbc, zp0, 3}, // E5
    instr! {"inc", inc, zp0, 5}, // E6
    instr! {"???", xxx, imp, 5}, // E7
    instr! {"inx", inx, imp, 2}, // E8
    instr! {"sbc", sbc, imm, 2}, // E9
    instr! {"nop", nop, imp, 2}, // EA
    instr! {"???", sbc, imp, 2}, // EB
    instr! {"cpx", cpx, abs, 4}, // EC
    instr! {"sbc", sbc, abs, 4}, // ED
    instr! {"inc", inc, abs, 6}, // EE
    instr! {"???", xxx, imp, 6}, // EF
    instr! {"beq", beq, rel, 2}, // F0
    instr! {"sbc", sbc, izy, 5}, // F1
    instr! {"???", xxx, imp, 2}, // F2
    instr! {"???", xxx, imp, 8}, // F3
    instr! {"???", nop, imp, 4}, // F4
    instr! {"sbc", sbc, zpx, 4}, // F5
    instr! {"inc", inc, zpx, 6}, // F6
    instr! {"???", xxx, imp, 6}, // F7
    instr! {"sed", sed, imp, 2}, // F8
    instr! {"sbc", sbc, aby, 4}, // F9
    instr! {"nop", nop, imp, 2}, // FA
    instr! {"???", xxx, imp, 7}, // FB
    instr! {"???", nop, imp, 4}, // FC
    instr! {"sbc", sbc, abx, 4}, // FD
    instr! {"inc", inc, abx, 7}, // FE
    instr! {"???", xxx, imp, 7}, // FF
];

pub enum StatusFlag {
    C = (1 << 0), // Carry Bit
    Z = (1 << 1), // Zero
    I = (1 << 2), // Disable Interrupts
    D = (1 << 3), // Decimal Mode
    B = (1 << 4), // Break
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}

#[allow(dead_code)]
impl<'a> Cpu<'a> {
    pub fn new(r: CpuRegisters, address_bus: AddressBus<'a>) -> Cpu<'a> {
        Cpu {
            r: r,
            remaining_cycles: 0,
            current_pc: 0,
            address_bus: address_bus,
        }
    }

    // ##### FLAGS ####
    pub fn set_flag(&mut self, flag: StatusFlag, value: bool) {
        if value {
            self.r.status |= flag as u8
        } else {
            self.r.status &= !(flag as u8);
        }
    }
    pub fn get_flag(&mut self, flag: StatusFlag) -> bool {
        (self.r.status & flag as u8) != 0
    }

    // ##### CYCLES ####
    pub fn reset(&mut self) {
        self.r.status = 0 | StatusFlag::U as u8;
        self.r.a = 0;
        self.r.x = 0;
        self.r.y = 0;
        self.r.sp = 0xFD;

        match self.address_bus.read(0xFFFC) {
            Ok(lo) => {
                self.r.pc = lo as u16;
                match self.address_bus.read(0xFFFD) {
                    Ok(hi) => {
                        self.r.pc |= (hi as u16) << 8;
                    }
                    Err(e) => panic!("addressing error {}", e),
                }
            }
            Err(e) => panic!("addressing error {}", e),
        }

        self.remaining_cycles = 7;
    }

    pub fn cycle(&mut self, debug: bool) {
        if self.remaining_cycles == 0 {
            match self.address_bus.read(self.r.pc) {
                Ok(opcode) => {
                    self.current_pc = self.r.pc;
                    self.r.pc += 1;

                    let operation = &OPCODES[opcode as usize];

                    if operation.name == "???" {
                        panic!("unknown opcode {:X}", opcode)
                    }

                    self.remaining_cycles = operation.cycles;

                    match (operation.address_mode)(self) {
                        Ok(address_mode_values) => {
                            self.remaining_cycles += address_mode_values.add_cycles;
                            self.remaining_cycles +=
                                (operation.operation)(self, address_mode_values, opcode);

                            if debug {
                                println!(
                                        "{:04x} {} - SP:{:02x} A:{:02x} X:{:02x} Y:{:02x} S:{:02x} {:08b}",
                                        self.current_pc,
                                        operation.name,
                                        self.r.sp,
                                        self.r.a,
                                        self.r.x,
                                        self.r.y,
                                        self.r.status,
                                        self.r.status
                                    );
                            }
                        }
                        Err(e) => panic!("addressing error {:?}", e),
                    }
                }
                Err(e) => panic!("addressing error with PC {:X} {:?}", self.r.pc, e),
            }
        }

        self.remaining_cycles -= 1;
    }

    pub fn cycle_file(&mut self, w: &mut File) {
        if self.remaining_cycles == 0 {
            match self.address_bus.read(self.r.pc) {
                Ok(opcode) => {
                    self.current_pc = self.r.pc;
                    self.r.pc += 1;

                    let operation = &OPCODES[opcode as usize];

                    if operation.name == "???" {
                        panic!("unknown opcode {:X}", opcode)
                    }

                    self.remaining_cycles = operation.cycles;

                    match (operation.address_mode)(self) {
                        Ok(address_mode_values) => {
                            self.remaining_cycles += address_mode_values.add_cycles;
                            self.remaining_cycles +=
                                (operation.operation)(self, address_mode_values, opcode);

                            writeln!(
                                w,
                                "{:04x} {} - SP:{:02x} A:{:02x} X:{:02x} Y:{:02x} S:{:02x} {:08b}",
                                self.current_pc,
                                operation.name,
                                self.r.sp,
                                self.r.a,
                                self.r.x,
                                self.r.y,
                                self.r.status,
                                self.r.status
                            )
                            .unwrap();
                        }
                        Err(e) => panic!("addressing error {:?}", e),
                    }
                }
                Err(e) => panic!("addressing error with PC {:X} {:?}", self.r.pc, e),
            }
        }

        self.remaining_cycles = 0; // skip cycles
    }

    pub fn wait_for_system_reset_cycles(&mut self) {
        while self.remaining_cycles > 0 {
            self.cycle(false);
        }
    }

    pub fn run(&mut self, from_addr: u16, to_addr: u16) {
        self.reset();
        self.r.pc = from_addr;
        self.wait_for_system_reset_cycles();
        let mut prev_pc = self.r.pc;

        while self.current_pc != to_addr {
            if self.remaining_cycles == 0 {
                if prev_pc == self.current_pc {
                    panic!("infinite loop at {:X}", self.current_pc);
                }
                prev_pc = self.current_pc;
            }
            self.cycle(false);
        }
    }
}
