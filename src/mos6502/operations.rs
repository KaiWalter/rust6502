// ##### OPERATIONS ####
use crate::mos6502::*;

fn fetch(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    match address_mode_values.result {
        AddressModeResult::Absolute => {
            match cpu.address_bus.read(address_mode_values.absolute_address) {
                Ok(fetched) => fetched,
                Err(_e) => panic!(
                    "addressing error {:X}",
                    address_mode_values.absolute_address
                ),
            }
        }
        AddressModeResult::Fetched => address_mode_values.fetched_value,
        AddressModeResult::Relative => panic!("it is not intended to fetch relative address"),
    }
}

fn absolute_sp(cpu: &Cpu) -> u16 {
    0x0100 + (cpu.r.sp as u16)
}

// ----------------------------------------------------------------------------

pub fn adc(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values) as u16;

    let carry = if cpu.get_flag(StatusFlag::C) {
        1u16
    } else {
        0u16
    };

    if cpu.get_flag(StatusFlag::D) {
        let mut temp_bcd = (cpu.r.a as u16 & 0x0F) + (fetched & 0x0F) + carry;
        if temp_bcd > 9 {
            temp_bcd += 6;
        }

        if temp_bcd < 0x0F {
            temp_bcd = (temp_bcd & 0x0F) + (cpu.r.a as u16 & 0xF0) + (fetched & 0xF0);
        } else {
            temp_bcd = (temp_bcd & 0x0F) + (cpu.r.a as u16 & 0xF0) + (fetched & 0xF0) + 0x10;
        }

        cpu.set_flag(
            StatusFlag::Z,
            (cpu.r.a as u16 + fetched + carry) & 0xFF == 0,
        );
        cpu.set_flag(StatusFlag::N, temp_bcd & 0x80 != 0);
        cpu.set_flag(
            StatusFlag::V,
            (cpu.r.a as u16 ^ fetched) & 0x0080 != 0 && (cpu.r.a as u16 ^ temp_bcd) & 0x0080 == 0,
        );

        if temp_bcd & 0x1f0 > 0x90 {
            temp_bcd += 0x60
        }

        cpu.set_flag(StatusFlag::C, temp_bcd > 0xF0);
        cpu.r.a = (temp_bcd & 0x00FF) as u8;
    } else {
        let temp_bin = cpu.r.a as u16 + fetched + carry;

        cpu.set_flag(StatusFlag::N, temp_bin & 0x80 != 0);
        cpu.set_flag(StatusFlag::Z, temp_bin & 0xFF == 0);
        cpu.set_flag(
            StatusFlag::V,
            ((!(cpu.r.a as u16 ^ fetched)) & (cpu.r.a as u16 ^ temp_bin)) & 0x0080 != 0,
        );
        cpu.set_flag(StatusFlag::C, temp_bin > 0xFF);
        cpu.r.a = (temp_bin & 0x00FF) as u8;
    }

    1
}

pub fn and(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values);

    cpu.r.a &= fetched;

    cpu.set_flag(StatusFlag::Z, cpu.r.a == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.a & 0x80 != 0);

    0
}

pub fn asl(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values) as u16;

    let temp = fetched << 1;

    cpu.set_flag(StatusFlag::C, temp & 0xFF00 > 0);
    cpu.set_flag(StatusFlag::Z, temp & 0xFF == 0);
    cpu.set_flag(StatusFlag::N, temp & 0x80 != 0);

    match address_mode_values.result {
        AddressModeResult::Absolute => match cpu
            .address_bus
            .write(address_mode_values.absolute_address, (temp & 0xFF) as u8)
        {
            Ok(_) => 0,
            Err(_e) => panic!(
                "addressing error {:X}",
                address_mode_values.absolute_address
            ),
        },
        AddressModeResult::Fetched => {
            cpu.r.a = (temp & 0xFF) as u8;
            0
        }
        AddressModeResult::Relative => panic!("it is not intended to fetch relative address"),
    }
}

