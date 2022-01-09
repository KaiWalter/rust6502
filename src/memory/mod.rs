#[cfg(test)]
mod tests;

use std::fs;

use crate::address_bus::{AddressingError, ExternalAddressing, InternalAddressing};

pub struct Memory {
    offset: u16,
    mem: Vec<u8>,
}

#[allow(dead_code)]
impl Memory {
    pub fn new(offset: u16, size: usize) -> Memory {
        Memory {
            offset: offset,
            mem: vec![0u8; size],
        }
    }

    pub fn from_vec(offset: u16, v: Vec<u8>) -> Memory {
        Memory {
            offset: offset,
            mem: v,
        }
    }

    pub fn load_rom(offset: u16, filename: String) -> Memory {
        let data = fs::read(filename).expect("could not read file");
        Memory {
            offset: offset,
            mem: data,
        }
    }

    pub fn fill(&mut self, size: usize, value: u8) {
        while self.mem.len() < size {
            self.mem.push(value);
        }
    }
}

impl InternalAddressing for Memory {
    fn int_read(&mut self, addr: u16) -> u8 {
        self.mem[(addr - self.offset) as usize]
    }

    fn int_write(&mut self, addr: u16, data: u8) {
        self.mem[(addr - self.offset) as usize] = data;
    }

    fn len(&self) -> usize {
        self.mem.len() as usize
    }
}

impl ExternalAddressing for Memory {
    fn read(&mut self, addr: u16) -> Result<u8, AddressingError> {
        if addr < self.offset || (addr - self.offset) as usize >= self.mem.len() {
            Err(AddressingError::new("read", addr))
        } else {
            Ok(self.mem[(addr - self.offset) as usize])
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), AddressingError> {
        if addr < self.offset || (addr - self.offset) as usize >= self.mem.len() {
            Err(AddressingError::new("write", addr))
        } else {
            self.mem[(addr - self.offset) as usize] = data;
            Ok(())
        }
    }
}
