use alloc::vec;
use alloc::vec::Vec;
use bitflags::bitflags;

use super::{
    address::{PhysPageNum, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
};

bitflags! {
    pub struct PTEFlags: u8 {
       const V = 1<<0;
       const R = 1<<1;
       const W = 1<<2;
       const X = 1<<3;
       const U = 1<<4;
       const G = 1<<5;
       const A = 1<<6;
       const D = 1<<7;
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }
    pub fn empty() -> Self {
        Self { bits: 0 }
    }
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1 << 44) - 1)).into()
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        self.flags() & PTEFlags::V != PTEFlags::empty()
    }
    pub fn readable(&self) -> bool {
        self.flags() & PTEFlags::R != PTEFlags::empty()
    }
    pub fn writeable(&self) -> bool {
        self.flags() & PTEFlags::W != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        self.flags() & PTEFlags::X != PTEFlags::empty()
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();
        Self {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", ppn);
        *pte = PageTableEntry::new(ppn, PTEFlags::V | flags);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }

    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & (1usize << 44) - 1),
            frames: Vec::new(),
        }
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| pte.clone())
    }

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;

        for (i, &idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[idx];
            if i == idxs.len() - 1 {
                return Some(pte);
            }
            if !pte.is_valid() {
                let frame = frame_alloc();
                if frame.is_none() {
                    return None;
                }
                let frame = frame.unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }

        unreachable!()
    }

    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        for (i, &idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[idx];
            if i == idxs.len() - 1 {
                return Some(pte);
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        None
    }

    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }
}