pub fn bcc(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let mut cycles = 0u8;

    if !cpu.get_flag(StatusFlag::C) {
        cycles += 1;
        let abs_addr = cpu.r.pc.wrapping_add(address_mode_values.relative_address);
        if abs_addr & 0xFF00 != cpu.r.pc & 0xFF00 {
            cycles += 1;
        }
        cpu.r.pc = abs_addr;
    }

    cycles
}

pub fn bcs(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let mut cycles = 0u8;

    if cpu.get_flag(StatusFlag::C) {
        cycles += 1;
        let abs_addr = cpu.r.pc.wrapping_add(address_mode_values.relative_address);
        if abs_addr & 0xFF00 != cpu.r.pc & 0xFF00 {
            cycles += 1;
        }
        cpu.r.pc = abs_addr;
    }

    cycles
}

pub fn beq(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let mut cycles = 0u8;

    if cpu.get_flag(StatusFlag::Z) {
        cycles += 1;
        let abs_addr = cpu.r.pc.wrapping_add(address_mode_values.relative_address);
        if abs_addr & 0xFF00 != cpu.r.pc & 0xFF00 {
            cycles += 1;
        }
        cpu.r.pc = abs_addr;
    }

    cycles
}

pub fn bit(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values);
    let temp = cpu.r.a & fetched;

    cpu.set_flag(StatusFlag::Z, temp & 0xFF == 0);
    cpu.set_flag(StatusFlag::N, fetched & (1 << 7) != 0);
    cpu.set_flag(StatusFlag::V, fetched & (1 << 6) != 0);

    0
}

pub fn bmi(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let mut cycles = 0u8;

    if cpu.get_flag(StatusFlag::N) {
        cycles += 1;
        let abs_addr = cpu.r.pc.wrapping_add(address_mode_values.relative_address);
        if abs_addr & 0xFF00 != cpu.r.pc & 0xFF00 {
            cycles += 1;
        }
        cpu.r.pc = abs_addr;
    }

    cycles
}

pub fn bne(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let mut cycles = 0u8;

    if !cpu.get_flag(StatusFlag::Z) {
        cycles += 1;
        let abs_addr = cpu.r.pc.wrapping_add(address_mode_values.relative_address);
        if abs_addr & 0xFF00 != cpu.r.pc & 0xFF00 {
            cycles += 1;
        }
        cpu.r.pc = abs_addr;
    }

    cycles
}

pub fn bpl(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let mut cycles = 0u8;

    if !cpu.get_flag(StatusFlag::N) {
        cycles += 1;
        let abs_addr = cpu.r.pc.wrapping_add(address_mode_values.relative_address);
        if abs_addr & 0xFF00 != cpu.r.pc & 0xFF00 {
            cycles += 1;
        }
        cpu.r.pc = abs_addr;
    }

    cycles
}

pub fn brk(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    match cpu
        .address_bus
        .write(absolute_sp(cpu), ((cpu.r.pc >> 8) & 0x00FF) as u8)
    {
        Ok(_) => {
            cpu.r.sp = cpu.r.sp.wrapping_sub(1);
            match cpu
                .address_bus
                .write(absolute_sp(cpu), (cpu.r.pc & 0x00FF) as u8)
            {
                Ok(_) => {
                    cpu.r.sp = cpu.r.sp.wrapping_sub(1);
                    cpu.set_flag(StatusFlag::B, true);
                    match cpu.address_bus.write(absolute_sp(cpu), cpu.r.status) {
                        Ok(_) => {
                            cpu.r.sp = cpu.r.sp.wrapping_sub(1);
                            cpu.set_flag(StatusFlag::I, true);
                            cpu.set_flag(StatusFlag::B, false);
                            match cpu.address_bus.read(0xFFFF) {
                                Ok(hi) => {
                                    cpu.r.pc = (hi as u16) << 8;
                                    match cpu.address_bus.read(0xFFFE) {
                                        Ok(lo) => {
                                            cpu.r.pc |= lo as u16;
                                            0
                                        }
                                        Err(e) => panic!("addressing error {}", e),
                                    }
                                }
                                Err(e) => panic!("addressing error {}", e),
                            }
                        }
                        Err(e) => panic!("addressing error {}", e),
                    }
                }
                Err(e) => panic!("addressing error {}", e),
            }
        }
        Err(e) => panic!("addressing error {}", e),
    };
    0
}

