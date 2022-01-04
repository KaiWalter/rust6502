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
    fn len(&self) -> usize;
}

pub struct AddressBus<'a> {
    block_size: usize,
    block_component_map: HashMap<u16, u16>, // map a 1..n blocks to 1 components
    component_addr: HashMap<u16, &'a mut (dyn Addressing)>, // 1:1 map component to its addressing
}

impl<'a> AddressBus<'a> {
    pub fn new(block_size: usize) -> AddressBus<'a> {
        AddressBus {
            block_size: block_size,
            block_component_map: HashMap::new(),
            component_addr: HashMap::new(),
        }
    }

    pub fn add_component(
        &mut self,
        from_addr: u16,
        size: usize,
        component: &'a mut (dyn Addressing),
    ) -> Result<(), AddressingError> {
        let size_outside_blocks = size % self.block_size as usize;
        let mem_outside_block = component.len() % self.block_size as usize;

        if size_outside_blocks == 0 && mem_outside_block == 0 {
            let component_key = self.component_addr.len() as u16;
            self.component_addr.insert(component_key, component);

            let from_block = (from_addr as usize / self.block_size) as u16;
            let to_block = ((from_addr as usize + size as usize) / self.block_size as usize) as u16;
            for block in from_block..to_block {
                self.block_component_map.insert(block, component_key);
            }

            Ok(())
        } else {
            Err(AddressingError::new("add_compontent", 0))
        }
    }

    pub fn read(&self, addr: u16) -> Result<u8, AddressingError> {
        let block = (addr as usize / self.block_size) as u16;
        if self.block_component_map.contains_key(&block) {
            let component_key = self.block_component_map[&block];
            Ok(self.component_addr[&component_key].read(addr))
        } else {
            Err(AddressingError::new("read", addr))
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) -> Result<(), AddressingError> {
        let block = (addr as usize / self.block_size) as u16;
        if self.block_component_map.contains_key(&block) {
            let component_key = self.block_component_map[&block];
            if let Some(x) = self.component_addr.get_mut(&component_key) {
                x.write(addr, data);
            };
            Ok(())
        } else {
            Err(AddressingError::new("write", addr))
        }
    }
}
