mod addressmodes;
mod operations;
#[cfg(test)]
mod tests;

use std::str::ParseBoolError;

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

#[derive(Debug, PartialEq)]
pub enum AddressModeResult {
    absolute,
    relative,
    fetched,
}

pub struct AddressModeValues {
    result: AddressModeResult,
    absolute_address: u16,
    relative_address: u16,
    fetched_value: u8,
    add_cycles: u8,
}

#[derive(Debug, Clone)]
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
    address_bus: AddressBus<'a>,
}

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
type OpCodeFunction = fn(cpu: &mut Cpu, address_mode_values: AddressModeValues);

struct OperationDefinition<'a> {
    name: &'a str,
    operation: OpCodeFunction,
    address_mode: AddressModeFunction,
    cycles: u8,
}

static OPCODES: [OperationDefinition; 256] = [
    instr! {"brk", brk, imm, 7},
    instr! {"ora", ora, izx, 6},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"???", nop, imp, 3},
    instr! {"ora", ora, zp0, 3},
    instr! {"asl", asl, zp0, 5},
    instr! {"???", xxx, imp, 5},
    instr! {"php", php, imp, 3},
    instr! {"ora", ora, imm, 2},
    instr! {"asl", asl, imp, 2},
    instr! {"???", xxx, imp, 2},
    instr! {"???", nop, imp, 4},
    instr! {"ora", ora, abs, 4},
    instr! {"asl", asl, abs, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"bpl", bpl, rel, 2},
    instr! {"ora", ora, izy, 5},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"???", nop, imp, 4},
    instr! {"ora", ora, zpx, 4},
    instr! {"asl", asl, zpx, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"clc", clc, imp, 2},
    instr! {"ora", ora, aby, 4},
    instr! {"???", nop, imp, 2},
    instr! {"???", xxx, imp, 7},
    instr! {"???", nop, imp, 4},
    instr! {"ora", ora, abx, 4},
    instr! {"asl", asl, abx, 7},
    instr! {"???", xxx, imp, 7},
    instr! {"jsr", jsr, abs, 6},
    instr! {"and", and, izx, 6},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"bit", bit, zp0, 3},
    instr! {"and", and, zp0, 3},
    instr! {"rol", rol, zp0, 5},
    instr! {"???", xxx, imp, 5},
    instr! {"plp", plp, imp, 4},
    instr! {"and", and, imm, 2},
    instr! {"rol", rol, imp, 2},
    instr! {"???", xxx, imp, 2},
    instr! {"bit", bit, abs, 4},
    instr! {"and", and, abs, 4},
    instr! {"rol", rol, abs, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"bmi", bmi, rel, 2},
    instr! {"and", and, izy, 5},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"???", nop, imp, 4},
    instr! {"and", and, zpx, 4},
    instr! {"rol", rol, zpx, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"sec", sec, imp, 2},
    instr! {"and", and, aby, 4},
    instr! {"???", nop, imp, 2},
    instr! {"???", xxx, imp, 7},
    instr! {"???", nop, imp, 4},
    instr! {"and", and, abx, 4},
    instr! {"rol", rol, abx, 7},
    instr! {"???", xxx, imp, 7},
    instr! {"rti", rti, imp, 6},
    instr! {"eor", eor, izx, 6},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"???", nop, imp, 3},
    instr! {"eor", eor, zp0, 3},
    instr! {"lsr", lsr, zp0, 5},
    instr! {"???", xxx, imp, 5},
    instr! {"pha", pha, imp, 3},
    instr! {"eor", eor, imm, 2},
    instr! {"lsr", lsr, imp, 2},
    instr! {"???", xxx, imp, 2},
    instr! {"jmp", jmp, abs, 3},
    instr! {"eor", eor, abs, 4},
    instr! {"lsr", lsr, abs, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"bvc", bvc, rel, 2},
    instr! {"eor", eor, izy, 5},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"???", nop, imp, 4},
    instr! {"eor", eor, zpx, 4},
    instr! {"lsr", lsr, zpx, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"cli", cli, imp, 2},
    instr! {"eor", eor, aby, 4},
    instr! {"???", nop, imp, 2},
    instr! {"???", xxx, imp, 7},
    instr! {"???", nop, imp, 4},
    instr! {"eor", eor, abx, 4},
    instr! {"lsr", lsr, abx, 7},
    instr! {"???", xxx, imp, 7},
    instr! {"rts", rts, imp, 6},
    instr! {"adc", adc, izx, 6},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"???", nop, imp, 3},
    instr! {"adc", adc, zp0, 3},
    instr! {"ror", ror, zp0, 5},
    instr! {"???", xxx, imp, 5},
    instr! {"pla", pla, imp, 4},
    instr! {"adc", adc, imm, 2},
    instr! {"ror", ror, imp, 2},
    instr! {"???", xxx, imp, 2},
    instr! {"jmp", jmp, ind, 5},
    instr! {"adc", adc, abs, 4},
    instr! {"ror", ror, abs, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"bvs", bvs, rel, 2},
    instr! {"adc", adc, izy, 5},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"???", nop, imp, 4},
    instr! {"adc", adc, zpx, 4},
    instr! {"ror", ror, zpx, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"sei", sei, imp, 2},
    instr! {"adc", adc, aby, 4},
    instr! {"???", nop, imp, 2},
    instr! {"???", xxx, imp, 7},
    instr! {"???", nop, imp, 4},
    instr! {"adc", adc, abx, 4},
    instr! {"ror", ror, abx, 7},
    instr! {"???", xxx, imp, 7},
    instr! {"???", nop, imp, 2},
    instr! {"sta", sta, izx, 6},
    instr! {"???", nop, imp, 2},
    instr! {"???", xxx, imp, 6},
    instr! {"sty", sty, zp0, 3},
    instr! {"sta", sta, zp0, 3},
    instr! {"stx", stx, zp0, 3},
    instr! {"???", xxx, imp, 3},
    instr! {"dey", dey, imp, 2},
    instr! {"???", nop, imp, 2},
    instr! {"txa", txa, imp, 2},
    instr! {"???", xxx, imp, 2},
    instr! {"sty", sty, abs, 4},
    instr! {"sta", sta, abs, 4},
    instr! {"stx", stx, abs, 4},
    instr! {"???", xxx, imp, 4},
    instr! {"bcc", bcc, rel, 2},
    instr! {"sta", sta, izy, 6},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 6},
    instr! {"sty", sty, zpx, 4},
    instr! {"sta", sta, zpx, 4},
    instr! {"stx", stx, zpy, 4},
    instr! {"???", xxx, imp, 4},
    instr! {"tya", tya, imp, 2},
    instr! {"sta", sta, aby, 5},
    instr! {"txs", txs, imp, 2},
    instr! {"???", xxx, imp, 5},
    instr! {"???", nop, imp, 5},
    instr! {"sta", sta, abx, 5},
    instr! {"???", xxx, imp, 5},
    instr! {"???", xxx, imp, 5},
    instr! {"ldy", ldy, imm, 2},
    instr! {"lda", lda, izx, 6},
    instr! {"ldx", ldx, imm, 2},
    instr! {"???", xxx, imp, 6},
    instr! {"ldy", ldy, zp0, 3},
    instr! {"lda", lda, zp0, 3},
    instr! {"ldx", ldx, zp0, 3},
    instr! {"???", xxx, imp, 3},
    instr! {"tay", tay, imp, 2},
    instr! {"lda", lda, imm, 2},
    instr! {"tax", tax, imp, 2},
    instr! {"???", xxx, imp, 2},
    instr! {"ldy", ldy, abs, 4},
    instr! {"lda", lda, abs, 4},
    instr! {"ldx", ldx, abs, 4},
    instr! {"???", xxx, imp, 4},
    instr! {"bcs", bcs, rel, 2},
    instr! {"lda", lda, izy, 5},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 5},
    instr! {"ldy", ldy, zpx, 4},
    instr! {"lda", lda, zpx, 4},
    instr! {"ldx", ldx, zpy, 4},
    instr! {"???", xxx, imp, 4},
    instr! {"clv", clv, imp, 2},
    instr! {"lda", lda, aby, 4},
    instr! {"tsx", tsx, imp, 2},
    instr! {"???", xxx, imp, 4},
    instr! {"ldy", ldy, abx, 4},
    instr! {"lda", lda, abx, 4},
    instr! {"ldx", ldx, aby, 4},
    instr! {"???", xxx, imp, 4},
    instr! {"cpy", cpy, imm, 2},
    instr! {"cmp", cmp, izx, 6},
    instr! {"???", nop, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"cpy", cpy, zp0, 3},
    instr! {"cmp", cmp, zp0, 3},
    instr! {"dec", dec, zp0, 5},
    instr! {"???", xxx, imp, 5},
    instr! {"iny", iny, imp, 2},
    instr! {"cmp", cmp, imm, 2},
    instr! {"dex", dex, imp, 2},
    instr! {"???", xxx, imp, 2},
    instr! {"cpy", cpy, abs, 4},
    instr! {"cmp", cmp, abs, 4},
    instr! {"dec", dec, abs, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"bne", bne, rel, 2},
    instr! {"cmp", cmp, izy, 5},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"???", nop, imp, 4},
    instr! {"cmp", cmp, zpx, 4},
    instr! {"dec", dec, zpx, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"cld", cld, imp, 2},
    instr! {"cmp", cmp, aby, 4},
    instr! {"nop", nop, imp, 2},
    instr! {"???", xxx, imp, 7},
    instr! {"???", nop, imp, 4},
    instr! {"cmp", cmp, abx, 4},
    instr! {"dec", dec, abx, 7},
    instr! {"???", xxx, imp, 7},
    instr! {"cpx", cpx, imm, 2},
    instr! {"sbc", sbc, izx, 6},
    instr! {"???", nop, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"cpx", cpx, zp0, 3},
    instr! {"sbc", sbc, zp0, 3},
    instr! {"inc", inc, zp0, 5},
    instr! {"???", xxx, imp, 5},
    instr! {"inx", inx, imp, 2},
    instr! {"sbc", sbc, imm, 2},
    instr! {"nop", nop, imp, 2},
    instr! {"???", sbc, imp, 2},
    instr! {"cpx", cpx, abs, 4},
    instr! {"sbc", sbc, abs, 4},
    instr! {"inc", inc, abs, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"beq", beq, rel, 2},
    instr! {"sbc", sbc, izy, 5},
    instr! {"???", xxx, imp, 2},
    instr! {"???", xxx, imp, 8},
    instr! {"???", nop, imp, 4},
    instr! {"sbc", sbc, zpx, 4},
    instr! {"inc", inc, zpx, 6},
    instr! {"???", xxx, imp, 6},
    instr! {"sed", sed, imp, 2},
    instr! {"sbc", sbc, aby, 4},
    instr! {"nop", nop, imp, 2},
    instr! {"???", xxx, imp, 7},
    instr! {"???", nop, imp, 4},
    instr! {"sbc", sbc, abx, 4},
    instr! {"inc", inc, abx, 7},
    instr! {"???", xxx, imp, 7},
];

// ##### FLAGS ####

enum StatusFlag {
    C = (1 << 0), // Carry Bit
    Z = (1 << 1), // Zero
    I = (1 << 2), // Disable Interrupts
    D = (1 << 3), // Decimal Mode
    B = (1 << 4), // Break
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}

impl<'a> Cpu<'a> {
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
}

// ##### CYCLES ####

pub fn cycle() {
    println!("6502 cycle");
    let dummy = &OPCODES[0];
    println!("{}", dummy.name);
}
