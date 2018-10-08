use super::Error;
use std::rc::Rc;

pub mod flat;
pub mod mmu;

pub const PROT_READ: u32 = 0b0001;
pub const PROT_WRITE: u32 = 0b0010;
pub const PROT_EXEC: u32 = 0b0100;

pub trait Memory {
    // Note this mmap only handles the very low level memory mapping logic.
    // It only takes an aligned address and size, then maps either existing
    // bytes or empty bytes to this range. It doesn't allocate addresses when
    // given 0 as address value. Instead, higher level machine should be leveraged
    // to manage code, heap(brk), mmap and stack regions.
    fn mmap(
        &mut self,
        addr: usize,
        size: usize,
        prot: u32,
        // TODO: we know using Rc and Box here is less optimal since we are adding
        // dynamic calls. The reason we are sticking with it, is that it would require
        // changing a whole lot of files if we add lifetime parameters here. Also in the
        // future, we might want to refactor this into a virtual file system style API,
        // or even remove Memory trait and merge everything into Machine trait, so
        // we are sticking with the simpler solution for now.
        source: Option<Rc<Box<[u8]>>>,
        offset: usize,
    ) -> Result<(), Error>;
    fn munmap(&mut self, addr: usize, size: usize) -> Result<(), Error>;

    // TODO: maybe parameterize those?
    fn load8(&mut self, addr: usize) -> Result<u8, Error>;
    fn load16(&mut self, addr: usize) -> Result<u16, Error>;
    fn load32(&mut self, addr: usize) -> Result<u32, Error>;
    fn load64(&mut self, addr: usize) -> Result<u64, Error>;

    fn execute_load16(&mut self, addr: usize) -> Result<u16, Error>;

    fn store8(&mut self, addr: usize, value: u8) -> Result<(), Error>;
    fn store16(&mut self, addr: usize, value: u16) -> Result<(), Error>;
    fn store32(&mut self, addr: usize, value: u32) -> Result<(), Error>;
    fn store64(&mut self, addr: usize, value: u64) -> Result<(), Error>;
    fn store_bytes(&mut self, addr: usize, value: &[u8]) -> Result<(), Error>;
}
