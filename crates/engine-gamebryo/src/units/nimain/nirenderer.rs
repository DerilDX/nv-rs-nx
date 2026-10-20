//! `NiMain/nirenderer.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRenderer::ms_RTTI` (the class's `NiRTTI` record).
const NI_RENDERER_RTTI: u32 = 0x011f_4758;

// Translated from 00a62b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderer::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_renderer_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_RENDERER_RTTI)
}

// Translated from 00a63230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderer::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a62b60`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x280 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_renderer_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_2b60;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x280u32]);
    }
    this
}

/// Import slots of `DeleteCriticalSection` and `InterlockedDecrement` (`stdcall`, one word).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `011f4748`: the global `00a62b60` clears.
const RENDERER_GLOBAL: u32 = 0x011f_4748;
/// `00a5d3d0` (unit `NiMain/niobject.cpp`): reached with a `JMP` (same `this`).
const OBJECT_TAIL: u32 = 0x00a5_d3d0;

// Translated from 00a62b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nirenderer.cpp`, `thiscall`, no stack word): sets the
/// vtable `0109bb94`, clears `011f4748`, deletes the critical sections at +0x180, +0x100 and
/// +0x80 (in that order), drops one reference of the objects at +0x14 and +8 when set
/// (`InterlockedDecrement` of the count at +4; at zero their vtable slot +4 runs), and
/// continues in `00a5d3d0` (a `JMP`; its result is passed on).
pub fn fn_00a62b60(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base, 0x0109_bb94);
    e.mem.set_u32(RENDERER_GLOBAL, 0);
    for offset in [0x180u32, 0x100, 0x80] {
        e.call(DELETE_CRITICAL_SECTION, &args![base + offset]);
    }
    for offset in [0x14u32, 8] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            let count = e
                .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
                .u32();
            if count == 0 {
                e.vcall(object, 4, &args![]);
            }
        }
    }
    e.call(OBJECT_TAIL, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a62b50, ni_renderer_get_rtti() -> Ptr),
        entry!(0x00a63230, ni_renderer_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a6_2b60, fn_00a62b60(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_renderer_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a62b50, &args![]).u32(), NI_RENDERER_RTTI);
    }

    #[test]
    fn ni_renderer_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_2b60, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a63230, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a63230, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x280]]);
        assert_eq!(calls_to(&e, 0x00a6_2b60), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a62b60_deletes_the_locks_and_releases_both_objects() {
        let mut e = Engine::new();
        e.map(RENDERER_GLOBAL & !0xfff, 0x1000);
        e.mem.set_u32(RENDERER_GLOBAL, 5);
        e.register(DELETE_CRITICAL_SECTION, |_, _| Ret::default());
        // The object at +0x14 keeps a reference, the one at +8 loses its last one.
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            e.mem.u32(a[0]).wrapping_sub(1).into_ret()
        });
        e.register(OBJECT_TAIL, |_, _| 0x99u32.into_ret());
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let this = e.mem.alloc(0x200);
        let kept = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        for (object, count) in [(kept, 2u32), (last, 1)] {
            e.mem.set_u32(object, 0x0150_0000);
            e.mem.set_u32(object + 4, count);
        }
        e.mem.set_u32(this + 0x14, kept);
        e.mem.set_u32(this + 8, last);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_2b60, &args![this]).u32(), 0x99);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (DELETE_CRITICAL_SECTION, vec![this + 0x180]),
                (DELETE_CRITICAL_SECTION, vec![this + 0x100]),
                (DELETE_CRITICAL_SECTION, vec![this + 0x80]),
                (INTERLOCKED_DECREMENT, vec![kept + 4]),
                (INTERLOCKED_DECREMENT, vec![last + 4]),
                (0x0150_1000, vec![last]),
                (OBJECT_TAIL, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0109_bb94);
        assert_eq!(e.mem.u32(RENDERER_GLOBAL), 0);
    }
}