pub fn bvc(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let mut cycles = 0u8;

    if !cpu.get_flag(StatusFlag::V) {
        cycles += 1;
        let abs_addr = cpu.r.pc.wrapping_add(address_mode_values.relative_address);
        if abs_addr & 0xFF00 != cpu.r.pc & 0xFF00 {
            cycles += 1;
        }
        cpu.r.pc = abs_addr;
    }

    cycles
}
pub fn bvs(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let mut cycles = 0u8;

    if cpu.get_flag(StatusFlag::V) {
        cycles += 1;
        let abs_addr = cpu.r.pc.wrapping_add(address_mode_values.relative_address);
        if abs_addr & 0xFF00 != cpu.r.pc & 0xFF00 {
            cycles += 1;
        }
        cpu.r.pc = abs_addr;
    }

    cycles
}

pub fn clc(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.set_flag(StatusFlag::C, false);
    0
}

pub fn cld(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.set_flag(StatusFlag::D, false);
    0
}

pub fn cli(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.set_flag(StatusFlag::I, false);
    0
}

pub fn clv(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.set_flag(StatusFlag::V, false);
    0
}

fn compare_with_register(
    cpu: &mut Cpu,
    address_mode_values: AddressModeValues,
    register: u8,
) -> u8 {
    let fetched = fetch(cpu, address_mode_values);
    let temp = register.wrapping_sub(fetched);
    cpu.set_flag(StatusFlag::C, register >= fetched);
    cpu.set_flag(StatusFlag::Z, temp & 0xFF == 0);
    cpu.set_flag(StatusFlag::N, temp & 0x80 != 0);
    1
}

pub fn cmp(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    compare_with_register(cpu, address_mode_values, cpu.r.a)
}

pub fn cpx(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    compare_with_register(cpu, address_mode_values, cpu.r.x)
}

pub fn cpy(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    compare_with_register(cpu, address_mode_values, cpu.r.y)
}

pub fn dec(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values);
    let temp = fetched.wrapping_sub(1);
    match cpu
        .address_bus
        .write(address_mode_values.absolute_address, (temp & 0xFF) as u8)
    {
        Ok(_) => {
            cpu.set_flag(StatusFlag::Z, temp & 0xFF == 0);
            cpu.set_flag(StatusFlag::N, temp & 0x80 != 0);
        }
        Err(e) => panic!("addressing error {}", e),
    }
    0
}

pub fn dex(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.x = cpu.r.x.wrapping_sub(1);
    cpu.set_flag(StatusFlag::Z, cpu.r.x == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.x & 0x80 != 0);

    0
}

pub fn dey(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.y = cpu.r.y.wrapping_sub(1);
    cpu.set_flag(StatusFlag::Z, cpu.r.y == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.y & 0x80 != 0);

    0
}

pub fn eor(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values);

    cpu.r.a ^= fetched;

    cpu.set_flag(StatusFlag::Z, cpu.r.a == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.a & 0x80 != 0);

    0
}

pub fn inc(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values) as u16;
    let temp = fetched + 1;
    match cpu
        .address_bus
        .write(address_mode_values.absolute_address, (temp & 0x00FF) as u8)
    {
        Ok(_) => {
            cpu.set_flag(StatusFlag::Z, temp & 0x00FF == 0);
            cpu.set_flag(StatusFlag::N, temp & 0x0080 != 0);
        }
        Err(e) => panic!("addressing error {}", e),
    }
    0
}

