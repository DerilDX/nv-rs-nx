//! The game's 32-bit address space.
//!
//! Game objects live here at their real field offsets, and pointers are the
//! 32-bit addresses the game uses. Memory is paged (4 KiB) and sparse: a
//! page exists once something maps or writes it. Reading an unmapped
//! address is a fault that names the address, so a translation that reads
//! data nobody provided stops at once instead of computing with zeros.
//!
//! Regions:
//! - the executable's data sections at their own addresses
//!   (`.rdata` 0x00FDF000, `.data` 0x01183000, ...), mapped from the
//!   player's installed FalloutNV.exe by [`crate::exe`] or set up by a test;
//! - the heap, from [`HEAP_BASE`], handed out by [`Mem::alloc`] (the
//!   translation of the game's allocator is `platform`, see
//!   `units/platform.rs`).

use std::collections::BTreeMap;

const PAGE_BITS: u32 = 12;
const PAGE: usize = 1 << PAGE_BITS;
const PAGES: usize = 1 << (32 - PAGE_BITS);

/// First heap address. Above the executable image (which ends below
/// 0x01410000) and clear of it, so a heap pointer never aliases image data.
pub const HEAP_BASE: u32 = 0x2000_0000;
/// End of the heap region.
pub const HEAP_END: u32 = 0x7FFF_0000;

/// A read or write of memory nobody mapped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fault(pub u32);

pub struct Mem {
    pages: Vec<Option<Box<[u8; PAGE]>>>,
    /// Heap: next fresh address, and freed blocks by size for reuse.
    brk: u32,
    live: BTreeMap<u32, u32>,
    free: BTreeMap<u32, Vec<u32>>,
}

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}

impl Mem {
    pub fn new() -> Self {
        let mut pages = Vec::with_capacity(PAGES);
        pages.resize_with(PAGES, || None);
        Mem {
            pages,
            brk: HEAP_BASE,
            live: BTreeMap::new(),
            free: BTreeMap::new(),
        }
    }

    /// Makes `[addr, addr + len)` addressable (zero-filled where new).
    pub fn map(&mut self, addr: u32, len: u32) {
        if len == 0 {
            return;
        }
        let first = addr >> PAGE_BITS;
        let last = (addr as u64 + len as u64 - 1) >> PAGE_BITS;
        for p in first as u64..=last {
            let slot = &mut self.pages[p as usize];
            if slot.is_none() {
                *slot = Some(Box::new([0; PAGE]));
            }
        }
    }

    /// Makes the pages of `[addr, addr + len)` unaddressable again; their
    /// contents are dropped. `addr` and `len` must be page multiples (the
    /// `VirtualFree` stand-in in `units/platform.rs` decommits this way).
    pub fn unmap(&mut self, addr: u32, len: u32) {
        assert!(
            addr % PAGE as u32 == 0 && len % PAGE as u32 == 0,
            "unmap of {addr:08x}+{len:x}: not whole pages"
        );
        let first = (addr >> PAGE_BITS) as u64;
        for p in first..first + (len >> PAGE_BITS) as u64 {
            self.pages[p as usize] = None;
        }
    }

    pub fn is_mapped(&self, addr: u32) -> bool {
        self.pages[(addr >> PAGE_BITS) as usize].is_some()
    }

    fn page(&self, addr: u32) -> Result<&[u8; PAGE], Fault> {
        self.pages[(addr >> PAGE_BITS) as usize]
            .as_deref()
            .ok_or(Fault(addr))
    }

    fn page_mut(&mut self, addr: u32) -> Result<&mut [u8; PAGE], Fault> {
        self.pages[(addr >> PAGE_BITS) as usize]
            .as_deref_mut()
            .ok_or(Fault(addr))
    }

    /// Reads `buf.len()` bytes; faults on the first unmapped byte.
    pub fn try_read(&self, addr: u32, buf: &mut [u8]) -> Result<(), Fault> {
        let mut a = addr;
        let mut done = 0;
        while done < buf.len() {
            let off = (a as usize) & (PAGE - 1);
            let n = (PAGE - off).min(buf.len() - done);
            let p = self.page(a)?;
            buf[done..done + n].copy_from_slice(&p[off..off + n]);
            done += n;
            a = a.wrapping_add(n as u32);
        }
        Ok(())
    }

