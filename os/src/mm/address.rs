use core::{fmt::Debug, slice};

use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};

use super::page_table::PageTableEntry;

const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhysAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhysPageNum(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtPageNum(pub usize);

impl From<usize> for PhysAddr {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtAddr {
    fn from(value: usize) -> Self {
        Self(value & ((1 << VA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PPN_WIDTH_SV39) - 1))
    }
}
impl From<usize> for VirtPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << VPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddr> for usize {
    fn from(value: PhysAddr) -> Self {
        value.0
    }
}
impl From<VirtAddr> for usize {
    fn from(value: VirtAddr) -> Self {
        if value.0 >= (1 << (VA_WIDTH_SV39 - 1)) {
            value.0 | !((1 << VA_WIDTH_SV39) - 1)
        } else {
            value.0
        }
    }
}
impl From<VirtPageNum> for usize {
    fn from(value: VirtPageNum) -> Self {
        value.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(value: PhysPageNum) -> Self {
        value.0
    }
}

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 + (PAGE_SIZE - 1)) / PAGE_SIZE)
    }
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn aligned(&self) -> bool {
        return self.page_offset() == 0;
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(value: PhysAddr) -> Self {
        assert!(value.aligned());
        value.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(value: PhysPageNum) -> Self {
        Self(value.0 << PAGE_SIZE_BITS)
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 + (PAGE_SIZE - 1)) / PAGE_SIZE)
    }
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn aligned(&self) -> bool {
        return self.page_offset() == 0;
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(value: VirtAddr) -> Self {
        assert!(value.aligned());
        value.floor()
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(value: VirtPageNum) -> Self {
        Self(value.0 << PAGE_SIZE_BITS)
    }
}

impl PhysPageNum {
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = self.clone().into();
        unsafe { slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, PAGE_SIZE / 8) }
    }
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = self.clone().into();
        unsafe { slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE) }
    }
    pub fn clear_page(&self) {
        self.get_bytes_array()
            .iter_mut()
            .for_each(|v| unsafe { (v as *mut u8).write_volatile(0) })
    }
    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = self.clone().into();
        unsafe { (pa.0 as *mut T).as_mut().unwrap() }
    }
}
impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & ((1 << 9) - 1);
            vpn >>= 9;
        }
        idx
    }
}

impl StepByOne for VirtPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

pub trait StepByOne {
    fn step(&mut self);
}

#[derive(Clone, Copy)]
pub struct SimpleRange<T> {
    start: T,
    end: T,
}

impl<T> SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end);
        Self { start, end }
    }
    pub fn get_start(&self) -> T {
        self.start
    }
    pub fn get_end(&self) -> T {
        self.end
    }
}

impl<T> IntoIterator for SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;

    type IntoIter = SimpleRangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator::new(self.start, self.end)
    }
}

pub struct SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    current: T,
    end: T,
}

impl<T> SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    fn new(current: T, end: T) -> Self {
        Self { current, end }
    }
}

impl<T> Iterator for SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}

pub type VPNRange = SimpleRange<VirtPageNum>;
