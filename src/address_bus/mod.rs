#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct AddressingError {
    operation: String,
    addr: u16,
}

impl AddressingError {
    fn new(operation: &str, addr: u16) -> AddressingError {
        AddressingError {
            operation: operation.to_string(),
            addr: addr,
        }
    }
}

impl fmt::Display for AddressingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{:x}", self.operation, self.addr)
    }
}

pub trait Addressing {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub struct AddressBus {
    block_size: u16,
    block_component_map: HashMap<u16, u16>,
    component_addr: HashMap<u16, Box<dyn Addressing>>,
}

impl AddressBus {
    pub fn new(block_size: u16) -> AddressBus {
        AddressBus {
            block_size: block_size,
            block_component_map: HashMap::new(),
            component_addr: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, from_addr: u16, to_addr: u16, component: Box<dyn Addressing>) {
        let from_block = from_addr / self.block_size;
        let to_block = to_addr / self.block_size;
        self.component_addr.insert(from_addr, component);
        for block in from_block..to_block + 1 {
            self.block_component_map.insert(block, from_addr);
        }
    }

    pub fn read(&self, addr: u16) -> Result<u8, AddressingError> {
        let block = addr / self.block_size;
        if self.block_component_map.contains_key(&block) {
            let from_addr = self.block_component_map[&block];
            Ok(self.component_addr[&from_addr].read(addr - from_addr))
        } else {
            Err(AddressingError::new("read", addr))
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) -> Result<(), AddressingError> {
        let block = addr / self.block_size;
        if self.block_component_map.contains_key(&block) {
            let from_addr = self.block_component_map[&block];
            if let Some(x) = self.component_addr.get_mut(&from_addr) {
                x.write(addr - from_addr, data);
            };
            Ok(())
        } else {
            Err(AddressingError::new("write", addr))
        }
    }
}
