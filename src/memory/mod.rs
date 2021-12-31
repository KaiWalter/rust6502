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
}

impl Addressing for Memory {
    fn read(&self, addr: u16) -> u8 {
        self.mem[(addr - self.offset) as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.mem[(addr - self.offset) as usize] = data;
    }
}
