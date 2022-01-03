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
    fn len(&self) -> u16;
}

pub struct AddressBus<'a> {
    block_size: u16,
    block_component_map: HashMap<u16, u16>, // map a 1..n blocks to 1 components
    component_addr: HashMap<u16, &'a mut (dyn Addressing)>, // 1:1 map component to its addressing
}

impl<'a> AddressBus<'a> {
    pub fn new(block_size: u16) -> AddressBus<'a> {
        AddressBus {
            block_size: block_size,
            block_component_map: HashMap::new(),
            component_addr: HashMap::new(),
        }
    }

    pub fn add_component(
        &mut self,
        from_addr: u16,
        size: u16,
        component: &'a mut (dyn Addressing),
    ) -> Result<(), AddressingError> {
        let size_outside_blocks = size % self.block_size;
        let mem_outside_block = component.len() % self.block_size;
        if size_outside_blocks == 0 && mem_outside_block == 0 {
            self.component_addr.insert(from_addr, component);
            let from_block = from_addr / self.block_size;
            let to_block = (from_addr + size) / self.block_size;
            for block in from_block..to_block {
                self.block_component_map.insert(block, from_addr);
            }
            Ok(())
        } else {
            Err(AddressingError::new("add_compontent", 0))
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
