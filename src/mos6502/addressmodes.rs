// ##### ADDRESS MODES ####
use crate::mos6502::*;

fn from_pc_word(cpu: &mut Cpu, operation: &str, index: u8) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new(operation, cpu.r.pc);

    match cpu.address_bus.read(cpu.r.pc) {
        Ok(lo) => {
            cpu.r.pc += 1;
            match cpu.address_bus.read(cpu.r.pc) {
                Ok(hi) => {
                    cpu.r.pc += 1;
                    let abs_addr = ((hi as u16) << 8 | lo as u16) + index as u16;
                    let add_cycles = if index > 0 && ((abs_addr & 0xFF00) != ((hi as u16) << 8)) {
                        1 // additional cycle when cross-page boundary
                    } else {
                        0
                    };
                    Ok(AddressModeValues {
                        result: AddressModeResult::Absolute,
                        absolute_address: abs_addr,
                        relative_address: 0,
                        fetched_value: 0,
                        add_cycles: add_cycles,
                    })
                }
                Err(_e) => Err(cpu_error),
            }
        }
        Err(_e) => Err(cpu_error),
    }
}

fn from_pc_byte(cpu: &mut Cpu, operation: &str) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new(operation, cpu.r.pc);
    match cpu.address_bus.read(cpu.r.pc) {
        Ok(abs_addr) => {
            cpu.r.pc += 1;
            Ok(AddressModeValues {
                result: AddressModeResult::Absolute,
                absolute_address: abs_addr as u16,
                relative_address: 0,
                fetched_value: 0,
                add_cycles: 0,
            })
        }
        Err(_e) => Err(cpu_error),
    }
}

pub fn abs(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    return from_pc_word(cpu, "ABS", 0);
}

pub fn abx(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    return from_pc_word(cpu, "ABX", cpu.r.x);
}

pub fn aby(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    return from_pc_word(cpu, "ABY", cpu.r.y);
}

pub fn ind(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("IND", cpu.r.pc);

    match from_pc_word(cpu, "IND", 0) {
        Ok(pointer) => {
            let mut abs_addr = pointer.absolute_address;
            match cpu.address_bus.read(abs_addr) {
                Ok(lo) => {
                    if abs_addr & 0x00FF == 0x00FF {
                        abs_addr = abs_addr & 0xFF00
                    } else {
                        abs_addr += 1
                    }
                    match cpu.address_bus.read(abs_addr) {
                        Ok(hi) => Ok(AddressModeValues {
                            result: AddressModeResult::Absolute,
                            absolute_address: (hi as u16) << 8 | lo as u16,
                            relative_address: 0,
                            fetched_value: 0,
                            add_cycles: 0,
                        }),
                        Err(_e) => Err(cpu_error),
                    }
                }
                Err(_e) => Err(cpu_error),
            }
        }
        Err(cpu_error) => Err(cpu_error),
    }
}

pub fn imm(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let addr = cpu.r.pc;
    cpu.r.pc += 1;
    Ok(AddressModeValues {
        result: AddressModeResult::Absolute,
        absolute_address: addr,
        relative_address: 0,
        fetched_value: 0,
        add_cycles: 0,
    })
}

pub fn imp(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    Ok(AddressModeValues {
        result: AddressModeResult::Fetched,
        absolute_address: 0,
        relative_address: 0,
        fetched_value: cpu.r.a,
        add_cycles: 0,
    })
}

pub fn izx(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("IZX", cpu.r.pc);

    match from_pc_byte(cpu, "IZX") {
        Ok(result) => {
            let indexed_address = result.absolute_address as u16 + cpu.r.x as u16;
            match cpu.address_bus.read(indexed_address & 0x00FF) {
                Ok(lo) => match cpu.address_bus.read((indexed_address + 1) & 0x00FF) {
                    Ok(hi) => Ok(AddressModeValues {
                        result: AddressModeResult::Absolute,
                        absolute_address: (hi as u16) << 8 | lo as u16,
                        relative_address: 0,
                        fetched_value: 0,
                        add_cycles: 0,
                    }),
                    Err(_e) => Err(cpu_error),
                },
                Err(_e) => Err(cpu_error),
            }
        }
        Err(_e) => Err(cpu_error),
    }
}

pub fn izy(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("IZY", cpu.r.pc);

    match from_pc_byte(cpu, "IZY") {
        Ok(result) => {
            let indexed_address = result.absolute_address as u16;
            match cpu.address_bus.read(indexed_address & 0x00FF) {
                Ok(lo) => match cpu.address_bus.read((indexed_address + 1) & 0x00FF) {
                    Ok(hi) => {
                        let abs_addr = ((hi as u16) << 8 | lo as u16) + cpu.r.y as u16;
                        let add_cycles = if (abs_addr & 0xFF00) != ((hi as u16) << 8) {
                            1 // additional cycle when cross-page boundary
                        } else {
                            0
                        };
                        Ok(AddressModeValues {
                            result: AddressModeResult::Absolute,
                            absolute_address: abs_addr,
                            relative_address: 0,
                            fetched_value: 0,
                            add_cycles: add_cycles,
                        })
                    }
                    Err(_e) => Err(cpu_error),
                },
                Err(_e) => Err(cpu_error),
            }
        }
        Err(_e) => Err(cpu_error),
    }
}

pub fn rel(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("REL", cpu.r.pc);
    match from_pc_byte(cpu, "REL") {
        Ok(result) => {
            let mut rel_address = result.absolute_address as u16;
            if rel_address & 0x80 != 0 {
                rel_address |= 0xFF00
            }
            Ok(AddressModeValues {
                result: AddressModeResult::Relative,
                absolute_address: 0,
                relative_address: rel_address,
                fetched_value: 0,
                add_cycles: 0,
            })
        }
        Err(_e) => Err(cpu_error),
    }
}

pub fn zp0(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("ZP0", cpu.r.pc);
    match from_pc_byte(cpu, "ZP0") {
        Ok(result) => Ok(AddressModeValues {
            result: AddressModeResult::Absolute,
            absolute_address: result.absolute_address as u16 & 0x00FF,
            relative_address: 0,
            fetched_value: 0,
            add_cycles: 0,
        }),
        Err(_e) => Err(cpu_error),
    }
}

pub fn zpx(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("ZPX", cpu.r.pc);
    match from_pc_byte(cpu, "ZPX") {
        Ok(result) => Ok(AddressModeValues {
            result: AddressModeResult::Absolute,
            absolute_address: (result.absolute_address as u16 & 0x00FF) + cpu.r.x as u16,
            relative_address: 0,
            fetched_value: 0,
            add_cycles: 0,
        }),
        Err(_e) => Err(cpu_error),
    }
}

pub fn zpy(cpu: &mut Cpu) -> Result<AddressModeValues, CpuError> {
    let cpu_error = CpuError::new("ZPY", cpu.r.pc);
    match from_pc_byte(cpu, "ZPY") {
        Ok(result) => Ok(AddressModeValues {
            result: AddressModeResult::Absolute,
            absolute_address: (result.absolute_address as u16 & 0x00FF) + cpu.r.y as u16,
            relative_address: 0,
            fetched_value: 0,
            add_cycles: 0,
        }),
        Err(_e) => Err(cpu_error),
    }
}
