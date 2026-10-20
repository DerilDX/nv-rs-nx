//! `NiXenonRenderer/run_00e7eea0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e7ee70 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor the entry pool's `vector constructor iterator` runs on each 0x10-byte
/// render-state entry (code Ghidra made no function of), `thiscall`, no stack words: clears
/// the four words and answers `this`.
pub fn fn_00e7ee70(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in [0u32, 4, 8, 0xc] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    this
}

// Translated from 00e7ee80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor the block pool's `vector constructor iterator` runs on each 0x14-byte
/// render-state block (code Ghidra made no function of), `thiscall`, no stack words: clears
/// the byte +0 and the words +4 to +0x10 and answers `this`.
pub fn fn_00e7ee80(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u8(base, 0);
    for offset in [4u32, 8, 0xc, 0x10] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

/// The globals `00e7eea0` sets: the renderer (`0126f710`) and its shader library word
/// (`0126f714`, the renderer's +0x8b8).
const RENDERER_0126F710: u32 = 0x0126_f710;
const LIBRARY_0126F714: u32 = 0x0126_f714;

// Translated from 00e7eea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7eea0`), `cdecl`, the renderer: keeps it at `0126f710`
/// and its +0x8b8 at `0126f714` (0 without a renderer).
pub fn fn_00e7eea0(e: &mut Engine, renderer: u32) {
    e.mem.set_u32(RENDERER_0126F710, renderer);
    let library = if renderer != 0 {
        e.mem.u32(renderer.wrapping_add(0x8b8))
    } else {
        0
    };
    e.mem.set_u32(LIBRARY_0126F714, library);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_eea0, fn_00e7eea0(u32)),
        entry!(0x00e7_ee70, fn_00e7ee70(Ptr) -> Ptr),
        entry!(0x00e7_ee80, fn_00e7ee80(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_constructors_clear_their_fields() {
        let mut e = Engine::new();
        let entry = e.mem.alloc(0x14);
        e.mem.write(entry, &[0xcc; 0x14]);
        assert_eq!(e.call(0x00e7_ee70, &args![entry]).u32(), entry);
        assert_eq!(e.mem.bytes(entry, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(entry + 0x10), 0xcccc_cccc);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xcc; 0x18]);
        assert_eq!(e.call(0x00e7_ee80, &args![block]).u32(), block);
        assert_eq!(e.mem.u8(block), 0);
        assert_eq!(e.mem.bytes(block + 1, 3), vec![0xcc; 3]);
        assert_eq!(e.mem.bytes(block + 4, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x14), 0xcccc_cccc);
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e7eea0_keeps_the_renderer_and_library() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x8b8, 0x77);
        e.call(0x00e7_eea0, &args![renderer]);
        assert_eq!(e.mem.u32(RENDERER_0126F710), renderer);
        assert_eq!(e.mem.u32(LIBRARY_0126F714), 0x77);
        e.call(0x00e7_eea0, &args![0u32]);
        assert_eq!(e.mem.u32(LIBRARY_0126F714), 0);
    }
}
