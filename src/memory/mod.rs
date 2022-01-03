#[cfg(test)]
mod tests;

use crate::address_bus::Addressing;

pub struct Memory {
    offset: u16,
    mem: Box<Vec<u8>>,
}

impl Memory {
    pub fn new(offset: u16, size: usize) -> Memory {
        Memory {
            offset: offset,
            mem: Box::new(vec![0u8; size]),
        }
    }
    pub fn from_vec(offset: u16, v: Vec<u8>) -> Memory {
        Memory {
            offset: offset,
            mem: Box::new(v),
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
