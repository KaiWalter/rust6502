// ##### OPERATIONS ####
use crate::mos6502::*;

fn fetch(cpu: &Cpu, address_mode_values: AddressModeValues) -> u8 {
    match address_mode_values.result {
        AddressModeResult::absolute => {
            match cpu.address_bus.read(address_mode_values.absolute_address) {
                Ok(fetched) => fetched,
                Err(_e) => panic!(
                    "addressing error {:X}",
                    address_mode_values.absolute_address
                ),
            }
        }
        AddressModeResult::fetched => address_mode_values.fetched_value,
        AddressModeResult::relative => panic!("it is not intended to fetch relative address"),
    }
}

pub fn adc(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn and(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn asl(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn bcc(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn bcs(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn beq(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn bit(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn bmi(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn bne(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn bpl(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn brk(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn bvc(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn bvs(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn clc(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn cld(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn cli(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn clv(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn cmp(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn cpx(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn cpy(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn dec(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn dex(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn dey(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn eor(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn inc(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn inx(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn iny(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn jmp(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn jsr(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}

pub fn lda(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    cpu.r.a = fetch(cpu, address_mode_values);
    cpu.set_flag(StatusFlag::Z, cpu.r.a == 0);
    cpu.set_flag(StatusFlag::N, cpu.r.a & 0x80 != 0);

    0
}

pub fn ldx(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn ldy(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn lsr(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn nop(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn ora(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn pha(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn php(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn pla(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn plp(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn rol(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn ror(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn rti(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn rts(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn sbc(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn sec(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn sed(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn sei(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn sta(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn stx(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn sty(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn tax(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn tay(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn tsx(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn txa(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn txs(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn tya(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
pub fn xxx(cpu: &mut Cpu, address_mode_values: AddressModeValues) -> u8 {
    0
}
