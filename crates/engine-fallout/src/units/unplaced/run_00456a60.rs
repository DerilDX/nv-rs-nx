//! `(unplaced)/run_00456a60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte global `00456c70` returns.
const FLAG_011C3C0D: u32 = 0x011c_3c0d;
/// `GridCellArray::Get` (Xbox PDB): thiscall(x, y).
const GRID_CELL_ARRAY_GET: u32 = 0x004b_a490;
/// `TESObjectCELL::Get3D` (Xbox PDB): thiscall, no stack words.
const CELL_GET_3D: u32 = 0x0054_5cb0;
/// `0043b4a0` (`modelloader.cpp`): thiscall(word) on the cell's 3D.
const NODE_LOOKUP_43B4A0: u32 = 0x0043_b4a0;

// Translated from 00456c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `011c3c0d` (in `AL`).
pub fn fn_00456c70(e: &mut Engine) -> u8 {
    e.global::<u8>(FLAG_011C3C0D)
}

// Translated from 00456fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For the cell at `this`: when `TESObjectCELL::Get3D` gives a node, the
/// result of `0043b4a0(node, word)`; null otherwise.
pub fn fn_00456fc0(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    let node = e.call(CELL_GET_3D, &args![this]).u32();
    if node == 0 {
        0
    } else {
        e.call(NODE_LOOKUP_43B4A0, &args![node, word]).u32()
    }
}

// Translated from 00457050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::Get(x, y)` on the grid at `this + 8`, its result passed
/// through (`EAX` is left as the callee set it).
pub fn fn_00457050(e: &mut Engine, this: Ptr, x: u32, y: u32) -> u32 {
    let grid = e.mem.u32(this.addr().wrapping_add(8));
    e.call(GRID_CELL_ARRAY_GET, &args![grid, x, y]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0045_6c70, fn_00456c70() -> u8),
        entry!(0x0045_6fc0, fn_00456fc0(Ptr, u32) -> u32),
        entry!(0x0045_7050, fn_00457050(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_reads_the_byte_global() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(0x011c_3c0c, 0xffff_33ffu32);
        assert_eq!(e.call(0x0045_6c70, &args![]).u32() & 0xff, 0x33);
    }

    #[test]
    fn cell_lookup_needs_the_cells_3d() {
        let mut e = Engine::new();
        e.register(CELL_GET_3D, |_, _| Ret::default());
        e.register(NODE_LOOKUP_43B4A0, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x0045_6fc0, &args![0x1000u32, 7u32]).u32(), 0);
        e.register(CELL_GET_3D, |_, _| 0x5000u32.into_ret());
        assert_eq!(e.call(0x0045_6fc0, &args![0x1000u32, 7u32]).u32(), 0x5007);
    }

    #[test]
    fn grid_get_goes_through_the_grid_at_8() {
        let mut e = Engine::new();
        e.register(GRID_CELL_ARRAY_GET, |_, a| (a[0] ^ a[1] ^ a[2]).into_ret());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        let back = e.call(0x0045_7050, &args![this, 3u32, 5u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0x7000 ^ 3 ^ 5);
        let call = log.iter().find(|(a, _)| *a == GRID_CELL_ARRAY_GET).unwrap();
        assert_eq!(call.1, vec![0x7000, 3, 5]);
    }
}
