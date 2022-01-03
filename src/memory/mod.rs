#[cfg(test)]
mod tests;

use std::fs;

use crate::address_bus::Addressing;

pub struct Memory {
    offset: u16,
    mem: Vec<u8>,
}

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
}

impl Addressing for Memory {
    fn read(&self, addr: u16) -> u8 {
        self.mem[(addr - self.offset) as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.mem[(addr - self.offset) as usize] = data;
    }

    fn len(&self) -> u16 {
        self.mem.len() as u16
    }
}
