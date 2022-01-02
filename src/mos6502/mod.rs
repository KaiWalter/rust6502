use crate::address_bus::AddressBus;

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

struct AddressModeValues {
    absolute_address: u16,
    fetched_value: u8,
}

struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    status: u8,
    address_bus: AddressBus,
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

// ##### ADDRESS MODES ####

fn abs(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("ABS", cpu.pc);

    match cpu.address_bus.read(cpu.pc) {
        Ok(lo) => {
            cpu.pc += 1;
            match cpu.address_bus.read(cpu.pc) {
                Ok(hi) => {
                    cpu.pc += 1;
                    Ok(AddressModeValues {
                        absolute_address: (hi as u16) << 8 | lo as u16,
                        fetched_value: 0,
                    })
                }
                Err(e) => Err(cpu_error),
            }
        }
        Err(e) => Err(cpu_error),
    }
}

fn abx(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("ABX", cpu.pc);
    Err(cpu_error)
}

fn aby(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("ABY", cpu.pc);
    Err(cpu_error)
}

fn ind(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("IND", cpu.pc);
    Err(cpu_error)
}

fn imm(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let addr = cpu.pc;
    cpu.pc += 1;
    Ok(AddressModeValues {
        absolute_address: addr,
        fetched_value: 0,
    })
}

fn imp(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    Ok(AddressModeValues {
        absolute_address: 0,
        fetched_value: cpu.a,
    })
}

fn izx(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("IZX", cpu.pc);
    Err(cpu_error)
}

fn izy(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("IZY", cpu.pc);
    Err(cpu_error)
}

fn rel(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("REL", cpu.pc);
    Err(cpu_error)
}

fn zp0(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("ZP0", cpu.pc);
    Err(cpu_error)
}

fn zpx(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("ZPX", cpu.pc);
    Err(cpu_error)
}

fn zpy(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("ZPX", cpu.pc);
    Err(cpu_error)
}

// ##### OP CODES ####

fn adc(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn and(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn asl(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn bcc(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn bcs(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn beq(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn bit(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn bmi(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn bne(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn bpl(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn brk(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn bvc(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn bvs(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn clc(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn cld(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn cli(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn clv(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn cmp(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn cpx(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn cpy(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn dec(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn dex(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn dey(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn eor(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn inc(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn inx(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn iny(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn jmp(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn jsr(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn lda(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn ldx(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn ldy(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn lsr(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn nop(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn ora(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn pha(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn php(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn pla(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn plp(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn rol(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn ror(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn rti(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn rts(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn sbc(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn sec(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn sed(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn sei(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn sta(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn stx(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn sty(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn tax(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn tay(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn tsx(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn txa(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn txs(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn tya(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}
fn xxx(cpu: &mut Cpu, address_mode_values: AddressModeValues) {}

// ##### CYCLES ####

pub fn cycle() {
    println!("6502 cycle");
    let dummy = &OPCODES[0];
    println!("{}", dummy.name);
}
