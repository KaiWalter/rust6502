use rust6502::address_bus::*;
use rust6502::mc6821::*;
use rust6502::memory::*;
use rust6502::mos6502::*;

use crate::wasm_terminal::WasmTerminal;

pub struct Apple1Compact<'a> {
    pub cpu: Option<Cpu<'a>>,
    pub bus: Option<Apple1CompactBus>,
    pub terminal: Option<WasmTerminal>,
    pub check_input: Option<Box<dyn Fn()>>,
}

pub struct Apple1CompactBus {
    pub mem: Option<Memory>,
    pub rom_monitor: Option<Memory>,
    pub pia: Option<MC6821>,
}

impl ExternalAddressing for Apple1CompactBus {
    fn read(&mut self, addr: u16) -> Result<u8, AddressingError> {
        match addr {
            0x0000..=0x1000 => Ok(self.mem.as_mut().unwrap().int_read(addr)),
            0x1001..=0xCFFF => Err(AddressingError::new("read", addr)),
            0xD000..=0xD1FF => Ok(self.pia.as_mut().unwrap().int_read(addr)),
            0xD200..=0xFEFF => Err(AddressingError::new("read", addr)),
            0xFF00..=0xFFFF => Ok(self.rom_monitor.as_mut().unwrap().int_read(addr)),
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), AddressingError> {
        match addr {
            0x0000..=0x1000 => Ok(self.mem.as_mut().unwrap().int_write(addr, data)),
            0x1001..=0xCFFF => Err(AddressingError::new("read", addr)),
            0xD000..=0xD1FF => Ok(self.pia.as_mut().unwrap().int_write(addr, data)),
            0xD200..=0xFFFF => Err(AddressingError::new("write", addr)),
        }
    }
}
