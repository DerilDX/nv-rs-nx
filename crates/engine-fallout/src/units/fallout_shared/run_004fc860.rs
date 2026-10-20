//! `fallout shared/run_004fc860` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fcbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `004fc240`. `004fc240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_004fcbf0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_c240, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

/// `00624700`: returns `this + 0x48` (the flags word of the water form).
const FLAGS_ADDRESS: u32 = 0x0062_4700;

// Translated from 004fc990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` nonzero) or clears (`value` zero) the bits `mask` of the
/// flags word whose address `00624700` returns for `this` (`RET 8`).
pub fn fn_004fc990(e: &mut Engine, this: Ptr, mask: u32, value: u8) {
    if value != 0 {
        let flags = e.call(FLAGS_ADDRESS, &args![this]).u32();
        let word = e.mem.u32(flags) | mask;
        e.mem.set_u32(flags, word);
    } else {
        let flags = e.call(FLAGS_ADDRESS, &args![this]).u32();
        let word = !mask & e.mem.u32(flags);
        e.mem.set_u32(flags, word);
    }
}

// Translated from 004fc940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form at +0x4C, or what `004fb070` (`bgsexplosion.cpp`, thiscall on
/// that form) gives for it when the form is there, form flag 0x8 is set
/// (`004013e0`) and `004fb070` gives nonzero (it is then called a second time
/// for the result).
pub fn fn_004fc940(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + 0x4c) != 0 && e.call(0x0040_13e0, &args![this]).bool() {
        let form = e.mem.u32(base + 0x4c);
        if e.call(0x004f_b070, &args![form]).u32() != 0 {
            let form = e.mem.u32(base + 0x4c);
            return e.call(0x004f_b070, &args![form]).u32();
        }
    }
    e.mem.u32(base + 0x4c)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_cbf0, fn_004fcbf0(Ptr, u32) -> u32),
        entry!(0x004f_c990, fn_004fc990(Ptr, u32, u8)),
        entry!(0x004f_c940, fn_004fc940(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_004fcbf0() {
        assert_eq!(
            check(0x004f_cbf0, 0x004f_c240, 0x30, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod flag_tests {
    use crate::prelude::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn at_004fc990_sets_or_clears_the_mask() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        e.register(0x0062_4700, |_, a| ret(a[0] + 0x48));
        e.mem.set_u32(this + 0x48, 0x0000_00f0);
        e.call(0x004f_c990, &[this, 0x8000_0001, 1]);
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_00f1);
        e.call(0x004f_c990, &[this, 0x0000_0030, 0x100]);
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_00c1);
        e.call(0x004f_c990, &[this, 0x0000_0002, 0x80]);
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_00c3);
    }

    fn getter(form: u32, flag: u32, mapped: u32) -> (u32, usize) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x4c, form);
        e.register_double(0x0040_13e0, move |_, _| ret(flag));
        e.register_double(0x004f_b070, move |_, _| ret(mapped));
        e.call_log = Some(vec![]);
        let r = e.call(0x004f_c940, &[this]).u32();
        let n = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == 0x004f_b070)
            .count();
        (r, n)
    }

    #[test]
    fn at_004fc940_maps_the_form_only_when_flag_8_and_a_mapping_exist() {
        assert_eq!(getter(0, 1, 0x77), (0, 0));
        assert_eq!(getter(0x40, 0x100, 0x77), (0x40, 0));
        assert_eq!(getter(0x40, 1, 0), (0x40, 1));
        assert_eq!(getter(0x40, 1, 0x77), (0x77, 2));
    }
}
