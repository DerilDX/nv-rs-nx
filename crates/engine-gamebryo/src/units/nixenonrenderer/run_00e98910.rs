//! `NiXenonRenderer/run_00e98910` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// What `00e98910` uses: the `NiRefObject` table and global object count (`011f4400`), the
/// interlocked increment import and the table it then gets (`010f16cc`).
const NI_REF_OBJECT_VTABLE_0101DCE4: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const INTERLOCKED_UP: u32 = 0x00fd_f1c0;
const VTABLE_010F16CC: u32 = 0x010f_16cc;

// Translated from 00e98910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e98910`): a constructor, `thiscall` with one word (the
/// renderer). The `NiRefObject` start (its table, count +4 cleared, the global object count
/// `011f4400` counted up), then the table `010f16cc`, the words +8 to +0x7c cleared and the
/// renderer at +0x84. With a renderer, its object at +0x288 is stored at +0x80; without one +0x80
/// keeps whatever it held. A nonzero +0x80 then gets its vtable slot 4 call (add reference).
/// Answers `this`.
pub fn fn_00e98910(e: &mut Engine, this: Ptr, renderer: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.call(INTERLOCKED_UP, &args![OBJECT_COUNT_011F4400]);
    e.mem.set_u32(base, VTABLE_010F16CC);
    for offset in (8..=0x7cu32).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u32(base.wrapping_add(0x84), renderer);
    if renderer != 0 {
        let object = e.mem.u32(renderer.wrapping_add(0x288));
        e.mem.set_u32(base.wrapping_add(0x80), object);
    }
    let object = e.mem.u32(base.wrapping_add(0x80));
    if object != 0 {
        e.vcall(object, 4, &args![]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_8910, fn_00e98910(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod b000y_tests {
    use super::*;

    const ADD_REF: u32 = 0x0e10_0004;

    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, ADD_REF);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    fn added(e: &mut Engine) -> Vec<u32> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == ADD_REF)
            .map(|(_, words)| words[0])
            .collect()
    }

    #[test]
    fn fn_00e98910_constructs_and_holds_the_renderer_object() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_UP, |_, _| Ret::default());
        e.register(ADD_REF, |_, _| Ret::default());
        let held = object(&mut e);
        let renderer = e.mem.alloc(0x290);
        e.mem.set_u32(renderer + 0x288, held);
        let this = e.mem.alloc(0x8c);
        e.mem.write(this, &[0xaa; 0x8c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_8910, &args![this, renderer]).u32(), this);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .contains(&(INTERLOCKED_UP, vec![OBJECT_COUNT_011F4400])));
        assert_eq!(added(&mut e), vec![held]);
        assert_eq!(e.mem.u32(this), VTABLE_010F16CC);
        assert_eq!(e.mem.bytes(this + 4, 0x7c), vec![0; 0x7c]);
        assert_eq!(e.mem.u32(this + 0x80), held);
        assert_eq!(e.mem.u32(this + 0x84), renderer);
        assert_eq!(e.mem.u32(this + 0x88), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_00e98910_without_a_renderer_keeps_the_old_word() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_UP, |_, _| Ret::default());
        e.register(ADD_REF, |_, _| Ret::default());
        let old = object(&mut e);
        let this = e.mem.alloc(0x8c);
        e.mem.set_u32(this + 0x80, old);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8910, &args![this, 0u32]);
        assert_eq!(added(&mut e), vec![old]);
        assert_eq!(e.mem.u32(this + 0x80), old);
        assert_eq!(e.mem.u32(this + 0x84), 0);
        // A renderer without an object: +0x80 cleared, nothing added.
        let renderer = e.mem.alloc(0x290);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8910, &args![this, renderer]);
        assert!(added(&mut e).is_empty());
        assert_eq!(e.mem.u32(this + 0x80), 0);
    }
}