pub fn inx(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.x = cpu.r.x.wrapping_add(1);
    cpu.set_flag(StatusFlag::Z, cpu.r.x == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.x & 0x80 != 0);

    0
}

pub fn iny(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.y = cpu.r.y.wrapping_add(1);
    cpu.set_flag(StatusFlag::Z, cpu.r.y == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.y & 0x80 != 0);

    0
}

pub fn jmp(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.pc = address_mode_values.absolute_address;
    0
}

pub fn jsr(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.pc -= 1;

    match cpu
        .address_bus
        .write(absolute_sp(cpu), ((cpu.r.pc >> 8) & 0x00FF) as u8)
    {
        Ok(_) => {
            cpu.r.sp = cpu.r.sp.wrapping_sub(1);
            match cpu
                .address_bus
                .write(absolute_sp(cpu), (cpu.r.pc & 0x00FF) as u8)
            {
                Ok(_) => {
                    cpu.r.sp = cpu.r.sp.wrapping_sub(1);
                    cpu.r.pc = address_mode_values.absolute_address;
                }
                Err(e) => panic!("addressing error {}", e),
            }
        }
        Err(e) => panic!("addressing error {}", e),
    };

    0
}

pub fn lda(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.a = fetch(cpu, address_mode_values);
    cpu.set_flag(StatusFlag::Z, cpu.r.a == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.a & 0x80 != 0);

    0
}

pub fn ldx(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.x = fetch(cpu, address_mode_values);
    cpu.set_flag(StatusFlag::Z, cpu.r.x == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.x & 0x80 != 0);

    0
}

pub fn ldy(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.y = fetch(cpu, address_mode_values);
    cpu.set_flag(StatusFlag::Z, cpu.r.y == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.y & 0x80 != 0);

    0
}

pub fn lsr(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values);

    cpu.set_flag(StatusFlag::C, fetched & 0x01 != 0);
    let temp = fetched as u16 >> 1;

    cpu.set_flag(StatusFlag::Z, temp & 0x00FF == 0);
    cpu.set_flag(StatusFlag::N, temp & 0x0080 != 0);

    match address_mode_values.result {
        AddressModeResult::Absolute => {
            match cpu
                .address_bus
                .write(address_mode_values.absolute_address, (temp & 0x00FF) as u8)
            {
                Ok(_) => {}
                Err(e) => panic!("addressing error {}", e),
            }
        }
        AddressModeResult::Fetched => {
            cpu.r.a = (temp & 0xFF) as u8;
        }
        AddressModeResult::Relative => panic!("it is not intended to fetch relative address"),
    }
    0
}

pub fn nop(_cpu: &mut Cpu, _address_mode_values: AddressModeValues, opcode: u8) -> u8 {
    // TODO: check in other emulators, these codes are not mapped to this opcode
    match opcode {
        0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1,
        _ => 0,
    }
}

pub fn ora(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values);
    cpu.r.a |= fetched;
    cpu.set_flag(StatusFlag::Z, cpu.r.a == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.a & 0x80 != 0);
    0
}

pub fn pha(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    match cpu.address_bus.write(absolute_sp(cpu), cpu.r.a) {
        Ok(_) => {
            cpu.r.sp = cpu.r.sp.wrapping_sub(1);
        }
        Err(e) => panic!("addressing error {}", e),
    }
    0
}

pub fn php(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    match cpu.address_bus.write(
        absolute_sp(cpu),
        cpu.r.status | StatusFlag::B as u8 | StatusFlag::U as u8,
    ) {
        Ok(_) => {
            cpu.set_flag(StatusFlag::B, false);
            cpu.r.sp = cpu.r.sp.wrapping_sub(1);
        }
        Err(e) => panic!("addressing error {}", e),
    }
    0
}

