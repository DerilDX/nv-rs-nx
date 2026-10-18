//! `Havok SDK/run_00df8910` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00df8930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0xc` (`SUB ECX,0xc`) and jumps to `00df9470`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00df8930(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_9470, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00df8940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x4` (`SUB ECX,0x4`) and jumps to `00df9470`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00df8940(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_9470, &args![this.addr().wrapping_sub(0x4), flags])
        .u32()
}

// Translated from 00df8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: copies the byte at `this + 0x14` to the byte at `out` and returns
/// `out` (`EAX`). The class is unconfirmed.
pub fn fn_00df8910(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let value = e.mem.u8(this.addr() + 0x14);
    e.mem.set_u8(out.addr(), value);
    out
}

// Translated from 00df8920 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: returns (`FLD`) the float at `this + 0x10`. The class is
/// unconfirmed.
pub fn fn_00df8920(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x10))
}

// Translated from 00df8950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `8` (`SUB ECX,0x8`) and jumps to `00df9470`, which (`RET 4`) is
/// the scalar deleting destructor the other thunks of this unit share (the decompiler shows its
/// body: four vtable words and a `0x80`-byte free when bit 0 of the flags is set). The caller
/// gets its `EAX`; the class is unconfirmed.
pub fn fn_00df8950(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_9470, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_8930, fn_00df8930(Ptr, u32) -> u32),
        entry!(0x00df_8940, fn_00df8940(Ptr, u32) -> u32),
        entry!(0x00df_8910, fn_00df8910(Ptr, Ptr) -> Ptr),
        entry!(0x00df_8920, fn_00df8920(Ptr) -> f32),
        entry!(0x00df_8950, fn_00df8950(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00df8930() {
        assert_eq!(check(0x00df_8930, 0x00df_9470, 0xc), 0xdead_beef);
    }

    #[test]
    fn thunk_00df8940() {
        assert_eq!(check(0x00df_8940, 0x00df_9470, 0x4), 0xdead_beef);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The byte at +0x14 is copied to `out`, which is returned.
    #[test]
    fn fn_00df8910_copies_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(4);
        e.mem.set_u8(this + 0x14, 0x5a);
        e.mem.set_u8(out, 0xaa);
        assert_eq!(e.call(0x00df_8910, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 0x5a);
    }

    /// The float at +0x10 comes back; a signalling NaN comes back quiet.
    #[test]
    fn fn_00df8920_returns_the_float() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 0x10, -4.25);
        assert_eq!(e.call(0x00df_8920, &args![this]).f32(), -4.25);
        e.mem.set_u32(this + 0x10, 0xff80_0007);
        let bits = e.call(0x00df_8920, &args![this]).f32().to_bits();
        assert_eq!(bits & 0x7fff_ffff, 0x7fc0_0007);
    }

    /// The thunk moves `this` back by 8 and passes the flags on; the callee's `EAX` comes back.
    #[test]
    fn fn_00df8950_moves_this_and_jumps() {
        let mut e = Engine::new();
        e.register(0x00df_9470, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let got = e.call(0x00df_8950, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00df_9470).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - 8, 0x1]);
        assert_eq!(got.u32(), 0xdead_beef);
    }
}
