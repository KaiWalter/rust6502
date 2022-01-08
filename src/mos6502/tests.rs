// use std::fs::File;

#[cfg(test)]
use super::*;
use crate::address_bus::*;
use crate::memory::*;

// ##### ADDRESS MODES ####

#[test]
fn test_address_mode_abs() {
    // arrange
    let expected: u16 = 0x0302;
    let mut mem = Memory::from_vec(0, vec![0x01, 0x02, 0x03, 0x00]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 1;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = abs(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
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

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 1;
    cpu.r.x = 2;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = abx(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
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

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 1;
    cpu.r.x = 3;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = abx(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
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

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 1;
    cpu.r.y = 3;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = aby(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
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

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 0x100;

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

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 3;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = ind(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
    assert_eq!(cpu_r_before.pc + 2, cpu.r.pc);
}

#[test]
fn test_address_mode_imm() {
    // arrange
    let expected: u16 = 0x1234;
    let mut address_bus = AddressBus::new(0x1000);
    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = expected;

    // act
    let actual = imm(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
    assert_eq!(expected + 1, cpu.r.pc);
}

#[test]
fn test_address_mode_imp() {
    // arrange
    let expected: u8 = 0x12;
    let mut address_bus = AddressBus::new(0x1000);
    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.a = expected;

    // act
    let actual = imp(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.fetched_value);
    assert_eq!(AddressModeResult::Fetched, a.result);
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

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.x = 1;
    cpu.r.pc = 1;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = izx(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
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

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.y = 2;
    cpu.r.pc = 1;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = izy(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
    assert_eq!(cpu_r_before.pc + 1, cpu.r.pc);
}

#[test]
fn test_address_mode_rel() {
    // arrange
    let expected: u16 = 0x06;
    let mut mem = Memory::from_vec(0, vec![0, 0x06, 0, 0, 0, 0, 0x01, 0x02]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 1;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = rel(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.relative_address);
    assert_eq!(AddressModeResult::Relative, a.result);
    assert_eq!(cpu_r_before.pc + 1, cpu.r.pc);
}

#[test]
fn test_address_mode_zp0() {
    // arrange
    let expected: u16 = 0x06;
    let mut mem = Memory::from_vec(0, vec![0, 0x06, 0, 0, 0, 0, 0x01, 0x02]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 1;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = zp0(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
    assert_eq!(cpu_r_before.pc + 1, cpu.r.pc);
}

#[test]
fn test_address_mode_zpx() {
    // arrange
    let expected: u16 = 0x07;
    let mut mem = Memory::from_vec(0, vec![0, 0x06, 0, 0, 0, 0, 0x01, 0x02]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 1;
    cpu.r.x = 1;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = zpx(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
    assert_eq!(cpu_r_before.pc + 1, cpu.r.pc);
}

#[test]
fn test_address_mode_zpy() {
    // arrange
    let expected: u16 = 0x08;
    let mut mem = Memory::from_vec(0, vec![0, 0x06, 0, 0, 0, 0, 0x01, 0x02]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.pc = 1;
    cpu.r.y = 2;

    let cpu_r_before = cpu.r.clone();

    // act
    let actual = zpy(&mut cpu);

    // assert
    assert_eq!(actual.is_ok(), true);
    assert_eq!(actual.is_err(), false);
    let a = actual.unwrap();
    assert_eq!(expected, a.absolute_address);
    assert_eq!(AddressModeResult::Absolute, a.result);
    assert_eq!(cpu_r_before.pc + 1, cpu.r.pc);
}

// ##### FLAGS ####

#[test]
fn test_set_flags() {
    // arrange
    let expected: u8 = StatusFlag::D as u8 | StatusFlag::C as u8;
    let mut mem = Memory::from_vec(0, vec![0]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);

    // act
    cpu.set_flag(StatusFlag::D, true);
    cpu.set_flag(StatusFlag::C, true);

    // assert
    assert_eq!(expected, cpu.r.status);
    assert_eq!(true, cpu.get_flag(StatusFlag::D));
    assert_eq!(true, cpu.get_flag(StatusFlag::C));
    assert_eq!(false, cpu.get_flag(StatusFlag::Z));
}

#[test]
fn test_clear_flags() {
    // arrange
    let expected: u8 = 0xFF & !(StatusFlag::D as u8 | StatusFlag::C as u8);
    let mut mem = Memory::from_vec(0, vec![0]);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.r.status = 0xFF;

    // act
    cpu.set_flag(StatusFlag::D, false);
    cpu.set_flag(StatusFlag::C, false);

    // assert
    assert_eq!(expected, cpu.r.status);
    assert_eq!(false, cpu.get_flag(StatusFlag::D));
    assert_eq!(false, cpu.get_flag(StatusFlag::C));
    assert_eq!(true, cpu.get_flag(StatusFlag::Z));
}

// ##### OPERATIONS ####

#[test]
fn test_lda_imp() {
    // arrange
    let expected: u8 = 0x55;
    let mut mem = Memory::from_vec(0, vec![0xA9, 0x55]); // LDA #$55
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    let cpu_r_before = cpu.r.clone();

    // act
    cpu.cycle(false);

    // assert
    assert_eq!(expected, cpu.r.a);
    assert_eq!(cpu_r_before.pc + 2, cpu.r.pc);
}

#[test]
fn test_lda_zp0() {
    // arrange
    let expected: u8 = 0x55;
    let mut mem = Memory::from_vec(0, vec![0xA5, 0x02, 0x55]); // LDA $02
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    let cpu_r_before = cpu.r.clone();

    // act
    cpu.cycle(false);

    // assert
    assert_eq!(expected, cpu.r.a);
    assert_eq!(cpu_r_before.pc + 2, cpu.r.pc);
}

#[test]
fn test_ld_axy_st_axy() {
    // arrange
    let expected: u8 = 0x55;
    let program = vec![
        0xA9, 0x55, //                  LDA #$55
        0x85, 0x10, //                  STA $10
        0xA6, 0x10, //                  LDX $10
        0xE8, //                  INX
        0x86, 0x11, //                  STX $11
        0xA4, 0x11, //                  LDY $11
        0xC8, //                  INY
        0x84, 0x12, //                  STY $12
        0x00, //                  BRK
        0x00, //                  BRK
        0x00, //                  BRK
        0x00, //                  BRK
        0x00, //                  BRK
        0x00, //                  BRK
        0x00, //                  BRK
    ];

    let mut mem = Memory::from_vec(0, program);
    let mut address_bus = AddressBus::new(mem.len());
    if address_bus.add_component(0, mem.len(), &mut (mem)).is_err() {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);

    // act
    while cpu.r.pc < 0x0E {
        cpu.cycle(false);
    }

    // assert
    assert_eq!(expected, cpu.r.a);
    assert_eq!(expected + 1, cpu.r.x);
    assert_eq!(expected + 2, cpu.r.y);
    assert_eq!(expected, cpu.address_bus.read(0x10).unwrap());
    assert_eq!(expected + 1, cpu.address_bus.read(0x11).unwrap());
    assert_eq!(expected + 2, cpu.address_bus.read(0x12).unwrap());
}

// ##### TEST ROMS ####

#[test]
fn functional_test() {
    // arrange
    // let mut w = File::create("func-rust.txt").unwrap();

    const END_OF_FUNCTIONAL_TEST: u16 = 0x3469;
    let mut mem = Memory::load_rom(0, "./roms/6502_functional_test.bin".to_string());

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut mem);
    cpu.reset();
    cpu.r.pc = 0x0400;
    cpu.wait_for_system_reset_cycles();
    let mut prev_pc = cpu.r.pc;

    // act
    while cpu.current_pc != END_OF_FUNCTIONAL_TEST {
        // last error : 'infinite loop at 3489', src/mos6502/tests.rs:540:17
        if cpu.remaining_cycles == 0 {
            if prev_pc == cpu.current_pc {
                // w.sync_all()?;
                panic!("infinite loop at {:X}", cpu.current_pc);
            }
            prev_pc = cpu.current_pc;
        }
        // cpu.cycle_file(&mut w);
        cpu.cycle(false);
    }

    // w.flush().unwrap();

    // assert
    assert_eq!(END_OF_FUNCTIONAL_TEST, cpu.current_pc);
}

#[test]
fn decimal_test() {
    // arrange
    // let mut w = File::create("dec-rust.txt").unwrap();

    const END_OF_DECIMAL_TEST: u16 = 0x024B;
    const RESULT_ADDR_DECIMAL_TEST: u16 = 0x0B;
    const RESULT_DECIMAL_TEST: u8 = 0;

    let mut address_bus = AddressBus::new(0x100);

    let mut mem = Memory::from_vec(0, vec![0; 0x200]);
    if address_bus
        .add_component(0x0000, mem.len(), &mut (mem))
        .is_err()
    {
        panic!("add_component failed");
    }

    let mut mem_high = Memory::from_vec(0xFF00, vec![0; 256]);
    if address_bus
        .add_component(0xFF00, mem_high.len(), &mut (mem_high))
        .is_err()
    {
        panic!("add_component failed");
    }

    let mut rom = Memory::load_rom(0x200, "./roms/6502_decimal_test.bin".to_string());
    rom.fill(0x200, 0);
    if address_bus
        .add_component(0x0200, rom.len(), &mut (rom))
        .is_err()
    {
        panic!("add_component failed");
    }

    let mut cpu = Cpu::new(CpuRegisters::default(), &mut address_bus);
    cpu.reset();
    cpu.r.pc = 0x200;
    cpu.wait_for_system_reset_cycles();

    // act
    while cpu.r.pc != END_OF_DECIMAL_TEST {
        // cpu.cycle_file(&mut w);
        cpu.cycle(false);
    }

    // w.sync_all().unwrap();

    let actual = cpu.address_bus.read(RESULT_ADDR_DECIMAL_TEST).unwrap();

    // assert
    assert_eq!(
        RESULT_DECIMAL_TEST, actual,
        "expected at {:04x}: {:02x} actual: {:02x}",
        RESULT_ADDR_DECIMAL_TEST, RESULT_DECIMAL_TEST, actual
    );
}
