#[cfg(test)]
use super::*;
use crate::address_bus::*;
use crate::memory::*;

#[test]
fn test_address_mode_abs() {
    // arrange
    let expected: u16 = 0x0302;
    let mut mem = Memory::from_vec(0, vec![0x01, 0x02, 0x03, 0x00]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu {
        r: CpuRegisters {
            a: 0,
            x: 0,
            y: 0,
            pc: 1,
            sp: 0,
            status: 0,
        },
        address_bus: address_bus,
    };

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = abs(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap().absolute_address);
    assert_eq!(cpu_r_before.pc + 2, cpu.r.pc);
}

#[test]
fn test_address_mode_abs_addr_error() {
    // arrange
    let mut mem = Memory::from_vec(0, vec![0x01, 0x02, 0x03, 0x00]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu {
        r: CpuRegisters {
            a: 0,
            x: 0,
            y: 0,
            pc: 0x100,
            sp: 0,
            status: 0,
        },
        address_bus: address_bus,
    };

    // act
    let actual = abs(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), false);
    assert_eq!(actual.is_err(), true);
}

#[test]
fn test_address_mode_imm() {
    // arrange
    let expected: u16 = 0x1234;
    let mut cpu = Cpu {
        r: CpuRegisters {
            a: 0,
            x: 0,
            y: 0,
            pc: expected,
            sp: 0,
            status: 0,
        },
        address_bus: AddressBus::new(0x0),
    };

    // act
    let actual = imm(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap().absolute_address);
    assert_eq!(expected + 1, cpu.r.pc);
}

#[test]
fn test_address_mode_imp() {
    // arrange
    let expected: u8 = 0x12;
    let mut cpu = Cpu {
        r: CpuRegisters {
            a: expected,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status: 0,
        },
        address_bus: AddressBus::new(0x0),
    };

    // act
    let actual = imp(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap().fetched_value);
}