pub fn pla(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.sp = cpu.r.sp.wrapping_add(1);
    match cpu.address_bus.read(absolute_sp(cpu)) {
        Ok(value) => {
            cpu.r.a = value;
            cpu.set_flag(StatusFlag::Z, cpu.r.a == 0);
            cpu.set_flag(StatusFlag::N, cpu.r.a & 0x80 != 0);
        }
        Err(e) => panic!("addressing error {}", e),
    }
    0
}

pub fn plp(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.sp = cpu.r.sp.wrapping_add(1);
    match cpu.address_bus.read(absolute_sp(cpu)) {
        Ok(value) => {
            cpu.r.status = value;
            cpu.set_flag(StatusFlag::U, true);
        }
        Err(e) => panic!("addressing error {}", e),
    }
    0
}

pub fn rol(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values);
    let carry = if cpu.get_flag(StatusFlag::C) { 1 } else { 0 };
    let temp = (fetched as u16) << 1 | carry;
    cpu.set_flag(StatusFlag::C, temp & 0xFF00 != 0);
    cpu.set_flag(StatusFlag::Z, temp & 0x00FF == 0);
    cpu.set_flag(StatusFlag::N, temp & 0x0080 != 0);

    match address_mode_values.result {
        AddressModeResult::Absolute => {
            match cpu
                .address_bus
                .write(address_mode_values.absolute_address, (temp & 0x00FF) as u8)
            {
                Ok(_) => {}
                Err(e) => panic!("addressing error {}", e),
            }
        }
        AddressModeResult::Fetched => {
            cpu.r.a = (temp & 0xFF) as u8;
        }
        AddressModeResult::Relative => panic!("it is not intended to fetch relative address"),
    }
    0
}

pub fn ror(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values);
    let carry = if cpu.get_flag(StatusFlag::C) { 1 } else { 0 };
    let temp = (fetched as u16) >> 1 | carry << 7;
    cpu.set_flag(StatusFlag::C, fetched & 0x01 != 0);
    cpu.set_flag(StatusFlag::Z, temp & 0x00FF == 0);
    cpu.set_flag(StatusFlag::N, temp & 0x0080 != 0);

    match address_mode_values.result {
        AddressModeResult::Absolute => {
            match cpu
                .address_bus
                .write(address_mode_values.absolute_address, (temp & 0x00FF) as u8)
            {
                Ok(_) => {}
                Err(e) => panic!("addressing error {}", e),
            }
        }
        AddressModeResult::Fetched => {
            cpu.r.a = (temp & 0xFF) as u8;
        }
        AddressModeResult::Relative => panic!("it is not intended to fetch relative address"),
    }
    0
}

pub fn rti(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.sp = cpu.r.sp.wrapping_add(1);
    match cpu.address_bus.read(absolute_sp(cpu)) {
        Ok(value) => {
            cpu.r.status = value;
            cpu.set_flag(StatusFlag::B, false);
            cpu.set_flag(StatusFlag::U, false);
            cpu.r.sp = cpu.r.sp.wrapping_add(1);
            match cpu.address_bus.read(absolute_sp(cpu)) {
                Ok(lo) => {
                    cpu.r.pc = lo as u16;
                    cpu.r.sp = cpu.r.sp.wrapping_add(1);
                    match cpu.address_bus.read(absolute_sp(cpu)) {
                        Ok(hi) => {
                            cpu.r.pc |= (hi as u16) << 8;
                        }
                        Err(e) => panic!("addressing error {}", e),
                    }
                }
                Err(e) => panic!("addressing error {}", e),
            }
        }
        Err(e) => panic!("addressing error {}", e),
    }
    0
}

pub fn rts(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.sp = cpu.r.sp.wrapping_add(1);
    match cpu.address_bus.read(absolute_sp(cpu)) {
        Ok(lo) => {
            cpu.r.pc = lo as u16;
            cpu.r.sp = cpu.r.sp.wrapping_add(1);
            match cpu.address_bus.read(absolute_sp(cpu)) {
                Ok(hi) => {
                    cpu.r.pc |= (hi as u16) << 8;
                    cpu.r.pc += 1;
                }
                Err(e) => panic!("addressing error {}", e),
            }
        }
        Err(e) => panic!("addressing error {}", e),
    }
    0
}

