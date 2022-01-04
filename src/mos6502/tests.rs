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
fn test_address_mode_abx() {
    // arrange
    let expected: u16 = 0x0304;
    let mut mem = Memory::from_vec(0, vec![0x01, 0x02, 0x03, 0x00]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu {
        r: CpuRegisters {
            a: 0,
            x: 2,
            y: 0,
            pc: 1,
            sp: 0,
            status: 0,
        },
        address_bus: address_bus,
    };

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = abx(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap().absolute_address);
    assert_eq!(cpu_r_before.pc + 2, cpu.r.pc);
}

#[test]
fn test_address_mode_abx_cross_page() {
    // arrange
    let expected: u16 = 0x0401;
    let mut mem = Memory::from_vec(0, vec![0x01, 0xFE, 0x03, 0x00]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu {
        r: CpuRegisters {
            a: 0,
            x: 3,
            y: 0,
            pc: 1,
            sp: 0,
            status: 0,
        },
        address_bus: address_bus,
    };

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = abx(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(1, a.add_cycles);
    assert_eq!(cpu_r_before.pc + 2, cpu.r.pc);
}

#[test]
fn test_address_mode_aby() {
    // arrange
    let expected: u16 = 0x0305;
    let mut mem = Memory::from_vec(0, vec![0x01, 0x02, 0x03, 0x00]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu {
        r: CpuRegisters {
            a: 0,
            x: 0,
            y: 3,
            pc: 1,
            sp: 0,
            status: 0,
        },
        address_bus: address_bus,
    };

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = aby(&mut cpu);

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
fn test_address_mode_ind() {
    // arrange
    let expected: u16 = 0x0201;
    let mut mem = Memory::from_vec(0, vec![0x00, 0x01, 0x02, 0x01, 0x00]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu {
        r: CpuRegisters {
            a: 0,
            x: 0,
            y: 0,
            pc: 3,
            sp: 0,
            status: 0,
        },
        address_bus: address_bus,
    };

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = ind(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap().absolute_address);
    assert_eq!(cpu_r_before.pc + 2, cpu.r.pc);
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

#[test]
fn test_address_mode_izx() {
    // arrange
    let expected: u16 = 0x0201;
    let mut mem = Memory::from_vec(0, vec![0, 0x05, 0, 0, 0, 0, 0x01, 0x02]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu {
        r: CpuRegisters {
            a: 0,
            x: 1,
            y: 0,
            pc: 1,
            sp: 0,
            status: 0,
        },
        address_bus: address_bus,
    };

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = izx(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap().absolute_address);
    assert_eq!(cpu_r_before.pc + 1, cpu.r.pc);
}

#[test]
fn test_address_mode_izy() {
    // arrange
    let expected: u16 = 0x0203;
    let mut mem = Memory::from_vec(0, vec![0, 0x06, 0, 0, 0, 0, 0x01, 0x02]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu {
        r: CpuRegisters {
            a: 0,
            x: 0,
            y: 2,
            pc: 1,
            sp: 0,
            status: 0,
        },
        address_bus: address_bus,
    };

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = izy(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    assert_eq!(expected, actual.unwrap().absolute_address);
    assert_eq!(cpu_r_before.pc + 1, cpu.r.pc);
}