    pub fn try_write(&mut self, addr: u32, data: &[u8]) -> Result<(), Fault> {
        let mut a = addr;
        let mut done = 0;
        while done < data.len() {
            let off = (a as usize) & (PAGE - 1);
            let n = (PAGE - off).min(data.len() - done);
            let p = self.page_mut(a)?;
            p[off..off + n].copy_from_slice(&data[done..done + n]);
            done += n;
            a = a.wrapping_add(n as u32);
        }
        Ok(())
    }

    /// Zeroes `[addr, addr + len)` page by page (what writing `len` zero bytes does, without
    /// a buffer of them). Panics as [`Mem::write`] does on the first unmapped byte, with the
    /// bytes before it zeroed.
    pub fn zero(&mut self, addr: u32, len: u32) {
        let mut a = addr;
        let mut left = len as usize;
        while left > 0 {
            let off = (a as usize) & (PAGE - 1);
            let n = (PAGE - off).min(left);
            match self.page_mut(a) {
                Ok(p) => p[off..off + n].fill(0),
                Err(Fault(at)) => panic!("write to unmapped memory at {at:08x}"),
            }
            left -= n;
            a = a.wrapping_add(n as u32);
        }
    }

    /// Reads bytes. Panics with the address on a fault: a translated
    /// function touched memory that was never provided.
    pub fn read(&self, addr: u32, buf: &mut [u8]) {
        if let Err(Fault(a)) = self.try_read(addr, buf) {
            panic!("read of unmapped memory at {a:08x}");
        }
    }

    pub fn write(&mut self, addr: u32, data: &[u8]) {
        if let Err(Fault(a)) = self.try_write(addr, data) {
            panic!("write to unmapped memory at {a:08x}");
        }
    }

    pub fn bytes(&self, addr: u32, len: u32) -> Vec<u8> {
        let mut v = vec![0; len as usize];
        self.read(addr, &mut v);
        v
    }

    /// The `N` bytes at `a`: a fixed-size copy out of the page when they lie in one page (no
    /// call to `memcpy`, which a variable-length copy costs; the translated code reads memory
    /// hundreds of millions of times a second), else [`Mem::read`] across the page boundary.
    /// Faults (panics) exactly as [`Mem::read`] does.
    #[inline(always)]
    fn load<const N: usize>(&self, a: u32) -> [u8; N] {
        let off = (a as usize) & (PAGE - 1);
        if off <= PAGE - N {
            if let Some(p) = self.pages[(a >> PAGE_BITS) as usize].as_deref() {
                let mut b = [0; N];
                b.copy_from_slice(&p[off..off + N]);
                return b;
            }
        }
        self.load_slow(a)
    }

    #[cold]
    #[inline(never)]
    fn load_slow<const N: usize>(&self, a: u32) -> [u8; N] {
        let mut b = [0; N];
        self.read(a, &mut b);
        b
    }

    /// Writes the `N` bytes `b` at `a`, as [`Mem::load`] reads them.
    #[inline(always)]
    fn store<const N: usize>(&mut self, a: u32, b: [u8; N]) {
        let off = (a as usize) & (PAGE - 1);
        if off <= PAGE - N {
            if let Some(p) = self.pages[(a >> PAGE_BITS) as usize].as_deref_mut() {
                p[off..off + N].copy_from_slice(&b);
                return;
            }
        }
        self.store_slow(a, b)
    }

    #[cold]
    #[inline(never)]
    fn store_slow<const N: usize>(&mut self, a: u32, b: [u8; N]) {
        self.write(a, &b);
    }

    #[inline]
    pub fn u8(&self, a: u32) -> u8 {
        self.load::<1>(a)[0]
    }
    #[inline]
    pub fn u16(&self, a: u32) -> u16 {
        u16::from_le_bytes(self.load(a))
    }
    #[inline]
    pub fn u32(&self, a: u32) -> u32 {
        u32::from_le_bytes(self.load(a))
    }
    #[inline]
    pub fn u64(&self, a: u32) -> u64 {
        u64::from_le_bytes(self.load(a))
    }
    #[inline]
    pub fn i8(&self, a: u32) -> i8 {
        self.u8(a) as i8
    }
    #[inline]
    pub fn i16(&self, a: u32) -> i16 {
        self.u16(a) as i16
    }
    #[inline]
    pub fn i32(&self, a: u32) -> i32 {
        self.u32(a) as i32
    }
    #[inline]
    pub fn f32(&self, a: u32) -> f32 {
        f32::from_bits(self.u32(a))
    }
    #[inline]
    pub fn f64(&self, a: u32) -> f64 {
        f64::from_bits(self.u64(a))
    }