pub fn sbc(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    let fetched = fetch(cpu, address_mode_values) as u16;

    let carry = if cpu.get_flag(StatusFlag::C) {
        1u16
    } else {
        0u16
    };

    let temp_bin = (cpu.r.a as u16)
        .wrapping_sub(fetched)
        .wrapping_sub(1 - carry);

    cpu.set_flag(
        StatusFlag::V,
        (cpu.r.a as u16 ^ fetched) & (cpu.r.a as u16 ^ temp_bin) & 0x80 != 0,
    );

    if cpu.get_flag(StatusFlag::D) {
        let mut temp_bcd = (cpu.r.a as u16 & 0x0F)
            .wrapping_sub(fetched & 0x0F)
            .wrapping_sub(1 - carry);
        if temp_bcd & 0x0010 != 0 {
            temp_bcd = (((temp_bcd.wrapping_sub(0x06)) & 0x0F) | (cpu.r.a as u16 & 0x0F0))
                .wrapping_sub(fetched & 0x0F0)
                .wrapping_sub(0x10);
        } else {
            temp_bcd = ((temp_bcd & 0x0F) | (cpu.r.a as u16 & 0x0F0)).wrapping_sub(fetched & 0x0F0);
        }

        if temp_bcd & 0x0100 != 0 {
            temp_bcd = temp_bcd.wrapping_sub(0x60);
        }

        cpu.r.a = (temp_bcd & 0xFF) as u8;
    } else {
        cpu.r.a = (temp_bin & 0xFF) as u8;
    }

    cpu.set_flag(StatusFlag::C, temp_bin < 0x0100);
    cpu.set_flag(StatusFlag::Z, temp_bin & 0xFF == 0);
    cpu.set_flag(StatusFlag::N, temp_bin & 0x80 != 0);

    1
}

pub fn sec(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.set_flag(StatusFlag::C, true);
    0
}

pub fn sed(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.set_flag(StatusFlag::D, true);
    0
}

pub fn sei(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.set_flag(StatusFlag::I, true);
    0
}

pub fn sta(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    match cpu
        .address_bus
        .write(address_mode_values.absolute_address, cpu.r.a)
    {
        Ok(()) => 0,
        Err(err) => panic!("{}", err),
    }
}

pub fn stx(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    match cpu
        .address_bus
        .write(address_mode_values.absolute_address, cpu.r.x)
    {
        Ok(()) => 0,
        Err(err) => panic!("{}", err),
    }
}

pub fn sty(cpu: &mut Cpu, address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    match cpu
        .address_bus
        .write(address_mode_values.absolute_address, cpu.r.y)
    {
        Ok(()) => 0,
        Err(err) => panic!("{}", err),
    }
}

pub fn tax(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.x = cpu.r.a;
    cpu.set_flag(StatusFlag::Z, cpu.r.x == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.x & 0x80 != 0);
    0
}

pub fn tay(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.y = cpu.r.a;
    cpu.set_flag(StatusFlag::Z, cpu.r.y == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.y & 0x80 != 0);
    0
}

pub fn tsx(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.x = cpu.r.sp;
    cpu.set_flag(StatusFlag::Z, cpu.r.x == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.x & 0x80 != 0);
    0
}
pub fn txa(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.a = cpu.r.x;
    cpu.set_flag(StatusFlag::Z, cpu.r.a == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.a & 0x80 != 0);
    0
}

pub fn txs(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.sp = cpu.r.x;
    0
}
pub fn tya(cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    cpu.r.a = cpu.r.y;
    cpu.set_flag(StatusFlag::Z, cpu.r.a == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.a & 0x80 != 0);
    0
}

pub fn xxx(_cpu: &mut Cpu, _address_mode_values: AddressModeValues, _opcode: u8) -> u8 {
    0
}
