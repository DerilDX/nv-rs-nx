//! `(unplaced)/run_00e673f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e67fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: clears one word in each of two arrays of
/// records inside the object. For each of the `[this + 0x10]` records of
/// 0x50 bytes starting at `this + 0xfb4`, the first word is set to 0; then,
/// for each of the `[this + 8]` records of 0x40 bytes starting at
/// `this + 0x50`, the first word is set to 0. The counts are read again
/// after every record, as the code does.
pub fn fn_00e67fd0(e: &mut Engine, this: Ptr) {
    let mut index = 0i32;
    if e.mem.i32(this.addr() + 0x10) > 0 {
        let mut record = this.addr() + 0xfb4;
        loop {
            e.mem.set_u32(record, 0);
            index += 1;
            record += 0x50;
            if index >= e.mem.i32(this.addr() + 0x10) {
                break;
            }
        }
    }
    index = 0;
    if e.mem.i32(this.addr() + 8) > 0 {
        let mut record = this.addr() + 0x50;
        loop {
            e.mem.set_u32(record, 0);
            index += 1;
            record += 0x40;
            if index >= e.mem.i32(this.addr() + 8) {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e67fd0, fn_00e67fd0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e67fd0_clears_the_first_word_of_each_record() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1200);
        e.mem.set_i32(this + 0x10, 3);
        e.mem.set_i32(this + 8, 2);
        for offset in (0..0x1200).step_by(4) {
            e.mem.set_u32(this + offset, 0x5555_5555);
        }
        e.mem.set_i32(this + 0x10, 3);
        e.mem.set_i32(this + 8, 2);
        e.call(0x00e6_7fd0, &args![Ptr::<()>::new(this)]);
        let cleared: Vec<u32> = (0..0x1200u32)
            .step_by(4)
            .filter(|offset| e.mem.u32(this + offset) == 0)
            .collect();
        assert_eq!(cleared, vec![0x50, 0x90, 0xfb4, 0x1004, 0x1054]);
        // A zero count touches nothing.
        e.mem.set_i32(this + 0x10, 0);
        e.mem.set_i32(this + 8, -1);
        e.mem.set_u32(this + 0x50, 7);
        e.call(0x00e6_7fd0, &args![Ptr::<()>::new(this)]);
        assert_eq!(e.mem.u32(this + 0x50), 7);
    }
}