    #[inline]
    pub fn set_u8(&mut self, a: u32, v: u8) {
        self.store(a, [v]);
    }
    #[inline]
    pub fn set_u16(&mut self, a: u32, v: u16) {
        self.store(a, v.to_le_bytes());
    }
    #[inline]
    pub fn set_u32(&mut self, a: u32, v: u32) {
        self.store(a, v.to_le_bytes());
    }
    #[inline]
    pub fn set_u64(&mut self, a: u32, v: u64) {
        self.store(a, v.to_le_bytes());
    }
    #[inline]
    pub fn set_i8(&mut self, a: u32, v: i8) {
        self.set_u8(a, v as u8);
    }
    #[inline]
    pub fn set_i16(&mut self, a: u32, v: i16) {
        self.set_u16(a, v as u16);
    }
    #[inline]
    pub fn set_i32(&mut self, a: u32, v: i32) {
        self.set_u32(a, v as u32);
    }
    #[inline]
    pub fn set_f32(&mut self, a: u32, v: f32) {
        self.set_u32(a, v.to_bits());
    }
    #[inline]
    pub fn set_f64(&mut self, a: u32, v: f64) {
        self.set_u64(a, v.to_bits());
    }

    /// NUL-terminated byte string at `a` (without the NUL).
    pub fn cstr(&self, a: u32) -> Vec<u8> {
        let mut out = Vec::new();
        let mut p = a;
        loop {
            let c = self.u8(p);
            if c == 0 {
                return out;
            }
            out.push(c);
            p = p.wrapping_add(1);
        }
    }

    /// Writes `s` and a NUL at `a`.
    pub fn set_cstr(&mut self, a: u32, s: &[u8]) {
        self.write(a, s);
        self.set_u8(a.wrapping_add(s.len() as u32), 0);
    }

    /// The heap's current top: the next fresh address [`Mem::alloc`] hands
    /// out (reused freed blocks lie below it). Tools that map their own memory
    /// inside the heap range above it (the oracle check's buffer arena at
    /// 0x30000000) check that the heap has not reached them.
    pub fn heap_top(&self) -> u32 {
        self.brk
    }

    /// Allocates `size` bytes (8-byte aligned, zero-filled) on the heap.
    /// Freed blocks of the same rounded size are reused, newest first.
    pub fn alloc(&mut self, size: u32) -> u32 {
        let size = size.max(1).div_ceil(8) * 8;
        let addr = match self.free.get_mut(&size).and_then(Vec::pop) {
            Some(a) => a,
            None => {
                let a = self.brk;
                assert!(
                    (a as u64 + size as u64) <= HEAP_END as u64,
                    "heap exhausted allocating {size} bytes"
                );
                self.brk += size;
                a
            }
        };
        self.map(addr, size);
        self.zero(addr, size);
        self.live.insert(addr, size);
        addr
    }

    /// Frees a block from [`Mem::alloc`]. Freeing 0 does nothing (as the
    /// game's `Deallocate`); freeing anything else that is not a live block
    /// panics.
    pub fn free(&mut self, addr: u32) {
        if addr == 0 {
            return;
        }
        let size = self
            .live
            .remove(&addr)
            .unwrap_or_else(|| panic!("free of {addr:08x}, which is not a live heap block"));
        self.free.entry(size).or_default().push(addr);
    }

