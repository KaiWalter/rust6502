#[cfg(test)]
mod tests;

use std::fmt;

#[derive(Debug)]
pub struct AddressingError {
    operation: String,
    addr: u16,
}

impl AddressingError {
    pub fn new(operation: &str, addr: u16) -> AddressingError {
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

pub trait InternalAddressing {
    fn int_read(&mut self, addr: u16) -> u8;
    fn int_write(&mut self, addr: u16, data: u8);
    fn len(&self) -> usize;
}

pub trait ExternalAddressing {
    fn read(&mut self, addr: u16) -> Result<u8, AddressingError>;
    fn write(&mut self, addr: u16, data: u8) -> Result<(), AddressingError>;
}

pub struct AddressBus<'a> {
    block_size: usize,
    block_component_map: Vec<usize>, // map a 1..n blocks to 1 components
    component_addr: Vec<&'a mut dyn InternalAddressing>, // 1:1 map component to its addressing
}

impl<'a> AddressBus<'a> {
    pub fn new(block_size: usize) -> AddressBus<'a> {
        AddressBus {
            block_size: block_size,
            block_component_map: vec![usize::MAX; 0x10000 / block_size], // assume 64kB max addressable space
            component_addr: vec![],
        }
    }

    pub fn add_component(
        &mut self,
        from_addr: u16,
        size: usize,
        component: &'a mut (dyn InternalAddressing),
    ) -> Result<(), AddressingError> {
        let size_outside_blocks = size % self.block_size as usize;
        let mem_outside_block = component.len() % self.block_size as usize;

        if size_outside_blocks == 0 && mem_outside_block == 0 {
            let component_key = self.component_addr.len();
            self.component_addr.push(component);

            let from_block = from_addr as usize / self.block_size;
            let to_block = ((from_addr as usize + size as usize - 1) / self.block_size) + 1;
            for block in from_block..to_block {
                self.block_component_map[block] = component_key;
            }

            Ok(())
        } else {
            Err(AddressingError::new("add_compontent", 0))
        }
    }
}

impl ExternalAddressing for AddressBus<'_> {
    fn read(&mut self, addr: u16) -> Result<u8, AddressingError> {
        let block = addr as usize / self.block_size;
        if self.block_component_map[block] == usize::MAX {
            Err(AddressingError::new("read", addr))
        } else {
            let component_key = self.block_component_map[block];
            match self.component_addr.get_mut(component_key) {
                Some(component) => Ok(component.int_read(addr)),
                None => Err(AddressingError::new("read", addr)),
            }
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), AddressingError> {
        let block = addr as usize / self.block_size;
        if self.block_component_map[block] == usize::MAX {
            Err(AddressingError::new("write", addr))
        } else {
            let component_key = self.block_component_map[block];
            match self.component_addr.get_mut(component_key) {
                Some(component) => {
                    component.int_write(addr, data);
                    Ok(())
                }
                None => Err(AddressingError::new("write", addr)),
            }
        }
    }
}