    /// Size of a live heap block.
    pub fn block_size(&self, addr: u32) -> Option<u32> {
        self.live.get(&addr).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unmapped_reads_fault() {
        let m = Mem::new();
        let mut b = [0; 4];
        assert_eq!(m.try_read(0x0118_3000, &mut b), Err(Fault(0x0118_3000)));
    }

    #[test]
    fn values_cross_pages() {
        let mut m = Mem::new();
        m.map(0x1000, 0x2000);
        m.set_u32(0x1ffe, 0xdead_beef);
        assert_eq!(m.u32(0x1ffe), 0xdead_beef);
        m.set_f64(0x1ffc, 1.25);
        assert_eq!(m.f64(0x1ffc), 1.25);
    }

    /// The fixed-size accessors read and write every width at every offset of a page, and at
    /// the page's end (where they go through the general path) as bytes would.
    #[test]
    fn scalar_accessors_match_byte_reads_at_every_offset() {
        let mut m = Mem::new();
        m.map(0x1000, 0x2000);
        for (i, a) in (0x1ff0..0x2010u32).enumerate() {
            m.set_u8(a, i as u8 ^ 0x5a);
        }
        for a in 0x1ff0..0x2008u32 {
            let b = m.bytes(a, 8);
            assert_eq!(m.u8(a), b[0]);
            assert_eq!(m.u16(a), u16::from_le_bytes(b[..2].try_into().unwrap()));
            assert_eq!(m.u32(a), u32::from_le_bytes(b[..4].try_into().unwrap()));
            assert_eq!(m.u64(a), u64::from_le_bytes(b.try_into().unwrap()));
        }
        for a in 0x1ff8..0x2001u32 {
            m.set_u64(a, 0x0102_0304_0506_0708);
            assert_eq!(m.bytes(a, 8), [8, 7, 6, 5, 4, 3, 2, 1]);
            m.set_u16(a, 0xa1b2);
            assert_eq!(m.bytes(a, 2), [0xb2, 0xa1]);
        }
    }

    /// A read of an unmapped page faults with the address, from the fast path and from a
    /// read that crosses into the unmapped page (with the first unmapped byte's address).
    #[test]
    #[should_panic(expected = "read of unmapped memory at 00005000")]
    fn unmapped_scalar_reads_name_the_address() {
        Mem::new().u32(0x5000);
    }

    #[test]
    #[should_panic(expected = "read of unmapped memory at 00002000")]
    fn reads_into_an_unmapped_page_name_its_first_byte() {
        let mut m = Mem::new();
        m.map(0x1000, 0x1000);
        m.u32(0x1ffe);
    }

    /// A write that crosses into an unmapped page writes the mapped part, then faults there.
    #[test]
    fn writes_into_an_unmapped_page_fault_after_the_mapped_part() {
        let mut m = Mem::new();
        m.map(0x1000, 0x1000);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            m.set_u32(0x1ffe, 0xaabb_ccdd)
        }));
        assert!(r.is_err());
        assert_eq!(m.bytes(0x1ffe, 2), [0xdd, 0xcc]);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut m = Mem::new();
            m.set_u8(0x7000, 1)
        }));
        let why = r.unwrap_err();
        assert_eq!(
            why.downcast_ref::<String>().map(String::as_str),
            Some("write to unmapped memory at 00007000")
        );
    }

    /// `zero` clears a range across pages.
    #[test]
    fn zero_clears_across_pages() {
        let mut m = Mem::new();
        m.map(0x1000, 0x2000);
        m.write(0x1ff0, &[0xff; 0x20]);
        m.zero(0x1ff4, 0x18);
        let b = m.bytes(0x1ff0, 0x20);
        assert_eq!(&b[..4], &[0xff; 4]);
        assert!(b[4..0x1c].iter().all(|&x| x == 0));
        assert_eq!(&b[0x1c..], &[0xff; 4]);
    }

    #[test]
    fn heap_reuses_freed_blocks() {
        let mut m = Mem::new();
        let a = m.alloc(12);
        let b = m.alloc(12);
        assert_eq!(b, a + 16);
        m.set_u32(a, 7);
        m.free(a);
        let c = m.alloc(16);
        assert_eq!(c, a);
        assert_eq!(m.u32(c), 0, "reused blocks are zeroed");
        m.free(0);
    }

    #[test]
    fn strings() {
        let mut m = Mem::new();
        let a = m.alloc(16);
        m.set_cstr(a, b"Goodsprings");
        assert_eq!(m.cstr(a), b"Goodsprings");
    }
}
